use std::env;
use std::fs;
use std::io::{self, Write, BufRead};
use std::path::Path;
use std::process::{Command, exit};

fn compile_rust(rust_source: &str, output_path: &str, verbose: bool) -> Result<(), String> {
    let tmp_dir = std::env::temp_dir().join("rython_compile");
    std::fs::create_dir_all(&tmp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;
    let rs_file = tmp_dir.join("main.rs");
    std::fs::write(&rs_file, rust_source).map_err(|e| format!("Failed to write source: {}", e))?;

    let mut cmd = Command::new("rustc");
    cmd.arg("--edition").arg("2021").arg("-o").arg(output_path).arg(rs_file.to_string_lossy().as_ref());

    struct WrapperCfg { name: &'static str, cpp: fn() -> &'static str, pkgs: &'static [&'static str] }
    let wrappers: &[WrapperCfg] = &[
        WrapperCfg { name: "qt6", cpp: || include_str!("wrappers/qt6.cpp"), pkgs: &["Qt6Widgets", "Qt6Core", "Qt6Gui"] },
        WrapperCfg { name: "sqlite3", cpp: || include_str!("wrappers/sqlite3.cpp"), pkgs: &["sqlite3"] },
        WrapperCfg { name: "sdl2", cpp: || include_str!("wrappers/sdl2.cpp"), pkgs: &["sdl2", "SDL2_image", "SDL2_ttf", "SDL2_mixer"] },
        WrapperCfg { name: "curl", cpp: || include_str!("wrappers/curl.cpp"), pkgs: &["libcurl"] },
        WrapperCfg { name: "image", cpp: || include_str!("wrappers/image.cpp"), pkgs: &["libpng", "libjpeg", "libwebp"] },
        WrapperCfg { name: "opengl", cpp: || include_str!("wrappers/opengl.cpp"), pkgs: &["x11", "gl"] },
        WrapperCfg { name: "git", cpp: || include_str!("wrappers/git.cpp"), pkgs: &["libgit2"] },
        WrapperCfg { name: "gtk4", cpp: || include_str!("wrappers/gtk4.cpp"), pkgs: &["gtk4"] },
        WrapperCfg { name: "ffmpeg", cpp: || include_str!("wrappers/ffmpeg.cpp"), pkgs: &["libavformat", "libavcodec", "libavutil", "libswscale"] },
        WrapperCfg { name: "font", cpp: || include_str!("wrappers/font.cpp"), pkgs: &["freetype2", "harfbuzz"] },
        WrapperCfg { name: "vulkan", cpp: || include_str!("wrappers/vulkan.cpp"), pkgs: &["vulkan"] },
        WrapperCfg { name: "torch", cpp: || include_str!("wrappers/torch.cpp"), pkgs: &["libtorch"] },
        WrapperCfg { name: "numpy", cpp: || include_str!("wrappers/numpy.cpp"), pkgs: &[] },
        WrapperCfg { name: "lvgl", cpp: || include_str!("wrappers/lvgl.cpp"), pkgs: &["lvgl"] },
        WrapperCfg { name: "cuda", cpp: || include_str!("wrappers/cuda.cpp"), pkgs: &[] },
    ];

    for wr in wrappers {
        let pattern_use = format!("use wrappers::{}::", wr.name);
        let pattern_crate = format!("crate::wrappers::{}::", wr.name);
        if !rust_source.contains(&pattern_use) && !rust_source.contains(&pattern_crate) {
            continue;
        }
        if wr.name == "cuda" {
            let cuda_marker = tmp_dir.join(".cuda_check");
            let cuda_ok = if let Ok(stamp) = std::fs::read(&cuda_marker) {
                stamp == b"1"
            } else {
                let result = Command::new("g++")
                    .args(["-x", "c++", "-", "-c", "-o", "/dev/null"])
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .and_then(|mut c| {
                        c.stdin.take().map(|mut s| s.write_all(b"#include <cuda_runtime.h>"));
                        c.wait()
                    });
                let ok = result.ok().map(|s| s.success()).unwrap_or(false);
                let _ = std::fs::write(&cuda_marker, if ok { b"1" } else { b"0" });
                ok
            };
            if !cuda_ok {
                if verbose { eprintln!("Warning: CUDA not available, skipping"); }
                continue;
            }
        } else if !wr.pkgs.is_empty() {
            let pkgs_available = std::process::Command::new("pkg-config")
                .args(wr.pkgs).arg("--exists")
                .status().ok().map(|s| s.success()).unwrap_or(false);
            if !pkgs_available {
                if verbose { eprintln!("Warning: packages for {} wrapper not found (pkg-config {} failed), skipping", wr.name, wr.pkgs.join(" ")); }
                continue;
            }
        }
        let cpp_source = (wr.cpp)();
        let tmp_obj = tmp_dir.join(format!("{}.o", wr.name));
        let tmp_obj_str = tmp_obj.to_string_lossy().to_string();
        let cpp_marker = tmp_dir.join(format!(".{}_stamp", wr.name));
        let needs_rebuild = std::fs::read(&cpp_marker).ok()
            .map(|stamp| stamp != cpp_source.as_bytes())
            .unwrap_or(true);
        if needs_rebuild {
            let _ = std::fs::write(&cpp_marker, cpp_source.as_bytes());
            let cflags = std::process::Command::new("pkg-config")
                .args(["--cflags"]).args(wr.pkgs)
                .output().ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
            let mut gpp = Command::new("g++");
            gpp.args(["-std=c++17", "-fPIC", "-c", "-x", "c++", "-", "-o", &tmp_obj_str]);
            for flag in cflags.split_whitespace() { gpp.arg(flag); }
            gpp.stdin(std::process::Stdio::piped());
            gpp.stdout(std::process::Stdio::null());
            gpp.stderr(std::process::Stdio::piped());
            if let Ok(mut child) = gpp.spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(cpp_source.as_bytes()).ok();
                }
                if let Ok(output) = child.wait_with_output() {
                    if !output.status.success() {
                        if verbose {
                            let err = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Warning: C++ {} compilation failed:\n{}", wr.name, err);
                        }
                        let _ = std::fs::remove_file(&cpp_marker);
                        let _ = std::fs::remove_file(&tmp_obj);
                        continue;
                    }
                }
            }
        }
        if !tmp_obj.exists() { continue; }
        cmd.arg("-C").arg(format!("link-arg={}", tmp_obj_str));
        let libs = std::process::Command::new("pkg-config")
            .args(["--libs"]).args(wr.pkgs)
            .output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();
        for flag in libs.split_whitespace() { cmd.arg(&flag); }
        cmd.arg("-lstdc++").arg("-lpthread");
    }

    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let output = cmd.output().map_err(|e| format!("Failed to run rustc: {}", e))?;
    if output.status.success() { Ok(()) }
    else { Err(String::from_utf8_lossy(&output.stderr).to_string()) }
}

fn run_with_python3(filename: &str) {
    let status = Command::new("python3")
        .arg(filename)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => exit(s.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("Failed to run python3: {}", e);
            exit(1);
        }
    }
}

fn safe_parse(source: &str, verbose: bool) -> Option<rython::ast::Program> {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let tokens = rython::lexer::tokenize(source);
    if verbose {
        eprintln!("=== TOKENS ===");
        for t in &tokens {
            eprintln!("{:?} (line {})", t.kind, t.line);
        }
        eprintln!("=== END ===");
    }

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        rython::parser::Parser::new(tokens).parse()
    }));

    std::panic::set_hook(prev);
    result.ok()
}

fn collect_local_imports(program: &rython::ast::Program) -> Vec<String> {
    let mut local = Vec::new();
    for stmt in &program.stmts {
        match stmt {
            rython::ast::Stmt::Import(names) => {
                for (name, _) in names {
                    if !rython::transpiler::STDLIB_MODULES.contains(&name.as_str()) {
                        if !local.contains(name) {
                            local.push(name.clone());
                        }
                    }
                }
            }
            rython::ast::Stmt::ImportFrom { module, .. } => {
                if let Some(mod_name) = module {
                    if !rython::transpiler::STDLIB_MODULES.contains(&mod_name.as_str()) {
                        if !local.contains(mod_name) {
                            local.push(mod_name.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    local
}

fn find_module_source(module_name: &str, base_dir: &Path) -> Option<String> {
    for dir in &[base_dir.to_path_buf(), base_dir.join("rython_packages"), base_dir.join("rython_packages").join(module_name).join("src")] {
        let py_path = dir.join(format!("{}.py", module_name));
        if let Ok(s) = fs::read_to_string(&py_path) {
            return Some(s);
        }
    }
    for dir in &[base_dir.to_path_buf(), base_dir.join("rython_packages")] {
        let init_path = dir.join(module_name).join("__init__.py");
        if let Ok(s) = fs::read_to_string(&init_path) {
            return Some(s);
        }
    }
    None
}

fn transpile_module(module_name: &str, base_dir: &Path, verbose: bool) -> String {
    let parts: Vec<&str> = module_name.split('.').collect();
    let top_module = parts[0];
    let source = match find_module_source(module_name, base_dir) {
        Some(s) => s,
        None => return format!("pub mod {} {{}}\n", top_module),
    };
    let program = match safe_parse(&source, false) {
        Some(p) => p,
        None => return format!("pub mod {} {{}}\n", top_module),
    };

    let mut sub_modules = String::new();
    let sub_imports = collect_local_imports(&program);
    for sub in sub_imports {
        let sub_path = format!("{}.{}", module_name, sub);
        let sub_def = transpile_module(&sub_path, base_dir, verbose);
        sub_modules.push_str(&sub_def);
    }

    let mut tr = rython::transpiler::Transpiler::new();
    let (defs_code, _) = tr.transpile(&program);

    if defs_code.trim().is_empty() && sub_modules.trim().is_empty() {
        return format!("pub mod {} {{}}\n", top_module);
    }

    let mut module_out = format!("pub mod {} {{\n", top_module);
    module_out.push_str("    use super::Value;\n");
    for line in defs_code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
                    let adjusted = if trimmed == "use wrappers::qt6::construct;" || trimmed == "use wrappers::opengl::construct;" || trimmed == "use wrappers::gtk4::construct;" {
                            let module = if trimmed.contains("qt6") { "qt6" } else if trimmed.contains("opengl") { "opengl" } else { "gtk4" };
                            format!("use super::wrappers::{}::construct;", module)
                        } else if trimmed == "use wrappers::qt6::method;" || trimmed == "use wrappers::opengl::method;" || trimmed == "use wrappers::gtk4::method;" {
                            let module = if trimmed.contains("qt6") { "qt6" } else if trimmed.contains("opengl") { "opengl" } else { "gtk4" };
                            format!("use super::wrappers::{}::method;", module)
                        } else if trimmed.starts_with("use wrappers::") {
            continue;
        } else {
            line.to_string()
        };
        module_out.push_str(&format!("    {}\n", adjusted));
    }
    for line in sub_modules.lines() {
        if !line.trim().is_empty() {
            module_out.push_str(&format!("    {}\n", line));
        }
    }
    module_out.push_str("}\n");
    module_out
}

fn build_full_source(main_defs: &str, main_stmts: &str, main_filename: &str,
                     module_defs: &[String], _verbose: bool, python_source: &str) -> String {
    let runtime_source = include_str!("runtime.rs");
    let stdlib_source = include_str!("stdlib.rs");
    let mut wrappers_source = String::from(include_str!("wrappers.rs"));

    // Replace include_str! in wrappers source with the actual C++ code as a string literal
    let qt6_cpp = include_str!("wrappers/qt6.cpp");
    let escaped = qt6_cpp.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
    let marker = "include_str!(\"wrappers/qt6.cpp\")";
    wrappers_source = wrappers_source.replace(marker, &format!("\"{}\"", escaped));

    let opengl_cpp = include_str!("wrappers/opengl.cpp");
    let escaped = opengl_cpp.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
    let marker = "include_str!(\"wrappers/opengl.cpp\")";
    wrappers_source = wrappers_source.replace(marker, &format!("\"{}\"", escaped));

    let gtk4_cpp = include_str!("wrappers/gtk4.cpp");
    let escaped = gtk4_cpp.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
    let marker = "include_str!(\"wrappers/gtk4.cpp\")";
    wrappers_source = wrappers_source.replace(marker, &format!("\"{}\"", escaped));

    let mut out = String::from("#![allow(dead_code)]\n\n");
    out.push_str(stdlib_source);
    out.push_str("\n\n");
    out.push_str(runtime_source);
    out.push_str("\n\n");
    out.push_str(&wrappers_source);
    out.push_str("\n\n");

    for mod_def in module_defs {
        out.push_str(mod_def);
        out.push_str("\n");
    }

    out.push_str(&format!("// Source: {}\n", main_filename));
    out.push_str(&rython::decompiler::embed_source(python_source));
    out.push_str("\n");
    out.push_str(main_defs);
    out.push_str("fn main() -> () {\n");
    out.push_str(main_stmts);
    out.push_str("}\n");
    out
}

fn print_help() {
    eprintln!("rython — Python→Rust transpiler");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  rython <file.py>              Transpile & run");
    eprintln!("  rython <file.py> --output <f> Transpile to Rust file");
    eprintln!("  rython <file.py> --verbose    Show tokens & compile logs");
    eprintln!("  rython fmt <file.py>          Format Python file");
    eprintln!("  rython repl                   Interactive REPL");
    eprintln!("  rython test <file.py>         Run tests");
    eprintln!("  rython decompile <binary>      Decompile binary back to Python (experimental)");
    eprintln!("  rython lsp                    LSP server (stdin/stdout)");
    eprintln!("  rython help                   Show this help");
    eprintln!();
    eprintln!("Package management: rip install <package>");
}

fn cmd_transpile(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: rython <file.py> [--output <file>] [--verbose]");
        exit(1);
    }
    let filename = &args[0];
    let verbose = args.iter().any(|a| a == "--verbose");
    let output_idx = args.iter().position(|a| a == "--output");
    let output_file = output_idx.map(|i| args[i + 1].clone());

    let source = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading '{}': {}", filename, e); exit(1); }
    };

    let program = match safe_parse(&source, verbose) {
        Some(p) => p,
        None => { run_with_python3(filename); return; }
    };

    let base_dir = Path::new(filename).parent().unwrap_or(Path::new(".")).to_path_buf();
    let local_modules = collect_local_imports(&program);
    let mut module_defs = Vec::new();
    for mod_name in &local_modules {
        module_defs.push(transpile_module(mod_name, &base_dir, verbose));
    }
    let mut tr = rython::transpiler::Transpiler::new();
    let (defs_code, stmts_code) = tr.transpile(&program);

    let full_source = build_full_source(&defs_code, &stmts_code, filename, &module_defs, verbose, &source);
    let cache_dir = Path::new(".rython_cache");
    let _ = fs::create_dir_all(cache_dir);
    let output_path = output_file.clone().unwrap_or_else(|| {
        let base = Path::new(filename).file_stem().unwrap_or_default().to_string_lossy();
        cache_dir.join(format!("{}.rs", base)).to_string_lossy().to_string()
    });
    fs::write(&output_path, &full_source).expect("Failed to write output");

    if verbose { eprintln!("Generated: {}", output_path); }

    let binary_path = output_file.map(|o| Path::new(&o).with_extension("").to_string_lossy().to_string())
        .unwrap_or_else(|| {
            let base = Path::new(filename).file_stem().unwrap_or_default().to_string_lossy();
            cache_dir.join(base.to_string()).to_string_lossy().to_string()
        });

    match compile_rust(&full_source, &binary_path, verbose) {
        Ok(_) => if verbose { eprintln!("Compiled: {}", binary_path); }
        Err(e) => { eprintln!("Compile error:\n{}", e); exit(1); }
    }

    if !args.iter().any(|a| a == "--output") || args.iter().any(|a| a == "--run") {
        let run_path = if binary_path.contains('/') { binary_path.clone() }
                       else { format!("./{}", binary_path) };
        let mut child = Command::new(&run_path).stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit()).spawn().unwrap_or_else(|e| {
                eprintln!("Failed to run binary: {}", e); exit(1);
            });
        let _ = child.wait();
    }
}

// ── rython fmt ─────────────────────────────────────────────
fn cmd_fmt(args: &[String]) {
    let filename = args.iter().find(|a| !a.starts_with('-'));
    let filename = match filename {
        Some(f) => f,
        None => { eprintln!("Usage: rython fmt [--check|--diff] <file.py>"); exit(1); }
    };
    let source = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading '{}': {}", filename, e); exit(1); }
    };
    let formatted = rython::formatter::format_python(&source);
    if args.iter().any(|a| a == "--check") {
        if source == formatted {
            eprintln!("{}: OK", filename);
        } else {
            eprintln!("{}: would reformat", filename);
            exit(1);
        }
        return;
    }
    if args.iter().any(|a| a == "--diff") {
        for (i, (a, b)) in source.lines().zip(formatted.lines()).enumerate() {
            if a != b {
                eprintln!("-{}:{}\t{}", filename, i + 1, a);
                eprintln!("+{}:{}\t{}", filename, i + 1, b);
            }
        }
        return;
    }
    fs::write(filename, &formatted).expect("Failed to write formatted file");
    eprintln!("Formatted: {}", filename);
}

// ── rython repl ────────────────────────────────────────────
fn cmd_repl(_args: &[String]) {
    eprintln!("rython REPL (type exit() to quit)");
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    loop {
        let prompt = if lines.is_empty() { ">>> " } else { "... " };
        print!("{}", prompt);
        io::stdout().flush().ok();
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).ok().is_none_or(|n| n == 0) {
            break;
        }
        let trimmed = input.trim().to_string();
        if trimmed == "exit()" { break; }
        if trimmed.is_empty() && lines.is_empty() { continue; }
        lines.push(trimmed);

        let code = lines.join("\n");
        let tokens = rython::lexer::tokenize(&code);
        let mut parser = rython::parser::Parser::new(tokens);
        let parsed = parser.parse();
        if parsed.stmts.is_empty() {
            continue;
        }
        let mut tr = rython::transpiler::Transpiler::new();
        let (defs, stmts) = tr.transpile(&parsed);
        let full = format!("#![allow(dead_code)]\n{}\n{}\n{}\nfn main() {{\n{}{}\n}}\n",
            include_str!("stdlib.rs"), include_str!("runtime.rs"), include_str!("wrappers.rs"), defs, stmts);
        let tmp = std::env::temp_dir().join("repl_eval.rs");
        let out = std::env::temp_dir().join("repl_eval_bin");
        let _ = fs::write(&tmp, &full);
        let compile = Command::new("rustc").args(["--edition", "2021", "-o"])
            .arg(&out).arg(&tmp).output();
        match compile {
            Ok(c) if c.status.success() => {
                let result = Command::new(&out).output().ok()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                    .unwrap_or_default();
                print!("{}", result);
            }
            Ok(c) => eprintln!("{}", String::from_utf8_lossy(&c.stderr)),
            Err(e) => eprintln!("{}", e),
        }
        let _ = fs::remove_file(&out);
        let _ = fs::remove_file(&tmp);
        lines.clear();
    }
}

// ── rython test ────────────────────────────────────────────
fn cmd_test(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: rython test <file.py>");
        exit(1);
    }
    let filename = &args[0];
    let source = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading '{}': {}", filename, e); exit(1); }
    };

    let program = match safe_parse(&source, false) {
        Some(p) => p,
        None => { eprintln!("Parse failed"); exit(1); }
    };

    // Find test functions (def test_*)
    let test_funcs: Vec<&str> = program.stmts.iter().filter_map(|stmt| {
        if let rython::ast::Stmt::FunctionDef { name, .. } = stmt {
            if name.starts_with("test_") { Some(name.as_str()) } else { None }
        } else { None }
    }).collect();

    if test_funcs.is_empty() {
        eprintln!("No test functions found (def test_*) in {}", filename);
        exit(1);
    }

    let base_dir = Path::new(filename).parent().unwrap_or(Path::new(".")).to_path_buf();
    let local_modules = collect_local_imports(&program);
    let mut module_defs = Vec::new();
    for mod_name in &local_modules {
        module_defs.push(transpile_module(mod_name, &base_dir, false));
    }

    let mut tr = rython::transpiler::Transpiler::new();
    let (defs_code, _) = tr.transpile(&program);

    let mut passed = 0u32;
    let mut failed = 0u32;

    for func_name in &test_funcs {
        let test_main = format!(
            "#![allow(dead_code)]
{}
{}
{}
{}
fn main() {{
    {}();
    println!(\"PASS\");
}}
",
            include_str!("stdlib.rs"),
            include_str!("runtime.rs"),
            include_str!("wrappers.rs"),
            defs_code,
            func_name
        );

        let out = std::env::temp_dir().join(format!("rython_test_{}", func_name));
        match compile_rust(&test_main, out.to_str().unwrap(), false) {
            Ok(_) => {
                let result = Command::new(&out).output().ok()
                    .map(|o| (o.status.success(), String::from_utf8_lossy(&o.stdout).to_string(), String::from_utf8_lossy(&o.stderr).to_string()))
                    .unwrap_or((false, String::new(), String::new()));
                if result.0 {
                    passed += 1;
                    println!("✓ {}: PASS", func_name);
                } else {
                    failed += 1;
                    println!("✗ {}: FAIL", func_name);
                    if !result.2.is_empty() { eprintln!("{}", result.2); }
                    if !result.1.is_empty() { eprintln!("{}", result.1); }
                }
                let _ = fs::remove_file(&out);
            }
            Err(e) => {
                failed += 1;
                println!("✗ {}: COMPILE ERROR\n{}", func_name, e);
            }
        }
    }

    let total = passed + failed;
    println!("\n{}/{} tests passed", passed, total);
    if failed > 0 { exit(1); }
}

fn cmd_decompile(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: rython decompile <binary> [--output <file.py>]");
        exit(1);
    }
    let binary = &args[0];
    let output_file = args.iter().position(|a| a == "--output").map(|i| args[i + 1].clone());

    match rython::decompiler::decompile(binary) {
        Ok(py_source) => {
            if let Some(path) = output_file {
                fs::write(&path, &py_source).expect("Failed to write output");
                eprintln!("Decompiled to: {}", path);
            } else {
                println!("{}", py_source);
            }
        }
        Err(e) => {
            eprintln!("Decompile error: {}", e);
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(1);
    }

    match args[1].as_str() {
        "help" | "--help" | "-h" => print_help(),
        "fmt" => cmd_fmt(&args[2..]),
        "repl" => cmd_repl(&args[2..]),
        "test" => cmd_test(&args[2..]),
        "decompile" => cmd_decompile(&args[2..]),
        "lsp" => rython::lsp::run_lsp(),
        _ => cmd_transpile(&args[1..]),
    }
}
