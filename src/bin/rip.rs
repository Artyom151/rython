use std::env;
use std::fs;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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
                let pkgs: Vec<String> = args[2..].iter()
                    .filter(|p| !p.starts_with("--"))
                    .cloned()
                    .collect();
                install_parallel(&pkgs, &packages_dir, verbose);
            } else {
                // No args: read deps from rython.json
                let deps = read_manifest_deps();
                if deps.is_empty() {
                    eprintln!("{}", styled("ERR", "Usage: rip install <package> (or add deps to rython.json)"));
                    exit(1);
                }
                eprintln!("{}", styled("INFO", format!("Installing {} dependencies from rython.json", deps.len()).as_str()));
                install_parallel(&deps, &packages_dir, verbose);
            }
            // Compile to .pyc cache after installation
            compile_to_pycache(&packages_dir);
        }

        "add" => {
            if args.len() < 3 {
                eprintln!("{}", styled("ERR", "Usage: rip add <package>"));
                exit(1);
            }
            let pkgs: Vec<String> = args[2..].iter()
                .filter(|p| !p.starts_with("--"))
                .cloned()
                .collect();
            install_parallel(&pkgs, &packages_dir, verbose);
            for pkg in &pkgs {
                add_to_manifest(pkg);
            }
            compile_to_pycache(&packages_dir);
        }

        "uninstall" | "remove" => {
            if args.len() < 3 {
                eprintln!("{}", styled("ERR", "Usage: rip uninstall <package>"));
                exit(1);
            }
            for pkg in &args[2..] {
                if pkg.starts_with("--") { continue; }
                uninstall_package(pkg, &packages_dir);
                remove_from_manifest(pkg);
            }
        }

        "build" => {
            build_project(&args[2..], verbose);
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
    eprintln!("  {} {}", col("install", 100, 255, 150), col("             (no args) install deps from rython.json", 180, 180, 180));
    eprintln!("  {} {}", col("install -r", 100, 255, 150), col("requirements.txt   Batch install", 180, 180, 180));
    eprintln!("  {} {}", col("add", 100, 255, 150), col("<package>       Install + save to rython.json", 180, 180, 180));
    eprintln!("  {} {}", col("uninstall", 255, 150, 150), col("<package>   Remove an installed package", 180, 180, 180));
    eprintln!("  {} {}", col("build", 255, 200, 100), col("<file.py>       Compile project to a standalone binary", 180, 180, 180));
    eprintln!("  {} {}", col("new", 150, 200, 255), col("<project>   Create a new rython project", 180, 180, 180));
    eprintln!("  {} {}", col("list", 200, 200, 100), col("             Show installed packages", 180, 180, 180));
    eprintln!("  {} {}", col("help", 200, 200, 255), col("             Show this help", 180, 180, 180));
    eprintln!();
    eprintln!("{}", col("EXAMPLES:", 200, 200, 200));
    eprintln!("  rip install pytz");
    eprintln!("  rip install              # from rython.json");
    eprintln!("  rip add aiogram");
    eprintln!("  rip install -r requirements.txt");
    eprintln!("  rip uninstall pytz");
    eprintln!("  rip build main.py");
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

fn parse_package_name(raw: &str) -> (String, Vec<String>) {
    if let Some(bracket_start) = raw.find('[') {
        if let Some(bracket_end) = raw.rfind(']') {
            let base = raw[..bracket_start].to_string();
            let extras_str = &raw[bracket_start+1..bracket_end];
            let extras: Vec<String> = extras_str.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            return (base, extras);
        }
    }
    (raw.to_string(), vec![])
}

fn install_extra_deps(base_name: &str, extras: &[String], packages_dir: &Path, verbose: bool) {
    if extras.is_empty() { return; }
    let pypi_url = format!("https://pypi.org/pypi/{}/json", base_name);
    let metadata = match fetch_url(&pypi_url) {
        Some(data) => data,
        None => { return; }
    };
    let extras_joined = extras.join(",");
    let script = format!(
        "import json,sys; data=json.loads(sys.stdin.read());\n\
         rd = data.get('info', {{}}).get('requires_dist', []);\n\
         extras = '{}'.split(',');\n\
         seen = set();\n\
         for dep in (rd or []):\n\
         \x20 if ';' not in dep: continue\n\
         \x20 pkg_part, cond = dep.split(';', 1)\n\
         \x20 for extra in extras:\n\
         \x20\x20 if (\"'\" + extra + \"'\" in cond) or ('\"' + extra + '\"' in cond):\n\
         \x20\x20\x20 name = pkg_part.strip()\n\
         \x20\x20\x20 for sep in ['[', '>', '<', '=', '!', '~']:\n\
         \x20\x20\x20\x20 name = name.split(sep)[0]\n\
         \x20\x20\x20 name = name.strip()\n\
         \x20\x20\x20 if name and name not in seen:\n\
         \x20\x20\x20\x20 seen.add(name); print(name)",
        extras_joined
    );
    let mut child = match Command::new("python3")
        .arg("-c").arg(&script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn() { Ok(c) => c, Err(_) => return };
    child.stdin.take().unwrap().write_all(metadata.as_bytes()).ok();
    let output = match child.wait_with_output() { Ok(o) => o, Err(_) => return };
    if !output.status.success() { return; }
    let deps: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    for dep in &deps {
        eprintln!("{}", styled("INFO", format!("Installing extra dependency: {}", dep).as_str()));
        install_package_inner(dep, packages_dir, verbose);
    }
}

fn install_package(name: &str, packages_dir: &Path, verbose: bool) {
    let visited = Arc::new(Mutex::new(std::collections::HashSet::new()));
    install_package_shared(name, packages_dir, verbose, &visited);
}

fn install_package_shared(name: &str, packages_dir: &Path, verbose: bool, visited: &Arc<Mutex<std::collections::HashSet<String>>>) {
    let (base_name, extras) = parse_package_name(name);
    resolve_package_tree(&base_name, packages_dir, verbose, visited);
    if !extras.is_empty() {
        // Extra deps still need separate fetch since they need extra-filtered metadata
        install_extra_deps(&base_name, &extras, packages_dir, verbose);
    }
}

/// Install multiple top-level packages in parallel, sharing a single visited set
/// so the dependency graph is never resolved twice.
fn install_parallel(pkgs: &[String], packages_dir: &Path, verbose: bool) {
    if pkgs.is_empty() { return; }
    if pkgs.len() == 1 {
        install_package(&pkgs[0], packages_dir, verbose);
        return;
    }

    let visited = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let max_workers = 4.min(pkgs.len());
    let queue = Arc::new(Mutex::new(pkgs.to_vec()));
    let packages_dir = packages_dir.to_path_buf();

    let mut handles = Vec::new();
    for _ in 0..max_workers {
        let queue = queue.clone();
        let visited = visited.clone();
        let packages_dir = packages_dir.clone();
        handles.push(thread::spawn(move || {
            loop {
                let pkg = {
                    let mut q = queue.lock().unwrap();
                    if q.is_empty() { break; }
                    q.remove(0)
                };
                install_package_shared(&pkg, &packages_dir, verbose, &visited);
            }
        }));
    }
    for h in handles {
        h.join().ok();
    }
}

fn resolve_package_tree(name: &str, packages_dir: &Path, verbose: bool, visited: &Arc<Mutex<std::collections::HashSet<String>>>) {
    {
        let mut v = visited.lock().unwrap();
        if !v.insert(name.to_string()) {
            return;
        }
    }
    if packages_dir.join(name).join("Cargo.toml").exists() {
        // Already installed — still resolve its missing transitive deps
        let pypi_url = format!("https://pypi.org/pypi/{}/json", name);
        if let Some(metadata) = fetch_url(&pypi_url) {
            install_regular_deps(&metadata, packages_dir, verbose, visited);
        }
        return;
    }
    // Install fresh
    install_package_inner(name, packages_dir, verbose);
    // Resolve transitive deps for the freshly installed package
    let pypi_url = format!("https://pypi.org/pypi/{}/json", name);
    if let Some(metadata) = fetch_url(&pypi_url) {
        install_regular_deps(&metadata, packages_dir, verbose, visited);
    }
}

fn install_regular_deps(metadata: &str, packages_dir: &Path, verbose: bool, visited: &Arc<Mutex<std::collections::HashSet<String>>>) {
    let script = "import json,sys; data=json.loads(sys.stdin.read());\n\
         rd = data.get('info', {}).get('requires_dist', []);\n\
         seen = set();\n\
         for dep in (rd or []):\n\
         \x20 if ';' in dep: continue\n\
         \x20 name = dep.strip()\n\
         \x20 for sep in ['[', '>', '<', '=', '!', '~']:\n\
         \x20\x20 name = name.split(sep)[0]\n\
         \x20 name = name.strip()\n\
         \x20 if name and name not in seen:\n\
         \x20\x20 seen.add(name); print(name)";
    let mut child = match Command::new("python3")
        .arg("-c").arg(script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn() { Ok(c) => c, Err(_) => return };
    child.stdin.take().unwrap().write_all(metadata.as_bytes()).ok();
    let output = match child.wait_with_output() { Ok(o) => o, Err(_) => return };
    if !output.status.success() { return; }
    let deps: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    for dep in &deps {
        resolve_package_tree(&dep, packages_dir, verbose, visited);
    }
}

fn install_package_inner(name: &str, packages_dir: &Path, verbose: bool) {
    if packages_dir.join(name).join("Cargo.toml").exists() {
        eprintln!("{} {} {}", col("skipping", 180, 180, 180), name, col("(already installed)", 120, 120, 120));
        return;
    }
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

    let out_dir = packages_dir.join(name);
    fs::create_dir_all(&out_dir).ok();

    // Try sdist first
    let sdist_url = extract_sdist_url(&metadata);
    let installed_from_sdist = if let Some(url) = sdist_url {
        eprintln!("{} {}", col("downloading", 180, 180, 180), name);
        let mut spinner = Spinner::new(&format!("{} {}", col("fetching", 150, 200, 255), &url[..url.len().min(80)]));
        let archive_data_opt = fetch_bytes(&url);
        spinner.stop();

        if let Some(archive_data) = archive_data_opt {
            if archive_data.len() > 0 {
                eprintln!("{} ({})", styled("OK", format!("Downloaded {}", name).as_str()), format!("{} bytes", archive_data.len()));
                let temp_dir = packages_dir.join(".tmp").join(name);
                fs::create_dir_all(&temp_dir).ok();
                eprintln!("{}", styled("INFO", format!("Extracting {}...", name).as_str()));
                extract_archive(&archive_data, &temp_dir, &url);

                let pkg_root = find_package_root(&temp_dir, name);
                // Check if sdist has .py files
                let has_py = has_py_files(&pkg_root);
                if has_py {
                    eprintln!("{}", styled("INFO", format!("Transforming {} to rython...", name).as_str()));
                    let mut spinner = Spinner::new(&format!("{}", col("transpiling modules", 150, 200, 150)));
                    transpile_package(&pkg_root, &out_dir, verbose);
                    spinner.stop();
                    copy_source_files(&pkg_root, &out_dir);
                    if !verbose {
                        fs::remove_dir_all(&temp_dir).ok();
                    }
                    generate_cargo_toml(name, &out_dir);
                    eprintln!("{} {} → {:?}", styled("DONE", name), col("installed (sdist)", 150, 255, 150), out_dir);
                    true
                } else {
                    // No .py files, need wheel
                    fs::remove_dir_all(&temp_dir).ok();
                    false
                }
            } else {
                false
            }
        } else {
            spinner.stop();
            eprintln!("{} {}", styled("WARN", "Failed to download sdist, trying wheel"), name);
            false
        }
    } else {
        false
    };

    // Fallback to wheel if sdist failed or had no .py files
    if !installed_from_sdist {
        eprintln!("{} {}", styled("WARN", "No sdist or no .py files, trying wheel"), name);
        // Detect Python version for wheel selection (e.g., "cp314" for Python 3.14)
        let py_ver = get_python_wheel_tag();
        let wheel_url = extract_wheel_url(&metadata, &py_ver);
        if let Some(url) = wheel_url {
            eprintln!("{} {}", col("downloading wheel", 180, 180, 180), name);
            let mut spinner = Spinner::new(&format!("{} {}", col("fetching wheel", 150, 200, 255), &url[..url.len().min(80)]));
            let wheel_data = match fetch_bytes(&url) {
                Some(d) => d,
                None => {
                    spinner.stop();
                    eprintln!("{} {}", styled("ERR", "Failed to download wheel"), name);
                    return;
                }
            };
            spinner.stop();
            eprintln!("{} ({})", styled("OK", format!("Downloaded wheel {}", name).as_str()), format!("{} bytes", wheel_data.len()));

            let temp_dir = packages_dir.join(".tmp").join(format!("{}_wheel", name));
            fs::create_dir_all(&temp_dir).ok();
            eprintln!("{}", styled("INFO", format!("Extracting wheel {}...", name).as_str()));
            extract_archive(&wheel_data, &temp_dir, &url);

            let pkg_root = find_package_root(&temp_dir, name);
            // For wheels, we just copy the .so/.pyd files and .py for fallback
            copy_source_files(&pkg_root, &out_dir);
            if !verbose {
                fs::remove_dir_all(&temp_dir).ok();
            }
            // Generate minimal Cargo.toml for wheel packages
            generate_cargo_toml(name, &out_dir);
            eprintln!("{} {} → {:?} (wheel)", styled("DONE", name), col("installed", 150, 255, 150), out_dir);
        } else {
            eprintln!("{} {}", styled("ERR", "No wheel available for"), name);
        }
    }
}

fn has_py_files(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false) {
                if has_py_files(&path) { return true; }
            } else if path.extension().map(|e| e == "py").unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

fn get_python_wheel_tag() -> String {
    // Get Python version for wheel tag (e.g., "cp314" for CPython 3.14)
    let output = Command::new("python3")
        .args(["-c", "import sys; print(f'cp{sys.version_info.major}{sys.version_info.minor}')"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok());
    output.map(|s| s.trim().to_string()).unwrap_or_else(|| "cp314".to_string())
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



const MAX_RETRIES: u32 = 3;

fn fetch_url(url: &str) -> Option<String> {
    for attempt in 0..MAX_RETRIES {
        let result = Command::new("curl")
            .args(["-sSL", "--retry", "2", "--connect-timeout", "15", url])
            .output().ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok());
        if let Some(data) = result {
            if !data.is_empty() { return Some(data); }
        }
        if attempt + 1 < MAX_RETRIES {
            thread::sleep(Duration::from_millis(400 * (attempt as u64 + 1)));
        }
    }
    None
}

fn fetch_bytes(url: &str) -> Option<Vec<u8>> {
    for attempt in 0..MAX_RETRIES {
        let result = Command::new("curl")
            .args(["-sSL", "--retry", "2", "--connect-timeout", "15", url])
            .output().ok()
            .filter(|o| o.status.success())
            .map(|o| o.stdout);
        if let Some(data) = result {
            if !data.is_empty() { return Some(data); }
        }
        if attempt + 1 < MAX_RETRIES {
            thread::sleep(Duration::from_millis(400 * (attempt as u64 + 1)));
        }
    }
    None
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

fn extract_wheel_url(metadata: &str, py_version: &str) -> Option<String> {
    let script = format!(
        "import json,sys; data=json.loads(sys.stdin.read());\n\
         py_ver = '{}';\n\
         for u in data['urls']:\n\
         \x20 if u['packagetype']=='bdist_wheel' and py_ver in u['python_version']:\n\
         \x20\x20 print(u['url']); break\n\
         else:\n\
         \x20 for u in data['urls']:\n\
         \x20\x20 if u['packagetype']=='bdist_wheel':\n\
         \x20\x20\x20 print(u['url']); break",
        py_version
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

fn copy_source_files(src_dir: &Path, out_dir: &Path) {
    let skip_dirs = ["docs", "tests", "changes", "__pycache__", ".git", ".github", "examples"];
    let entries = match fs::read_dir(src_dir) { Ok(e) => e, Err(_) => return };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false) {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            if dir_name.starts_with('.') || skip_dirs.contains(&dir_name.as_str()) { continue; }
            let sub_out = out_dir.join(&dir_name);
            fs::create_dir_all(&sub_out).ok();
            copy_source_files(&path, &sub_out);
        } else {
            // Copy all files for python3 fallback (py, pem, txt, json, etc.)
            if let Ok(data) = fs::read(&path) {
                let out_path = out_dir.join(entry.file_name());
                fs::write(&out_path, &data).ok();
            }
        }
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
            let dir_name = entry.file_name().to_string_lossy().to_string();
            if dir_name.starts_with('.') || dir_name == "__pycache__" { continue; }
            let mut sub_mod = String::new();
            let sub_out = out_dir.join(&dir_name);
            walk_py_files(root, &path, &sub_out, &mut sub_mod, verbose);
            if !sub_mod.trim().is_empty() {
                fs::create_dir_all(&sub_out).ok();
                fs::write(sub_out.join("mod.rs"), &sub_mod).ok();
                mod_file.push_str(&format!("pub mod {};\n", dir_name));
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if file_stem == "__init__" {
                if let Ok(source) = fs::read_to_string(&path) {
                    transpile_source_to_mod(source, mod_file, verbose);
                }
            } else if file_stem != "__pycache__" {
                if let Ok(source) = fs::read_to_string(&path) {
                    let rust = transpile_py_to_rust(&source, verbose);
                    if !rust.trim().is_empty() {
                        fs::write(out_dir.join(format!("{}.rs", file_stem)), &rust).ok();
                    } else {
                        let stub = format!("// stub: {}.py could not be transpiled\n", file_stem);
                        fs::write(out_dir.join(format!("{}.rs", file_stem)), &stub).ok();
                    }
                    mod_file.push_str(&format!("pub mod {};\n", file_stem));
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
    let code = safe_transpile(&source);
    for line in code.lines() {
        let trimmed = line.trim();
        // Skip use statements referencing sibling modules (stubs won't export them)
        if trimmed.starts_with("use ") && !trimmed.starts_with("use super::") && !trimmed.starts_with("use crate::") {
            continue;
        }
        if trimmed.is_empty() || trimmed == "#![allow(dead_code)]" || trimmed.starts_with("// Source:") {
            continue;
        }
        output.push_str(line);
        output.push('\n');
    }
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

// ── Manifest (rython.json) support ──────────────────────────

fn manifest_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("rython.json")
}

fn read_manifest_deps() -> Vec<String> {
    let path = manifest_path();
    if !path.exists() { return Vec::new(); }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    // Simple JSON parsing: find "dependencies": [...] section
    let mut deps = Vec::new();
    let mut in_deps = false;
    for line in content.lines() {
        let line = line.trim();
        if line.contains("\"dependencies\"") {
            in_deps = true;
            continue;
        }
        if in_deps {
            if line.contains(']') {
                break;
            }
            if line.starts_with('"') {
                let dep = line.trim_end_matches(',').trim_matches('"');
                if !dep.is_empty() && !dep.contains('{') && !dep.contains('}') {
                    deps.push(dep.to_string());
                }
            }
        }
    }
    deps
}

fn add_to_manifest(pkg: &str) {
    let path = manifest_path();
    let mut deps = read_manifest_deps();
    let pkg_base = pkg.split(&['[', '>', '<', '=', '!', '~'][..]).next().unwrap_or(pkg).trim().to_string();
    if deps.contains(&pkg_base) { return; }
    deps.push(pkg_base);

    let json_content = format!(
        "{{\n  \"name\": \"project\",\n  \"version\": \"0.1.0\",\n  \"dependencies\": [\n{}\n  ]\n}}",
        deps.iter().map(|d| format!("    \"{}\"", d)).collect::<Vec<_>>().join(",\n")
    );
    fs::write(&path, &json_content).ok();
    eprintln!("{} {}", styled("INFO", "Added"), format!("{} to rython.json", pkg));
}

fn remove_from_manifest(pkg: &str) {
    let path = manifest_path();
    if !path.exists() { return; }
    let mut deps = read_manifest_deps();
    let pkg_base = pkg.split(&['[', '>', '<', '=', '!', '~'][..]).next().unwrap_or(pkg).trim().to_string();
    deps.retain(|d| d != &pkg_base);

    let json_content = format!(
        "{{\n  \"name\": \"project\",\n  \"version\": \"0.1.0\",\n  \"dependencies\": [\n{}\n  ]\n}}",
        deps.iter().map(|d| format!("    \"{}\"", d)).collect::<Vec<_>>().join(",\n")
    );
    fs::write(&path, &json_content).ok();
    eprintln!("{} {}", styled("INFO", "Removed"), format!("{} from rython.json", pkg));
}

// ── .pyc cache support ──────────────────────────────────────

fn compile_to_pycache(packages_dir: &Path) {
    // Create __pycache__ directories and compile .py files to .pyc
    let pycache_dir = packages_dir.join("__pycache__");
    fs::create_dir_all(&pycache_dir).ok();

    let entries: Vec<_> = match fs::read_dir(packages_dir) {
        Ok(e) => e.filter_map(|x| x.ok()).collect(),
        Err(_) => return,
    };

    for entry in entries {
        if !entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || name == "__pycache__" { continue; }

        let pkg_path = entry.path();
        let pkg_pycache = pkg_path.join("__pycache__");
        fs::create_dir_all(&pkg_pycache).ok();

        // Find all .py files and compile them
        compile_py_dir(&pkg_path, &pkg_pycache);
    }
}

fn compile_py_dir(src_dir: &Path, pycache_dir: &Path) {
    let entries = match fs::read_dir(src_dir) { Ok(e) => e, Err(_) => return };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false) {
            let dir_name = entry.file_name().to_string_lossy().to_string();
            if dir_name.starts_with('.') || dir_name == "__pycache__" { continue; }
            let sub_pycache = pycache_dir.join(&dir_name);
            fs::create_dir_all(&sub_pycache).ok();
            compile_py_dir(&path, &sub_pycache);
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            compile_py_to_pyc(&path, pycache_dir);
        }
    }
}

fn compile_py_to_pyc(py_path: &Path, pycache_dir: &Path) {
    let file_name = match py_path.file_name() {
        Some(n) => n.to_string_lossy().to_string(),
        None => return,
    };
    let pyc_name = format!("{}.c", file_name); // Simple .pyc alternative
    let pyc_path = pycache_dir.join(&pyc_name);

    // Check if already cached and source hasn't changed
    if let Ok(src_meta) = fs::metadata(py_path) {
        if let Ok(pyc_meta) = fs::metadata(&pyc_path) {
            if pyc_meta.modified().ok() > src_meta.modified().ok() {
                return; // Already up to date
            }
        }
    }

    // Use python3 to compile to .pyc
    let result = Command::new("python3")
        .args(["-c", &format!(
            "import py_compile, sys, os; \
             src='{}'; \
             dest='{}'; \
             py_compile.compile(src, dest, doraise=False)",
            py_path.to_string_lossy().replace('\\', "\\\\"),
            pyc_path.to_string_lossy().replace('\\', "\\\\")
        )])
        .output();

    if let Ok(out) = result {
        if !out.status.success() {
            // Fallback: just copy the .py file as cache marker
            let _ = fs::copy(py_path, &pyc_path);
        }
    } else {
        // Fallback: copy .py as marker
        let _ = fs::copy(py_path, &pyc_path);
    }
}

// ── Build command ───────────────────────────────────────────

fn build_project(args: &[String], verbose: bool) {
    if args.is_empty() {
        eprintln!("{}", styled("ERR", "Usage: rip build <file.py> [--output <binary>]"));
        exit(1);
    }

    let filename = &args[0];
    let output_file = args.iter().position(|a| a == "--output").map(|i| args[i + 1].clone());

    // First ensure all dependencies are installed
    let deps = read_manifest_deps();
    if !deps.is_empty() {
        eprintln!("{}", styled("INFO", format!("Ensuring {} dependencies are installed", deps.len()).as_str()));
        let packages_dir = PathBuf::from("rython_packages");
        install_parallel(&deps, &packages_dir, verbose);
    }

    // Use rython to compile the main file
    let cache_dir = Path::new(".rython_cache");
    let _ = fs::create_dir_all(cache_dir);

    let output_path = output_file.clone().unwrap_or_else(|| {
        let base = Path::new(filename).file_stem().unwrap_or_default().to_string_lossy();
        base.to_string()
    });

    eprintln!("{}", styled("INFO", format!("Building {} to binary", filename).as_str()));

    // Find rython binary (same directory as rip or in PATH)
    let rython_bin = std::env::current_exe()
        .ok()
        .map(|p| p.parent().map(|d| d.join("rython")).filter(|p| p.exists()).unwrap_or(p))
        .unwrap_or_else(|| std::path::PathBuf::from("rython"));

    // Call rython compiler
    let mut cmd = Command::new(&rython_bin);
    cmd.arg(filename)
        .arg("--output")
        .arg(cache_dir.join(format!("{}.rs", Path::new(filename).file_stem().unwrap_or_default().to_string_lossy())));
    if verbose {
        cmd.arg("--verbose");
    }

    let compile_result = cmd.output();
    match compile_result {
        Ok(out) => {
            if !out.status.success() {
                eprintln!("{}", styled("ERR", "Transpilation failed"));
                if verbose {
                    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
                }
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("{} {}", styled("ERR", "Failed to run rython:"), e);
            exit(1);
        }
    }

    // Now compile the Rust with rustc
    let rs_path = cache_dir.join(format!("{}.rs", Path::new(filename).file_stem().unwrap_or_default().to_string_lossy()));
    let rust_source = match fs::read_to_string(&rs_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", styled("ERR", "Failed to read generated Rust:"), e);
            exit(1);
        }
    };

    // Compile using the same logic as main.rs
    match compile_rust_standalone(&rust_source, &output_path, verbose) {
        Ok(_) => {
            eprintln!("{} {} → {}", styled("DONE", "Built"), filename, col(&output_path, 0, 255, 128));
        }
        Err(e) => {
            eprintln!("{} {}", styled("ERR", "Rust compilation failed:"), e);
            exit(1);
        }
    }
}

fn compile_rust_standalone(rust_source: &str, output_path: &str, _verbose: bool) -> Result<(), String> {
    let tmp_dir = std::env::temp_dir().join("rython_build");
    std::fs::create_dir_all(&tmp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;
    let rs_file = tmp_dir.join("main.rs");
    std::fs::write(&rs_file, rust_source).map_err(|e| format!("Failed to write source: {}", e))?;

    let mut cmd = Command::new("rustc");
    cmd.arg("--edition").arg("2021")
        .arg("-o").arg(output_path)
        .arg(rs_file.to_string_lossy().as_ref())
        .arg("-C").arg("opt-level=3")
        .arg("-C").arg("strip=symbols");

    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let output = cmd.output().map_err(|e| format!("Failed to run rustc: {}", e))?;
    if output.status.success() { Ok(()) }
    else { Err(String::from_utf8_lossy(&output.stderr).to_string()) }
}
