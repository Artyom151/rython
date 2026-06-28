use std::env;
use std::fs;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const PACKAGES_DIR: &str = "rython_packages";

const RESET: &str = "\x1b[0m";

fn fg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}
fn bg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

fn styled(kind: &str, msg: &str) -> String {
    match kind {
        "WARN" => format!("{}{} WARN {}{}{}{}", bg(139, 128, 0), fg(255, 255, 255), RESET, fg(200, 200, 150), RESET, msg),
        "ERR"  => format!("{}{} ERR {}{}{}{}", bg(139, 0, 0), fg(255, 255, 255), RESET, fg(255, 150, 150), RESET, msg),
        "OK"   => format!("{}✓ {}{}", fg(0, 200, 0), fg(200, 255, 200), msg),
        "INFO" => format!("{}→ {}{}", fg(100, 200, 255), RESET, msg),
        "DONE" => format!("{}✔ {}{}", fg(0, 200, 0), RESET, msg),
        _      => msg.to_string(),
    }
}

fn col(msg: &str, r: u8, g: u8, b: u8) -> String {
    format!("{}{}{}", fg(r, g, b), msg, RESET)
}

const DOT_CYCLE: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

struct Spinner {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    fn new(msg: &str) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        let m = msg.to_string();
        let handle = thread::spawn(move || {
            let mut i = 0;
            while r.load(Ordering::Relaxed) {
                let s = DOT_CYCLE[i % DOT_CYCLE.len()];
                let c = if i % 2 == 0 { fg(0, 255, 128) } else { fg(128, 255, 255) };
                print!("\r{} {} {}... {}", c, s, RESET, m);
                stdout().flush().ok();
                thread::sleep(Duration::from_millis(100));
                i += 1;
            }
            print!("\r\x1b[K");
            stdout().flush().ok();
        });
        Spinner { running, handle: Some(handle) }
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            h.join().ok();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        exit(1);
    }

    let verbose = args.contains(&"--verbose".to_string());
    let packages_dir = PathBuf::from(PACKAGES_DIR);

    match args[1].as_str() {
        "help" | "--help" | "-h" => print_help(),

        "install" => {
            if args.len() > 2 && args[2] == "-r" {
                if args.len() < 4 {
                    eprintln!("{}", styled("ERR", "Usage: rip install -r requirements.txt"));
                    exit(1);
                }
                install_requirements(&args[3], &packages_dir, verbose);
            } else if args.len() > 2 {
                for pkg in &args[2..] {
                    if pkg.starts_with("--") { continue; }
                    install_package(pkg, &packages_dir, verbose);
                }
            } else {
                eprintln!("{}", styled("ERR", "Usage: rip install <package>"));
                exit(1);
            }
        }

        "uninstall" | "remove" => {
            if args.len() < 3 {
                eprintln!("{}", styled("ERR", "Usage: rip uninstall <package>"));
                exit(1);
            }
            for pkg in &args[2..] {
                if pkg.starts_with("--") { continue; }
                uninstall_package(pkg, &packages_dir);
            }
        }

        "new" => {
            if args.len() < 3 {
                eprintln!("{}", styled("ERR", "Usage: rip new <project_name>"));
                exit(1);
            }
            create_project(&args[2]);
        }

        "list" | "ls" => {
            list_packages(&packages_dir);
        }

        _ => {
            eprintln!("{} {} {}",
                styled("ERR", "Unknown command:"), args[1],
                styled("INFO", "run 'rip help' for usage"));
            exit(1);
        }
    }
}

fn print_help() {
    let title = format!("{} {} {}", col("⣿", 0, 255, 128), col("rip", 255, 200, 100), col("— rython package manager", 150, 150, 150));
    eprintln!("{}", title);
    eprintln!();
    eprintln!("{}", col("USAGE:", 200, 200, 200));
    eprintln!("  rip <command> [options]");
    eprintln!();
    eprintln!("{}", col("COMMANDS:", 200, 200, 200));
    eprintln!("  {} {}", col("install", 100, 255, 150), col("<package>   Download + transpile a PyPI package", 180, 180, 180));
    eprintln!("  {} {}", col("install -r", 100, 255, 150), col("requirements.txt   Batch install", 180, 180, 180));
    eprintln!("  {} {}", col("uninstall", 255, 150, 150), col("<package>   Remove an installed package", 180, 180, 180));
    eprintln!("  {} {}", col("new", 150, 200, 255), col("<project>   Create a new rython project", 180, 180, 180));
    eprintln!("  {} {}", col("list", 200, 200, 100), col("             Show installed packages", 180, 180, 180));
    eprintln!("  {} {}", col("help", 200, 200, 255), col("             Show this help", 180, 180, 180));
    eprintln!();
    eprintln!("{}", col("EXAMPLES:", 200, 200, 200));
    eprintln!("  rip install pytz");
    eprintln!("  rip install -r requirements.txt");
    eprintln!("  rip uninstall pytz");
    eprintln!("  rip new my_project");
    eprintln!("  rip list");
    eprintln!();
    eprintln!("{}", col("With --verbose for detailed logs.", 120, 120, 120));
}

fn install_requirements(req_file: &str, packages_dir: &Path, verbose: bool) {
    let content = match fs::read_to_string(req_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} '{}': {}", styled("ERR", "Error reading requirements file"), req_file, e);
            exit(1);
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("-i") || trimmed.starts_with("--") {
            continue;
        }
        let pkg_name = trimmed
            .split(&['>', '<', '=', '!', '~', ' ', '\t'][..])
            .next()
            .unwrap_or(trimmed)
            .trim();
        if !pkg_name.is_empty() {
            eprintln!("{}", styled("INFO", format!("Installing dependency: {}", pkg_name).as_str()));
            install_package(pkg_name, packages_dir, verbose);
        }
    }
}

fn install_package(name: &str, packages_dir: &Path, verbose: bool) {
    eprintln!("{}", styled("INFO", format!("Fetching {} from PyPI...", name).as_str()));

    let mut spinner = Spinner::new(&format!("{} {}", col("resolving", 180, 180, 180), name));
    let pypi_url = format!("https://pypi.org/pypi/{}/json", name);
    let metadata = fetch_url(&pypi_url);
    spinner.stop();

    let metadata = match metadata {
        Some(data) => data,
        None => {
            eprintln!("{} {}", styled("ERR", "Failed to fetch package"), name);
            return;
        }
    };

    let sdist_url = extract_sdist_url(&metadata);
    let sdist_url = match sdist_url {
        Some(url) => url,
        None => {
            eprintln!("{} {}", styled("ERR", "No source distribution for"), name);
            return;
        }
    };

    eprintln!("{} {}", col("downloading", 180, 180, 180), name);
    let mut spinner = Spinner::new(&format!("{} {}", col("fetching", 150, 200, 255), &sdist_url[..sdist_url.len().min(80)]));

    let archive_data = match fetch_bytes(&sdist_url) {
        Some(d) => d,
        None => {
            spinner.stop();
            eprintln!("{} {}", styled("ERR", "Failed to download"), name);
            return;
        }
    };
    spinner.stop();
    eprintln!("{} ({})", styled("OK", format!("Downloaded {}", name).as_str()), format!("{} bytes", archive_data.len()));

    let temp_dir = packages_dir.join(".tmp").join(name);
    fs::create_dir_all(&temp_dir).ok();
    eprintln!("{}", styled("INFO", format!("Extracting {}...", name).as_str()));
    extract_archive(&archive_data, &temp_dir, &sdist_url);

    let pkg_root = find_package_root(&temp_dir, name);

    let out_dir = packages_dir.join(name);
    fs::create_dir_all(&out_dir).ok();
    eprintln!("{}", styled("INFO", format!("Transforming {} to rython...", name).as_str()));
    let mut spinner = Spinner::new(&format!("{}", col("transpiling modules", 150, 200, 150)));
    transpile_package(&pkg_root, &out_dir, verbose);
    spinner.stop();

    if !verbose {
        fs::remove_dir_all(&temp_dir).ok();
    }

    generate_cargo_toml(name, &out_dir);

    eprintln!("{} {} → {:?}", styled("DONE", name), col("installed", 150, 255, 150), out_dir);
}

fn uninstall_package(name: &str, packages_dir: &Path) {
    let pkg_path = packages_dir.join(name);
    if !pkg_path.exists() {
        eprintln!("{} {} {}", styled("WARN", "Package not installed:"), name, styled("INFO", "run 'rip list' to see installed packages"));
        return;
    }
    match fs::remove_dir_all(&pkg_path) {
        Ok(_) => eprintln!("{} {}", styled("DONE", name), col("removed", 150, 255, 150)),
        Err(e) => eprintln!("{} {}: {}", styled("ERR", "Failed to remove"), name, e),
    }
}

fn create_project(name: &str) {
    let dir = Path::new(name);
    if dir.exists() {
        eprintln!("{} Directory '{}' already exists", styled("ERR", ""), name);
        exit(1);
    }

    fs::create_dir_all(dir.join("rython_packages")).ok();
    fs::create_dir_all(dir.join("examples")).ok();

    
    let main_py = r#"# rython project: __PROJECT__
# Example program

def greet(name):
    print("Hello, " + name + "!")

def fib(n):
    a, b = 0, 1
    while a < n:
        print(a)
        a, b = b, a + b

greet("rython")
fib(100)
"#.replace("__PROJECT__", name);
    fs::write(dir.join("main.py"), main_py).ok();

    
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#, name
    );
    fs::write(dir.join("Cargo.toml"), &cargo_toml).ok();

    
    fs::write(dir.join(".gitignore"), "/target\nrython_packages/*\n!rython_packages/mod.rs\n").ok();

    
    fs::write(dir.join("README.md"), format!("# {}\n\nA rython project.\n", name)).ok();

    eprintln!("{}", styled("DONE", format!("Created project '{}'", name).as_str()));
    eprintln!("  cd {} && rip install    # install dependencies", name);
    eprintln!("  rython {}/main.py       # run the project", name);
}

fn list_packages(packages_dir: &Path) {
    if !packages_dir.exists() {
        eprintln!("{} No packages installed", styled("INFO", ""));
        return;
    }

    let entries: Vec<_> = fs::read_dir(packages_dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false) && !e.file_name().to_string_lossy().starts_with('.'))
        .collect();

    if entries.is_empty() {
        eprintln!("{} No packages installed", styled("INFO", ""));
        return;
    }

    eprintln!("{}", col("Installed packages:", 200, 200, 200));
    for entry in &entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let has_cargo = entry.path().join("Cargo.toml").exists();
        let status = if has_cargo { col("✓", 0, 200, 0) } else { col("?", 200, 200, 100) };
        eprintln!("  {} {}", status, name);
    }
}



fn fetch_url(url: &str) -> Option<String> {
    Command::new("curl").args(["-sSL", url]).output().ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
}

fn fetch_bytes(url: &str) -> Option<Vec<u8>> {
    Command::new("curl").args(["-sSL", url]).output().ok()
        .filter(|o| o.status.success())
        .map(|o| o.stdout)
}

fn extract_sdist_url(metadata: &str) -> Option<String> {
    let script = format!(
        "import json,sys; data=json.loads(sys.stdin.read());\n\
         for u in data['urls']:\n\
         \x20 if u['packagetype']=='sdist': print(u['url']); break"
    );
    let mut child = Command::new("python3")
        .arg("-c").arg(&script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn().ok()?;
    child.stdin.take()?.write_all(metadata.as_bytes()).ok()?;
    let output = child.wait_with_output().ok()?;
    if output.status.success() {
        let url = String::from_utf8(output.stdout).ok()?.trim().to_string();
        if !url.is_empty() { Some(url) } else { None }
    } else { None }
}

fn extract_archive(data: &[u8], dest: &Path, url: &str) {
    let temp_archive = dest.parent().unwrap().join("archive");
    fs::write(&temp_archive, data).ok();

    let ok = if url.ends_with(".tar.gz") || url.ends_with(".tgz") {
        Command::new("tar")
            .args(["-xzf", &temp_archive.to_string_lossy(), "-C", &dest.to_string_lossy()])
            .status().ok()
            .map_or(false, |s| s.success())
    } else if url.ends_with(".zip") {
        Command::new("unzip")
            .args(["-o", &temp_archive.to_string_lossy(), "-d", &dest.to_string_lossy()])
            .status().ok()
            .map_or(false, |s| s.success())
    } else {
        eprintln!("{}: {}", styled("WARN", "Unknown archive format"), url);
        false
    };

    if !ok { eprintln!("{}", styled("ERR", "Failed to extract archive")); }
    fs::remove_file(&temp_archive).ok();
}

fn find_package_root(temp_dir: &Path, package_name: &str) -> PathBuf {
    let entries: Vec<_> = fs::read_dir(temp_dir).ok().into_iter().flatten()
        .filter_map(|e| e.ok()).collect();
    if entries.len() == 1 && entries[0].file_type().map(|t| t.is_dir()).unwrap_or(false) {
        entries[0].path()
    } else {
        let pkg_dir = temp_dir.join(package_name);
        if pkg_dir.exists() && pkg_dir.is_dir() { pkg_dir } else { temp_dir.to_path_buf() }
    }
}

fn transpile_package(src_dir: &Path, out_dir: &Path, verbose: bool) {
    let mut mod_file = String::new();
    walk_py_files(src_dir, src_dir, out_dir, &mut mod_file, verbose);
    fs::write(out_dir.join("mod.rs"), &mod_file).ok();
}

fn walk_py_files(root: &Path, src_dir: &Path, out_dir: &Path, mod_file: &mut String, verbose: bool) {
    let entries = match fs::read_dir(src_dir) { Ok(e) => e, Err(_) => return };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false) {
            let mut sub_mod = String::new();
            walk_py_files(root, &path, out_dir, &mut sub_mod, verbose);
            if !sub_mod.trim().is_empty() {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                let sub_dir = out_dir.join(&dir_name);
                fs::create_dir_all(&sub_dir).ok();
                fs::write(sub_dir.join("mod.rs"), &sub_mod).ok();
                mod_file.push_str(&format!("pub mod {};\n", dir_name));
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if file_stem == "__init__" {
                if let Ok(source) = fs::read_to_string(&path) {
                    transpile_source_to_mod(source, mod_file, verbose);
                }
            } else if !file_stem.starts_with('_') || file_stem == "__main__" {
                if let Ok(source) = fs::read_to_string(&path) {
                    let rust = transpile_py_to_rust(&source, verbose);
                    if !rust.trim().is_empty() {
                        fs::write(out_dir.join(format!("{}.rs", file_stem)), &rust).ok();
                        mod_file.push_str(&format!("pub mod {};\n", file_stem));
                    }
                }
            }
        }
    }
}

fn safe_tokenize(source: &str) -> Vec<rython::lexer::Token> {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| rython::lexer::tokenize(source)));
    std::panic::set_hook(prev);
    r.unwrap_or_default()
}

fn safe_parse(tokens: Vec<rython::lexer::Token>) -> Option<rython::ast::Program> {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        rython::parser::Parser::new(tokens).parse()
    }));
    std::panic::set_hook(prev);
    r.ok()
}

fn safe_transpile(source: &str) -> String {
    let tokens = safe_tokenize(source);
    if tokens.is_empty() { return String::new(); }
    let program = match safe_parse(tokens) { Some(p) => p, None => return String::new() };
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut tr = rython::transpiler::Transpiler::new();
        tr.transpile(&program).0
    }));
    std::panic::set_hook(prev);
    r.unwrap_or_default()
}

fn transpile_source_to_mod(source: String, output: &mut String, _verbose: bool) {
    output.push_str(&safe_transpile(&source));
}

fn transpile_py_to_rust(source: &str, _verbose: bool) -> String {
    let code = safe_transpile(source);
    if code.trim().is_empty() { return String::new(); }
    let mut out = String::from("use super::Value;\n\n");
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "#![allow(dead_code)]" || trimmed.starts_with("// Source:") { continue; }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn generate_cargo_toml(name: &str, out_dir: &Path) {
    let cargo_path = out_dir.join("Cargo.toml");
    if cargo_path.exists() { return; }
    fs::write(&cargo_path, format!(
        "[package]\nname = \"{0}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n", name
    )).ok();
}
