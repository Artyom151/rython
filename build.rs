fn build_wrapper(name: &str, pkgs: &[&str], file: &str) {
    let flags = std::process::Command::new("pkg-config")
        .args(["--cflags", "--libs"]).args(pkgs)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    let mut cfg = cc::Build::new();
    cfg.cpp(true)
        .file(file)
        .std("c++17");

    let has_pkgs = std::process::Command::new("pkg-config")
        .args(pkgs).arg("--exists")
        .status().ok().map(|s| s.success()).unwrap_or(false);

    if has_pkgs {
        for flag in flags.split_whitespace() {
            if let Some(include) = flag.strip_prefix("-I") {
                cfg.include(include);
            } else if let Some(lib) = flag.strip_prefix("-l") {
                cfg.flag(&format!("-l{}", lib));
            } else if let Some(def) = flag.strip_prefix("-D") {
                cfg.define(def, None);
            }
        }
    }

    cfg.compile(&format!("{}_wrapper", name));
    println!("cargo:rerun-if-changed={}", file);
}

fn main() {
    build_wrapper("qt6", &["Qt6Widgets", "Qt6Core", "Qt6Gui"], "src/wrappers/qt6.cpp");
    build_wrapper("opengl", &["x11", "gl"], "src/wrappers/opengl.cpp");
    build_wrapper("gtk4", &["gtk4"], "src/wrappers/gtk4.cpp");
    build_wrapper("ffmpeg", &["libavformat", "libavcodec", "libavutil", "libswscale"], "src/wrappers/ffmpeg.cpp");
}
