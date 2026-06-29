use std::fs;
use std::process::Command;

const SRC_MARKER_START: &[u8] = b"__RYTHON_SRC__";
const SRC_MARKER_END: &[u8] = b"__RYTHON_END__";

/// Embed Python source code as a `#[used] static` so it survives in the binary.
/// Uses `r###" ... "###` delimiters; picks N hashes so it never clashes.
pub fn embed_source(source: &str) -> String {
    let mut hash_count = 3;
    let probe = format!("\"{}#", "#".repeat(hash_count));
    while source.contains(&probe) {
        hash_count += 1;
    }
    let hashes = "#".repeat(hash_count);
    format!(
        "#[used]\n\
         static RYTHON_SRC: &str = r{hashes}\"{marker_start}\n{source}\n{marker_end}\"{hashes};\n",
        hashes = hashes,
        marker_start = std::str::from_utf8(SRC_MARKER_START).unwrap(),
        marker_end = std::str::from_utf8(SRC_MARKER_END).unwrap(),
        source = source
    )
}

/// Decompile a binary: extract the embedded Python source code.
pub fn decompile(binary_path: &str) -> Result<String, String> {
    let data = fs::read(binary_path).map_err(|e| format!("Cannot read '{}': {}", binary_path, e))?;

    // Try to find embedded source directly in bytes (always present)
    if let Some(src) = extract_from_bytes(&data) {
        return Ok(src);
    }

    // Fallback: try strings command on stripped binaries
    if let Ok(src) = extract_via_strings(binary_path) {
        return Ok(src);
    }

    // Fallback: try to find .py file alongside the binary
    if let Some(src) = find_py_alongside(binary_path) {
        return Ok(src);
    }

    Err("No embedded source in binary. Recompile the .py with the latest rython, or place the original .py next to the binary.".to_string())
}

fn extract_from_bytes(data: &[u8]) -> Option<String> {
    let start = find_pattern(data, SRC_MARKER_START)?;
    let start_after = start + SRC_MARKER_START.len();

    // Skip leading newline after marker
    let start_after = if data.get(start_after) == Some(&b'\n') {
        start_after + 1
    } else {
        start_after
    };

    let end = find_pattern(&data[start_after..], SRC_MARKER_END)?;
    let end_actual = start_after + end;

    // Trim trailing newline before end marker
    let end_actual = if data.get(end_actual - 1) == Some(&b'\n') {
        end_actual - 1
    } else {
        end_actual
    };

    let src_bytes = &data[start_after..end_actual];
    String::from_utf8(src_bytes.to_vec()).ok()
}

fn extract_via_strings(binary_path: &str) -> Result<String, String> {
    let output = Command::new("strings")
        .arg(binary_path)
        .output()
        .map_err(|e| format!("strings command failed: {}", e))?;

    if !output.status.success() {
        return Err("strings command returned error".to_string());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();

    let mut in_source = false;
    let mut source_lines = Vec::new();

    for line in &lines {
        if line.trim() == std::str::from_utf8(SRC_MARKER_START).unwrap() {
            in_source = true;
            continue;
        }
        if line.trim() == std::str::from_utf8(SRC_MARKER_END).unwrap() {
            in_source = false;
            break;
        }
        if in_source {
            source_lines.push(*line);
        }
    }

    if source_lines.is_empty() {
        return Err("No embedded source via strings".to_string());
    }

    Ok(source_lines.join("\n"))
}

fn find_py_alongside(binary_path: &str) -> Option<String> {
    let p = std::path::Path::new(binary_path);
    let dir = p.parent().unwrap_or(std::path::Path::new("."));
    let stem = p.file_stem().map(|s| s.to_string_lossy()).unwrap_or_default();

    // Try binary stem name first, then app.py, then any .py in the directory
    let candidates = [
        dir.join(format!("{}.py", stem)),
        dir.join("app.py"),
        dir.join("main.py"),
    ];

    for path in &candidates {
        if let Ok(src) = fs::read_to_string(path) {
            return Some(src);
        }
    }

    // Fallback: pick the first .py in the directory
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "py").unwrap_or(false) {
                if let Ok(src) = fs::read_to_string(&path) {
                    return Some(src);
                }
            }
        }
    }

    None
}

fn find_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len())
        .position(|window| window == pattern)
}
