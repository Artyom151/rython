use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};
use std::sync::Arc;
use std::sync::Mutex;

pub fn run_lsp() {
    let state = Arc::new(Mutex::new(LspState::new()));
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    loop {
        input.clear();
        let mut content_length: Option<usize> = None;

        loop {
            let mut header = String::new();
            if reader.read_line(&mut header).ok() != Some(0) && !header.is_empty() {
                let header = header.trim();
                if header.is_empty() {
                    break;
                }
                if let Some(len_str) = header.strip_prefix("Content-Length: ") {
                    content_length = len_str.trim().parse().ok();
                }
            } else {
                return;
            }
        }

        if let Some(len) = content_length {
            let mut buf = vec![0u8; len];
            if reader.read_exact(&mut buf).is_err() {
                return;
            }
            let body = String::from_utf8_lossy(&buf).to_string();
            let response = handle_message(&body, &state);
            if let Some(resp) = response {
                write_response(&resp);
            }
        }
    }
}

fn write_response(json: &str) {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let _ = write!(out, "Content-Length: {}\r\n\r\n{}", json.len(), json);
    let _ = out.flush();
}

struct LspState {
    documents: HashMap<String, String>,
    next_id: u64,
}

impl LspState {
    fn new() -> Self {
        LspState { documents: HashMap::new(), next_id: 1 }
    }
}

fn handle_message(body: &str, state: &Arc<Mutex<LspState>>) -> Option<String> {
    let method = json_get_string(body, "method");
    let id_val = json_get_value(body, "id");
    let id = id_val.as_deref().unwrap_or("null").to_string();
    let params = json_get_value(body, "params").unwrap_or_default();

    match method.as_deref() {
        Some("initialize") => {
            Some(initialize_response(&id))
        }
        Some("initialized") => None,
        Some("shutdown") => {
            Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#, id))
        }
        Some("exit") => std::process::exit(0),
        Some("textDocument/didOpen") => {
            let uri = json_get_string(&params, "textDocument.uri").unwrap_or_default();
            let text = json_get_string(&params, "textDocument.text").unwrap_or_default();
            let mut s = state.lock().unwrap();
            s.documents.insert(uri.clone(), text.clone());
            send_diagnostics(&uri, &text);
            None
        }
        Some("textDocument/didChange") => {
            let uri = json_get_string(&params, "textDocument.uri").unwrap_or_default();
            let text = json_get_string(&params, "contentChanges[0].text").unwrap_or_default();
            let mut s = state.lock().unwrap();
            s.documents.insert(uri.clone(), text.clone());
            send_diagnostics(&uri, &text);
            None
        }
        Some("textDocument/completion") => {
            let uri = json_get_string(&params, "textDocument.uri").unwrap_or_default();
            let line = json_get_f64(&params, "position.line").unwrap_or(0.0) as usize;
            let col = json_get_f64(&params, "position.character").unwrap_or(0.0) as usize;
            let text = {
                let s = state.lock().unwrap();
                s.documents.get(&uri).cloned()
            };
            if let Some(ref src) = text {
                Some(completion_response(&id, src, line, col))
            } else {
                Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":{{"isIncomplete":false,"items":[]}}}}"#, id))
            }
        }
        Some("textDocument/hover") => {
            let uri = json_get_string(&params, "textDocument.uri").unwrap_or_default();
            let line = json_get_f64(&params, "position.line").unwrap_or(0.0) as usize;
            let col = json_get_f64(&params, "position.character").unwrap_or(0.0) as usize;
            let text = {
                let s = state.lock().unwrap();
                s.documents.get(&uri).cloned()
            };
            if let Some(ref src) = text {
                Some(hover_response(&id, src, line, col))
            } else {
                Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#, id))
            }
        }
        Some("textDocument/definition") => {
            Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#, id))
        }
        Some("textDocument/diagnostic") => {
            let uri = json_get_string(&params, "textDocument.uri").unwrap_or_default();
            let text = {
                let s = state.lock().unwrap();
                s.documents.get(&uri).cloned()
            };
            if let Some(ref src) = text {
                Some(diagnostic_full_response(&id, src))
            } else {
                Some(format!(r#"{{"jsonrpc":"2.0","id":{},"result":{{"kind":"full","items":[]}}}}"#, id))
            }
        }
        _ => None,
    }
}

fn initialize_response(id: &str) -> String {
    format!(r#"{{
        "jsonrpc":"2.0","id":{},"result":{{
            "capabilities":{{
                "textDocumentSync":2,
                "completionProvider":{{"triggerCharacters":["."]}},
                "hoverProvider":true,
                "definitionProvider":true,
                "diagnosticProvider":{{"interFileDependencies":false,"workspaceDiagnostics":false}}
            }},
            "serverInfo":{{"name":"rython-lsp","version":"0.1.0"}}
        }}
    }}"#, id)
}

fn parse_with_diags(source: &str) -> Vec<String> {
    let diags = Arc::new(Mutex::new(Vec::new()));
    let diags_clone = diags.clone();
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else {
            "Parse error".to_string()
        };
        let line = msg.rsplit("line ").next()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        let mut d = diags_clone.lock().unwrap();
        d.push(format!(
            r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":100}}}},
            "severity":1,"message":{},"source":"rython"}}"#,
            line.saturating_sub(1),
            line.saturating_sub(1),
            json_escape(&msg)
        ));
    }));

    let tokens = crate::lexer::tokenize(source);
    let mut parser = crate::parser::Parser::new(tokens);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        parser.parse();
    }));

    std::panic::set_hook(prev);
    Arc::try_unwrap(diags).ok().map(|m| m.into_inner().unwrap()).unwrap_or_default()
}

fn send_diagnostics(uri: &str, source: &str) {
    let diags = parse_with_diags(source);
    let notification = format!(
        r#"{{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics",
        "params":{{"uri":{},"diagnostics":[{}]}}}}"#,
        json_escape(uri),
        diags.join(",")
    );
    write_response(&notification);
}

fn diagnostic_full_response(id: &str, source: &str) -> String {
    let items = parse_with_diags(source);
    format!(r#"{{"jsonrpc":"2.0","id":{},"result":{{"kind":"full","items":[{}]}}}}"#, id, items.join(","))
}

fn completion_response(id: &str, _source: &str, _line: usize, _col: usize) -> String {
    let keywords = [
        "def", "class", "if", "elif", "else", "for", "while", "return",
        "import", "from", "as", "pass", "break", "continue", "try",
        "except", "finally", "raise", "with", "yield", "lambda", "assert",
        "global", "nonlocal", "del", "async", "await", "and", "or", "not",
        "in", "is", "True", "False", "None",
    ];
    let builtins = [
        "print", "len", "type", "int", "float", "str", "list", "dict",
        "set", "tuple", "bool", "range", "enumerate", "zip", "reversed",
        "sorted", "min", "max", "sum", "abs", "any", "all", "isinstance",
        "hasattr", "getattr", "setattr", "map", "filter", "open", "input",
    ];
    let stdlib = [
        "math.", "sys.", "os.", "json.", "time.", "re.", "random.",
        "collections.", "itertools.", "functools.", "pathlib.", "datetime.",
        "typing.", "abc.",
    ];

    let mut items = Vec::new();
    for kw in &keywords {
        items.push(format!(r#"{{"label":{},"kind":14,"detail":"keyword"}}"#, json_escape(kw)));
    }
    for b in &builtins {
        items.push(format!(r#"{{"label":{},"kind":3,"detail":"builtin"}}"#, json_escape(b)));
    }
    for s in &stdlib {
        items.push(format!(r#"{{"label":{},"kind":9,"detail":"module"}}"#, json_escape(s)));
    }

    format!(r#"{{"jsonrpc":"2.0","id":{},"result":{{"isIncomplete":false,"items":[{}]}}}}"#, id, items.join(","))
}

fn hover_response(id: &str, source: &str, line: usize, col: usize) -> String {
    let word = extract_word(source, line, col);

    let info = match word.as_str() {
        "print" => "```python\nprint(*objects, sep=' ', end='\\n', file=None, flush=False)\n```\nPrints values to stdout.",
        "len" => "```python\nlen(obj)\n```\nReturns the length of a sequence or collection.",
        "type" => "```python\ntype(obj)\n```\nReturns the type of an object.",
        "range" => "```python\nrange(stop)\nrange(start, stop[, step])\n```\nCreates an iterable sequence of integers.",
        "int" => "```python\nint(x=0) -> int\nint(x, base=10) -> int\n```\nConverts to integer.",
        "float" => "```python\nfloat(x=0.0) -> float\n```\nConverts to float.",
        "str" => "```python\nstr(obj='') -> str\n```\nConverts to string.",
        "list" => "```python\nlist(iterable=()) -> list\n```\nCreates a list.",
        "dict" => "```python\ndict(**kwargs) -> dict\ndict(mapping) -> dict\n```\nCreates a dictionary.",
        "set" => "```python\nset(iterable) -> set\n```\nCreates a set.",
        "True" => "```python\nTrue\n```\nBoolean true value.",
        "False" => "```python\nFalse\n```\nBoolean false value.",
        "None" => "```python\nNone\n```\nNull value.",
        _ => {
            if word.starts_with("math.") {
                "```python\nmath module — mathematical functions\n```"
            } else if word.starts_with("sys.") {
                "```python\nsys module — system-specific parameters\n```"
            } else if word.starts_with("os.") {
                "```python\nos module — operating system interface\n```"
            } else if word.starts_with("json.") {
                "```python\njson module — JSON encoder/decoder\n```"
            } else {
                return format!(r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#, id);
            }
        }
    };

    format!(r#"{{"jsonrpc":"2.0","id":{},"result":{{"contents":{{"kind":"markdown","value":{}}}}}}}"#, id, json_escape(info))
}

fn extract_word(source: &str, line: usize, col: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();
    if line >= lines.len() { return String::new(); }
    let l = lines[line];
    if col >= l.len() { return String::new(); }

    let chars: Vec<char> = l.chars().collect();
    let mut start = col;
    let mut end = col;

    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_' || chars[start - 1] == '.') {
        start -= 1;
    }
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_' || chars[end] == '.') {
        end += 1;
    }

    chars[start..end].iter().collect()
}

fn json_get_string(body: &str, path: &str) -> Option<String> {
    let val = json_get_value(body, path)?;
    let trimmed = val.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        let inner = &trimmed[1..trimmed.len()-1];
        Some(json_unescape(inner))
    } else {
        None
    }
}

fn json_get_f64(body: &str, path: &str) -> Option<f64> {
    let val = json_get_value(body, path)?;
    val.trim().parse().ok()
}

fn json_get_value<'a>(body: &'a str, path: &'a str) -> Option<String> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = body.trim();

    for part in &parts {
        let search;
        let is_index;
        if let Some(idx_str) = part.strip_suffix(']') {
            if let Some(key) = idx_str.split_once('[') {
                search = key.0;
                is_index = Some(key.1);
            } else { return None; }
        } else {
            search = part;
            is_index = None;
        }

        let key = format!("\"{}\"", search);
        if let Some(pos) = current.find(&key) {
            let after_key = &current[pos + key.len()..].trim_start();
            if after_key.starts_with(':') {
                let after_colon = after_key[1..].trim_start();
                if let Some(idx) = is_index {
                    let idx: usize = idx.parse().ok()?;
                    if after_colon.starts_with('[') {
                        let arr = find_json_array(after_colon)?;
                        let items = parse_json_array(arr);
                        let s = items.get(idx)?.clone();
                        return Some(s);
                    } else { return None; }
                } else {
                    current = after_colon;
                }
            } else { return None; }
        } else { return None; }
    }

    // Extract the value (up to comma or closing bracket)
    let mut depth = 0i32;
    let mut in_str = false;
    let mut end = 0;
    for (i, c) in current.char_indices() {
        if in_str {
            if c == '\\' { continue; }
            if c == '"' { in_str = false; }
        } else {
            match c {
                '"' => in_str = true,
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                ',' | '}' | ']' if depth <= 0 => { end = i; break; }
                _ => {}
            }
        }
        end = i + 1;
    }
    Some(current[..end].trim().to_string())
}

fn find_json_array(s: &str) -> Option<&str> {
    let s = s.trim_start();
    if s.starts_with('[') {
        let mut depth = 0i32;
        let mut in_str = false;
        for (i, c) in s.char_indices() {
            if in_str {
                if c == '\\' { continue; }
                if c == '"' { in_str = false; }
            } else {
                match c {
                    '"' => in_str = true,
                    '[' => depth += 1,
                    ']' => { depth -= 1; if depth == 0 { return Some(&s[1..i]); } }
                    _ => {}
                }
            }
        }
    }
    None
}

fn parse_json_array(s: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if in_str {
            if c == '\\' { continue; }
            if c == '"' { in_str = false; }
        } else {
            match c {
                '"' => in_str = true,
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    items.push(s[start..i].trim().to_string());
                    start = i + 1;
                }
                _ => {}
            }
        }
    }
    if start < s.len() {
        let last = s[start..].trim().to_string();
        if !last.is_empty() { items.push(last); }
    }
    items
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(code) {
                            out.push(ch);
                        }
                    }
                }
                Some(c) => out.push(c),
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}
