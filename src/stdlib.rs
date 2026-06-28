


pub mod math {
    use super::Value;

    pub fn sqrt(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.sqrt(),
            Value::Int(i) => (i as f64).sqrt(),
            _ => 0.0,
        })
    }

    pub fn sin(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.sin(),
            Value::Int(i) => (i as f64).sin(),
            _ => 0.0,
        })
    }

    pub fn cos(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.cos(),
            Value::Int(i) => (i as f64).cos(),
            _ => 0.0,
        })
    }

    pub fn tan(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.tan(),
            Value::Int(i) => (i as f64).tan(),
            _ => 0.0,
        })
    }

    pub fn asin(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.asin(),
            Value::Int(i) => (i as f64).asin(),
            _ => 0.0,
        })
    }

    pub fn acos(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.acos(),
            Value::Int(i) => (i as f64).acos(),
            _ => 0.0,
        })
    }

    pub fn atan(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.atan(),
            Value::Int(i) => (i as f64).atan(),
            _ => 0.0,
        })
    }

    pub fn atan2(y: Value, x: Value) -> Value {
        let yv = match y { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let xv = match x { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        Value::Float(yv.atan2(xv))
    }

    pub fn hypot(x: Value, y: Value) -> Value {
        let xv = match x { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let yv = match y { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        Value::Float(xv.hypot(yv))
    }

    pub fn floor(x: Value) -> Value {
        Value::Int(match x {
            Value::Float(f) => f.floor() as i64,
            Value::Int(i) => i,
            _ => 0,
        })
    }

    pub fn ceil(x: Value) -> Value {
        Value::Int(match x {
            Value::Float(f) => f.ceil() as i64,
            Value::Int(i) => i,
            _ => 0,
        })
    }

    pub fn trunc(x: Value) -> Value {
        Value::Int(match x {
            Value::Float(f) => f.trunc() as i64,
            Value::Int(i) => i,
            _ => 0,
        })
    }

    pub fn fabs(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.abs(),
            Value::Int(i) => (i as f64).abs(),
            _ => 0.0,
        })
    }

    pub fn pow(x: Value, y: Value) -> Value {
        let xv = match x { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let yv = match y { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        Value::Float(xv.powf(yv))
    }

    pub fn exp(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.exp(),
            Value::Int(i) => (i as f64).exp(),
            _ => 0.0,
        })
    }

    pub fn log(x: Value, base: Value) -> Value {
        let xv = match x { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let bv = match base { Value::Float(f) => f, Value::Int(i) => i as f64, _ => std::f64::consts::E };
        Value::Float(xv.log(bv))
    }

    pub fn log10(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.log10(),
            Value::Int(i) => (i as f64).log10(),
            _ => 0.0,
        })
    }

    pub fn log2(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.log2(),
            Value::Int(i) => (i as f64).log2(),
            _ => 0.0,
        })
    }

    pub fn degrees(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.to_degrees(),
            Value::Int(i) => (i as f64).to_degrees(),
            _ => 0.0,
        })
    }

    pub fn radians(x: Value) -> Value {
        Value::Float(match x {
            Value::Float(f) => f.to_radians(),
            Value::Int(i) => (i as f64).to_radians(),
            _ => 0.0,
        })
    }

    pub fn isclose(a: Value, b: Value) -> Value {
        let av = match a { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let bv = match b { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        Value::Bool((av - bv).abs() < 1e-9)
    }

    pub fn isfinite(x: Value) -> Value {
        let v = match x { Value::Float(f) => f, Value::Int(_) => return Value::Bool(true), _ => f64::NAN };
        Value::Bool(v.is_finite())
    }

    pub fn isinf(x: Value) -> Value {
        let v = match x { Value::Float(f) => f, Value::Int(_) => return Value::Bool(false), _ => f64::NAN };
        Value::Bool(v.is_infinite())
    }

    pub fn isnan(x: Value) -> Value {
        let v = match x { Value::Float(f) => f, Value::Int(_) => return Value::Bool(false), _ => f64::NAN };
        Value::Bool(v.is_nan())
    }

    pub fn copysign(x: Value, y: Value) -> Value {
        let xv = match x { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let yv = match y { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        Value::Float(xv.copysign(yv))
    }

    pub fn gcd(a: Value, b: Value) -> Value {
        let mut av = match a { Value::Int(i) => i.abs(), _ => 0 };
        let mut bv = match b { Value::Int(i) => i.abs(), _ => 0 };
        while bv != 0 {
            let t = bv;
            bv = av % bv;
            av = t;
        }
        Value::Int(av)
    }

    pub fn lcm(a: Value, b: Value) -> Value {
        let av = match a { Value::Int(i) => i, _ => return Value::Int(0) };
        let bv = match b { Value::Int(i) => i, _ => return Value::Int(0) };
        if av == 0 || bv == 0 {
            return Value::Int(0);
        }
        let g = {
            let mut x = av.abs();
            let mut y = bv.abs();
            while y != 0 {
                let t = y;
                y = x % y;
                x = t;
            }
            x
        };
        Value::Int((av.abs() / g) * bv.abs())
    }

    pub fn factorial(n: Value) -> Value {
        let nv = match n {
            Value::Int(i) if i >= 0 => i,
            _ => return Value::None,
        };
        let mut result: i64 = 1;
        for i in 2..=nv {
            result = result.saturating_mul(i);
        }
        Value::Int(result)
    }

    pub fn gamma(x: Value) -> Value {
        let xv = match x { Value::Float(f) => f, Value::Int(i) => i as f64, _ => return Value::None };
        Value::Float(lanczos_gamma(xv))
    }

    fn lanczos_gamma(z: f64) -> f64 {
        if z < 0.5 {
            std::f64::consts::PI / (std::f64::consts::PI * z).sin() * lanczos_gamma(1.0 - z)
        } else {
            let g = 7.0;
            let c = [
                0.99999999999980993,
                676.5203681218851,
                -1259.1392167224028,
                771.32342877765313,
                -176.61502916214059,
                12.507343278686905,
                -0.13857109526572012,
                9.9843695780195716e-6,
                1.5056327351493116e-7,
            ];
            let z = z - 1.0;
            let mut x = c[0];
            for i in 1..c.len() {
                x += c[i] / (z + i as f64);
            }
            let t = z + g + 0.5;
            (2.0 * std::f64::consts::PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * x
        }
    }

    pub fn pi() -> Value {
        Value::Float(std::f64::consts::PI)
    }

    pub fn e() -> Value {
        Value::Float(std::f64::consts::E)
    }

    pub fn tau() -> Value {
        Value::Float(std::f64::consts::TAU)
    }

    pub fn inf() -> Value {
        Value::Float(f64::INFINITY)
    }

    pub fn nan() -> Value {
        Value::Float(f64::NAN)
    }
}

pub mod sys {
    use super::Value;
    use std::env;
    use std::process;

    pub fn argv() -> Value {
        let args: Vec<String> = env::args().collect();
        Value::Tuple(args.into_iter().map(|s| Value::Str(s)).collect())
    }

    pub fn exit(code: Value) -> ! {
        let c = match code {
            Value::Int(i) => i as i32,
            Value::Str(_) => 0,
            _ => 0,
        };
        process::exit(c)
    }

    pub fn stdout() -> Value {
        Value::Str("<stdout>".to_string())
    }

    pub fn stdin() -> Value {
        Value::Str("<stdin>".to_string())
    }

    pub fn stderr() -> Value {
        Value::Str("<stderr>".to_string())
    }

    pub fn version() -> Value {
        Value::Str("3.12.0 (rython)".to_string())
    }

    pub fn executable() -> Value {
        Value::Str(env::current_exe().unwrap_or_default().to_string_lossy().to_string())
    }

    pub fn modules() -> Value {
        Value::Dict(std::collections::BTreeMap::new())
    }
}

pub mod os {
    use super::Value;
    use std::env;

    pub mod path_ {
        use super::super::Value;
        use std::path::Path;

        pub fn join(a: Value, b: Value) -> Value {
            let ap = match a { Value::Str(s) => s, _ => return Value::None };
            let bp = match b { Value::Str(s) => s, _ => return Value::None };
            let p = Path::new(&ap).join(&bp);
            Value::Str(p.to_string_lossy().to_string())
        }

        pub fn exists(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::Bool(false) };
            Value::Bool(Path::new(&ps).exists())
        }

        pub fn isfile(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::Bool(false) };
            Value::Bool(Path::new(&ps).is_file())
        }

        pub fn isdir(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::Bool(false) };
            Value::Bool(Path::new(&ps).is_dir())
        }

        pub fn basename(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::None };
            let p = Path::new(&ps);
            Value::Str(p.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default())
        }

        pub fn dirname(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::None };
            let p = Path::new(&ps);
            Value::Str(p.parent().map(|s| s.to_string_lossy().to_string()).unwrap_or_default())
        }

        pub fn splitext(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::None };
            let p = Path::new(&ps);
            let stem = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            let ext = p.extension().map(|s| format!(".{}", s.to_string_lossy())).unwrap_or_default();
            Value::Tuple(vec![Value::Str(stem + &ext), Value::Str(ext)])
        }

        pub fn abspath(p: Value) -> Value {
            let ps = match p { Value::Str(s) => s, _ => return Value::None };
            let p = Path::new(&ps);
            Value::Str(std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf()).to_string_lossy().to_string())
        }
    }

    pub fn getcwd() -> Value {
        Value::Str(env::current_dir().unwrap_or_default().to_string_lossy().to_string())
    }

    pub fn chdir(p: Value) -> Value {
        let ps = match p { Value::Str(s) => s, _ => return Value::None };
        let _ = env::set_current_dir(ps);
        Value::None
    }

    pub fn listdir(p: Value) -> Value {
        let ps = match p { Value::Str(s) => s, _ => return Value::None };
        match std::fs::read_dir(ps) {
            Ok(entries) => {
                let mut items = Vec::new();
                for entry in entries.flatten() {
                    items.push(Value::Str(entry.file_name().to_string_lossy().to_string()));
                }
                Value::List(items)
            }
            Err(_) => Value::List(vec![]),
        }
    }

    pub fn environ() -> Value {
        let mut map = std::collections::BTreeMap::new();
        for (k, v) in env::vars() {
            map.insert(k, Value::Str(v));
        }
        Value::Dict(map)
    }

    pub fn sep() -> Value {
        Value::Str(std::path::MAIN_SEPARATOR.to_string())
    }

    pub fn linesep() -> Value {
        if cfg!(windows) { Value::Str("\r\n".to_string()) } else { Value::Str("\n".to_string()) }
    }
}

pub mod json {
    use super::Value;
    use std::collections::BTreeMap;

    pub fn dumps(obj: Value) -> Value {
        Value::Str(json_value_to_string(&obj))
    }

    pub fn loads(s: Value) -> Value {
        let ss = match s { Value::Str(s) => s, _ => return Value::None };
        match json_parse(ss.trim()) {
            Some((val, _)) => val,
            None => Value::None,
        }
    }

    fn json_value_to_string(v: &Value) -> String {
        match v {
            Value::None => "null".to_string(),
            Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            Value::Int(i) => i.to_string(),
            Value::Float(f) => {
                if f.is_nan() || f.is_infinite() {
                    "null".to_string()
                } else {
                    f.to_string()
                }
            }
            Value::Str(s) => {
                let mut escaped = String::new();
                for c in s.chars() {
                    match c {
                        '"' => escaped.push_str("\\\""),
                        '\\' => escaped.push_str("\\\\"),
                        '\n' => escaped.push_str("\\n"),
                        '\r' => escaped.push_str("\\r"),
                        '\t' => escaped.push_str("\\t"),
                        c if (c as u32) < 0x20 => escaped.push_str(&format!("\\u{:04x}", c as u32)),
                        c => escaped.push(c),
                    }
                }
                format!("\"{}\"", escaped)
            }
            Value::List(items) => {
                let inner: Vec<String> = items.iter().map(|i| json_value_to_string(i)).collect();
                format!("[{}]", inner.join(","))
            }
            Value::Tuple(items) => {
                let inner: Vec<String> = items.iter().map(|i| json_value_to_string(i)).collect();
                format!("[{}]", inner.join(","))
            }
            Value::Dict(map) => {
                let inner: Vec<String> = map.iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, json_value_to_string(v)))
                    .collect();
                format!("{{{}}}", inner.join(","))
            }
            _ => "null".to_string(),
        }
    }

    fn json_parse(s: &str) -> Option<(Value, &str)> {
        let s = s.trim_start();
        if s.is_empty() { return None }
        match s.as_bytes()[0] {
            b'{' => parse_object(s),
            b'[' => parse_array(s),
            b'"' => parse_string(s),
            b't' | b'f' => parse_bool(s),
            b'n' => parse_null(s),
            b'-' | b'0'..=b'9' | b'.' => parse_number(s),
            _ => None,
        }
    }

    fn parse_object(s: &str) -> Option<(Value, &str)> {
        let s = s.strip_prefix('{')?.trim_start();
        let mut map = BTreeMap::new();
        if s.starts_with('}') {
            return Some((Value::Dict(map), &s[1..]));
        }
        let mut rest = s;
        loop {
            let (key, r) = parse_string(rest)?;
            let r = r.trim_start();
            let r = r.strip_prefix(':')?;
            let r = r.trim_start();
            let (val, r) = json_parse(r)?;
            map.insert(match key { Value::Str(s) => s, _ => String::new() }, val);
            let r = r.trim_start();
            if let Some(r) = r.strip_prefix(',') {
                rest = r.trim_start();
                continue;
            }
            rest = r.strip_prefix('}')?;
            break;
        }
        Some((Value::Dict(map), rest))
    }

    fn parse_array(s: &str) -> Option<(Value, &str)> {
        let s = s.strip_prefix('[')?.trim_start();
        let mut items = Vec::new();
        if s.starts_with(']') {
            return Some((Value::List(items), &s[1..]));
        }
        let mut rest = s;
        loop {
            let (val, r) = json_parse(rest)?;
            items.push(val);
            let r = r.trim_start();
            if let Some(r) = r.strip_prefix(',') {
                rest = r.trim_start();
                continue;
            }
            rest = r.strip_prefix(']')?;
            break;
        }
        Some((Value::List(items), rest))
    }

    fn parse_string(s: &str) -> Option<(Value, &str)> {
        let s = s.strip_prefix('"')?;
        let mut chars = s.char_indices();
        let mut result = String::new();
        while let Some((_, c)) = chars.next() {
            match c {
                '"' => {
                    let remaining = &s[result.len() + 1..];
                    return Some((Value::Str(result), remaining));
                }
                '\\' => {
                    match chars.next() {
                        Some((_, '"')) => result.push('"'),
                        Some((_, '\\')) => result.push('\\'),
                        Some((_, '/')) => result.push('/'),
                        Some((_, 'n')) => result.push('\n'),
                        Some((_, 'r')) => result.push('\r'),
                        Some((_, 't')) => result.push('\t'),
                        Some((_, 'u')) => {
                            let hex: String = chars.by_ref().take(4).map(|(_, c)| c).collect();
                            if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                if let Some(c) = char::from_u32(code) {
                                    result.push(c);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => result.push(c),
            }
        }
        None
    }

    fn parse_bool(s: &str) -> Option<(Value, &str)> {
        if let Some(r) = s.strip_prefix("true") {
            Some((Value::Bool(true), r))
        } else if let Some(r) = s.strip_prefix("false") {
            Some((Value::Bool(false), r))
        } else {
            None
        }
    }

    fn parse_null(s: &str) -> Option<(Value, &str)> {
        let r = s.strip_prefix("null")?;
        Some((Value::None, r))
    }

    fn parse_number(s: &str) -> Option<(Value, &str)> {
        let end = s.find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-' && c != '+' && c != 'e' && c != 'E').unwrap_or(s.len());
        let num_str = &s[..end];
        if num_str.contains('.') || num_str.contains('e') || num_str.contains('E') {
            num_str.parse::<f64>().ok().map(|f| (Value::Float(f), &s[end..]))
        } else {
            num_str.parse::<i64>().ok().map(|i| (Value::Int(i), &s[end..]))
        }
    }
}

pub mod image {
    use super::Value;

    pub fn open(path: Value) -> Value {
        let p = match &path { Value::Str(s) => s.clone(), _ => return Value::Tuple(vec![Value::Int(0), Value::Int(0), Value::Int(0), Value::Bytes(vec![])]) };
        crate::wrappers::image::load(&p)
    }

    pub fn save(path: Value, img: Value) -> Value {
        let p = match &path { Value::Str(s) => s.clone(), _ => return Value::None };
        let (w, h, ch, data) = match &img {
            Value::Tuple(items) if items.len() >= 4 => {
                let w = match &items[0] { Value::Int(i) => *i, _ => 0 };
                let h = match &items[1] { Value::Int(i) => *i, _ => 0 };
                let ch = match &items[2] { Value::Int(i) => *i, _ => 0 };
                let data = match &items[3] { Value::Bytes(b) => b.clone(), _ => vec![] };
                (w, h, ch, data)
            }
            _ => return Value::None,
        };
        crate::wrappers::image::save(&p, w, h, ch, &data)
    }

    pub fn resize(img: Value, dst_w: Value, dst_h: Value) -> Value {
        let (w, h, ch, data) = match &img {
            Value::Tuple(items) if items.len() >= 4 => {
                let w = match &items[0] { Value::Int(i) => *i, _ => 0 };
                let h = match &items[1] { Value::Int(i) => *i, _ => 0 };
                let ch = match &items[2] { Value::Int(i) => *i, _ => 0 };
                let data = match &items[3] { Value::Bytes(b) => b.clone(), _ => vec![] };
                (w, h, ch, data)
            }
            _ => return Value::Bytes(vec![]),
        };
        let dw = match dst_w { Value::Int(i) => i, _ => w };
        let dh = match dst_h { Value::Int(i) => i, _ => h };
        crate::wrappers::image::resize(&data, w, h, ch, dw, dh)
    }
}

pub mod opengl {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::opengl::init()
    }

    pub fn create_window(title: Value, width: Value, height: Value) -> Value {
        let t = match &title { Value::Str(s) => s.clone(), _ => String::new() };
        let w = match width { Value::Int(i) => i as i32, _ => 640 };
        let h = match height { Value::Int(i) => i as i32, _ => 480 };
        let c = std::ffi::CString::new(t.clone()).unwrap_or_default();
        crate::wrappers::opengl::construct("Window", vec![Value::Int(w as i64), Value::Int(h as i64), Value::Str(t)]);
        let _ = c;
        Value::None
    }

    pub fn make_context_current(window: &Value) -> Value {
        crate::wrappers::opengl::method(window, "make_context_current", vec![])
    }

    pub fn swap_buffers(window: &Value) -> Value {
        crate::wrappers::opengl::method(window, "swap_buffers", vec![])
    }

    pub fn poll_events() -> Value {
        crate::wrappers::opengl::poll_events()
    }

    pub fn window_should_close(window: &Value) -> Value {
        crate::wrappers::opengl::method(window, "should_close", vec![])
    }

    pub fn destroy_window(window: &Value) -> Value {
        crate::wrappers::opengl::method(window, "destroy", vec![])
    }

    pub fn terminate() -> Value {
        crate::wrappers::opengl::terminate()
    }

    pub fn get_time() -> Value {
        crate::wrappers::opengl::get_time()
    }

    pub fn clear_color(r: Value, g: Value, b: Value, a: Value) -> Value {
        let rv = match r { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let gv = match g { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let bv = match b { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let av = match a { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::clear_color(rv, gv, bv, av)
    }

    pub fn clear(mask: Value) -> Value {
        let m = match mask { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::clear(m)
    }

    pub fn viewport(x: Value, y: Value, w: Value, h: Value) -> Value {
        let xv = match x { Value::Int(i) => i as i32, _ => 0 };
        let yv = match y { Value::Int(i) => i as i32, _ => 0 };
        let wv = match w { Value::Int(i) => i as i32, _ => 0 };
        let hv = match h { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::viewport(xv, yv, wv, hv)
    }

    pub fn begin(mode: Value) -> Value {
        let m = match mode { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::begin(m)
    }

    pub fn end_() -> Value {
        crate::wrappers::opengl::end_()
    }

    pub fn vertex2f(x: Value, y: Value) -> Value {
        let xv = match x { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let yv = match y { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::vertex2f(xv, yv)
    }

    pub fn vertex3f(x: Value, y: Value, z: Value) -> Value {
        let xv = match x { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let yv = match y { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let zv = match z { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::vertex3f(xv, yv, zv)
    }

    pub fn color3f(r: Value, g: Value, b: Value) -> Value {
        let rv = match r { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let gv = match g { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let bv = match b { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::color3f(rv, gv, bv)
    }

    pub fn color4f(r: Value, g: Value, b: Value, a: Value) -> Value {
        let rv = match r { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let gv = match g { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let bv = match b { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let av = match a { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::color4f(rv, gv, bv, av)
    }

    pub fn load_identity() -> Value {
        crate::wrappers::opengl::load_identity()
    }

    pub fn translatef(x: Value, y: Value, z: Value) -> Value {
        let xv = match x { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let yv = match y { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let zv = match z { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::translatef(xv, yv, zv)
    }

    pub fn rotatef(angle: Value, x: Value, y: Value, z: Value) -> Value {
        let av = match angle { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let xv = match x { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let yv = match y { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let zv = match z { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::rotatef(av, xv, yv, zv)
    }

    pub fn ortho(left: Value, right: Value, bottom: Value, top: Value, near_val: Value, far_val: Value) -> Value {
        let lv = match left { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let rv = match right { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let bv = match bottom { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let tv = match top { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let nv = match near_val { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        let fv = match far_val { Value::Float(f) => f as f32, Value::Int(i) => i as f32, _ => 0.0 };
        crate::wrappers::opengl::ortho(lv, rv, bv, tv, nv, fv)
    }

    pub fn matrix_mode(mode: Value) -> Value {
        let m = match mode { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::matrix_mode(m)
    }

    pub fn enable(cap: Value) -> Value {
        let c = match cap { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::enable(c)
    }

    pub fn disable(cap: Value) -> Value {
        let c = match cap { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::disable(c)
    }

    pub fn flush() -> Value {
        crate::wrappers::opengl::flush()
    }

    pub fn get_error() -> Value {
        crate::wrappers::opengl::get_error()
    }

    pub fn create_shader(shader_type: Value) -> Value {
        let t = match shader_type { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::opengl::create_shader(t)
    }

    pub fn shader_source(shader: Value, source: Value) -> Value {
        let s = match shader { Value::Int(i) => i as u32, _ => 0 };
        let src = match &source { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::opengl::shader_source(s, &src)
    }

    pub fn compile_shader(shader: Value) -> Value {
        let s = match shader { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::opengl::compile_shader(s)
    }

    pub fn create_program() -> Value {
        crate::wrappers::opengl::create_program()
    }

    pub fn attach_shader(program: Value, shader: Value) -> Value {
        let p = match program { Value::Int(i) => i as u32, _ => 0 };
        let s = match shader { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::opengl::attach_shader(p, s)
    }

    pub fn link_program(program: Value) -> Value {
        let p = match program { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::opengl::link_program(p)
    }

    pub fn use_program(program: Value) -> Value {
        let p = match program { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::opengl::use_program(p)
    }
}

pub mod sdl2 {
    use super::Value;

    pub fn init(flags: Value) -> Value {
        let f = match flags { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::sdl2::init(f)
    }

    pub fn create_window(title: Value, x: Value, y: Value, w: Value, h: Value, flags: Value) -> Value {
        let t = match &title { Value::Str(s) => s.clone(), _ => String::new() };
        let xv = match x { Value::Int(i) => i as i32, _ => 0 };
        let yv = match y { Value::Int(i) => i as i32, _ => 0 };
        let wv = match w { Value::Int(i) => i as i32, _ => 640 };
        let hv = match h { Value::Int(i) => i as i32, _ => 480 };
        let fv = match flags { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::sdl2::create_window(&t, xv, yv, wv, hv, fv)
    }

    pub fn destroy_window(win: &Value) -> Value {
        crate::wrappers::sdl2::destroy_window(win);
        Value::None
    }

    pub fn create_renderer(window: &Value, index: Value, flags: Value) -> Value {
        let idx = match index { Value::Int(i) => i as i32, _ => -1 };
        let fv = match flags { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::sdl2::create_renderer(window, idx, fv)
    }

    pub fn destroy_renderer(renderer: &Value) -> Value {
        crate::wrappers::sdl2::destroy_renderer(renderer);
        Value::None
    }

    pub fn render_clear(renderer: &Value) -> Value {
        crate::wrappers::sdl2::render_clear(renderer);
        Value::None
    }

    pub fn render_present(renderer: &Value) -> Value {
        crate::wrappers::sdl2::render_present(renderer);
        Value::None
    }

    pub fn set_render_draw_color(renderer: &Value, r: Value, g: Value, b: Value, a: Value) -> Value {
        let rv = match r { Value::Int(i) => i as u8, _ => 0 };
        let gv = match g { Value::Int(i) => i as u8, _ => 0 };
        let bv = match b { Value::Int(i) => i as u8, _ => 0 };
        let av = match a { Value::Int(i) => i as u8, _ => 255 };
        crate::wrappers::sdl2::set_render_draw_color(renderer, rv, gv, bv, av);
        Value::None
    }

    pub fn render_fill_rect(renderer: &Value, x: Value, y: Value, w: Value, h: Value) -> Value {
        let xv = match x { Value::Int(i) => i as i32, _ => 0 };
        let yv = match y { Value::Int(i) => i as i32, _ => 0 };
        let wv = match w { Value::Int(i) => i as i32, _ => 10 };
        let hv = match h { Value::Int(i) => i as i32, _ => 10 };
        crate::wrappers::sdl2::render_fill_rect(renderer, xv, yv, wv, hv);
        Value::None
    }

    pub fn poll_event() -> Value {
        crate::wrappers::sdl2::poll_event()
    }

    pub fn delay(ms: Value) -> Value {
        let m = match ms { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::sdl2::delay(m);
        Value::None
    }

    pub fn get_ticks() -> Value {
        crate::wrappers::sdl2::get_ticks()
    }

    pub fn create_texture_from_surface(renderer: &Value, surface: &Value) -> Value {
        crate::wrappers::sdl2::create_texture_from_surface(renderer, surface)
    }

    pub fn render_copy(renderer: &Value, texture: &Value, sx: Value, sy: Value, sw: Value, sh: Value, dx: Value, dy: Value, dw: Value, dh: Value) -> Value {
        let sxv = match sx { Value::Int(i) => i as i32, _ => 0 };
        let syv = match sy { Value::Int(i) => i as i32, _ => 0 };
        let swv = match sw { Value::Int(i) => i as i32, _ => 0 };
        let shv = match sh { Value::Int(i) => i as i32, _ => 0 };
        let dxv = match dx { Value::Int(i) => i as i32, _ => 0 };
        let dyv = match dy { Value::Int(i) => i as i32, _ => 0 };
        let dwv = match dw { Value::Int(i) => i as i32, _ => 0 };
        let dhv = match dh { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::sdl2::render_copy(renderer, texture, sxv, syv, swv, shv, dxv, dyv, dwv, dhv);
        Value::None
    }

    pub fn destroy_texture(texture: &Value) -> Value {
        crate::wrappers::sdl2::destroy_texture(texture);
        Value::None
    }

    pub fn image_load(file: Value) -> Value {
        let f = match &file { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::sdl2::image_load(&f)
    }

    pub fn free_surface(surface: &Value) -> Value {
        crate::wrappers::sdl2::free_surface(surface);
        Value::None
    }

    pub fn get_keyboard_state() -> Value {
        crate::wrappers::sdl2::get_keyboard_state()
    }

    pub fn ttf_init() -> Value {
        crate::wrappers::sdl2::ttf_init()
    }

    pub fn ttf_quit() -> Value {
        crate::wrappers::sdl2::ttf_quit();
        Value::None
    }

    pub fn ttf_open_font(path: Value, size: Value) -> Value {
        let p = match &path { Value::Str(s) => s.clone(), _ => String::new() };
        let sz = match size { Value::Int(i) => i as i32, _ => 16 };
        crate::wrappers::sdl2::ttf_open_font(&p, sz)
    }

    pub fn ttf_render_text_solid(font: &Value, text: Value, r: Value, g: Value, b: Value) -> Value {
        let t = match &text { Value::Str(s) => s.clone(), _ => String::new() };
        let rv = match r { Value::Int(i) => i as u8, _ => 255 };
        let gv = match g { Value::Int(i) => i as u8, _ => 255 };
        let bv = match b { Value::Int(i) => i as u8, _ => 255 };
        crate::wrappers::sdl2::ttf_render_text_solid(font, &t, rv, gv, bv)
    }

    pub fn mixer_init(freq: Value, format: Value, channels: Value, chunksize: Value) -> Value {
        let f = match freq { Value::Int(i) => i as i32, _ => 44100 };
        let fmt = match format { Value::Int(i) => i as u16, _ => 0x8010 };
        let ch = match channels { Value::Int(i) => i as i32, _ => 2 };
        let cs = match chunksize { Value::Int(i) => i as i32, _ => 4096 };
        crate::wrappers::sdl2::mixer_init(f, fmt, ch, cs)
    }

    pub fn mixer_load_music(file: Value) -> Value {
        let f = match &file { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::sdl2::mixer_load_music(&f)
    }

    pub fn mixer_play_music(music: &Value, loops: Value) -> Value {
        let l = match loops { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::sdl2::mixer_play_music(music, l);
        Value::None
    }

    pub fn mixer_load_chunk(file: Value) -> Value {
        let f = match &file { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::sdl2::mixer_load_chunk(&f)
    }

    pub fn mixer_play_channel(channel: Value, chunk: &Value, loops: Value) -> Value {
        let ch = match channel { Value::Int(i) => i as i32, _ => -1 };
        let l = match loops { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::sdl2::mixer_play_channel(ch, chunk, l)
    }

    pub fn quit() -> Value {
        crate::wrappers::sdl2::quit();
        Value::None
    }
}

pub mod time {
    use super::Value;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn time() -> Value {
        Value::Float(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64())
    }

    pub fn sleep(secs: Value) -> Value {
        let s = match secs {
            Value::Float(f) => f,
            Value::Int(i) => i as f64,
            _ => 0.0,
        };
        std::thread::sleep(std::time::Duration::from_secs_f64(s));
        Value::None
    }
}

pub mod random {
    use super::Value;

    pub fn random() -> Value {
        Value::Float(rand())
    }

    pub fn randint(a: Value, b: Value) -> Value {
        let lo = match &a { Value::Int(i) => *i, _ => 0 };
        let hi = match &b { Value::Int(i) => *i, _ => 100 };
        Value::Int(lo + (rand() * (hi - lo + 1) as f64) as i64)
    }

    pub fn randrange(start: Value, stop: Option<Value>, step: Option<Value>) -> Value {
        let lo = match &start { Value::Int(i) => *i, _ => 0 };
        let hi = match &stop { Some(Value::Int(i)) => *i, None => lo + 1, _ => 100 };
        let st = match &step { Some(Value::Int(i)) => *i, _ => 1 };
        let count = (hi - lo + st - 1) / st;
        if count <= 0 { return Value::Int(lo); }
        Value::Int(lo + st * (rand() * count as f64) as i64)
    }

    fn rand() -> f64 {
        let mut seed: u64 = unsafe { std::ptr::read_volatile(&0u64 as *const u64) };
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut x = seed;
        x ^= x >> 30;
        x = x.wrapping_mul(0xbf58476d1ce4e5b9);
        x ^= x >> 27;
        x = x.wrapping_mul(0x94d049bb133111eb);
        x ^= x >> 31;
        x as f64 / u64::MAX as f64
    }
}

pub mod sqlite3 {
    use super::Value;

    pub fn connect(database: Value) -> Value {
        let path = match &database { Value::Str(s) => s.clone(), _ => ":memory:".to_string() };
        crate::wrappers::sqlite3::connect(&path)
    }

    pub fn execute(conn: &Value, sql: Value, _params: Option<Vec<Value>>) -> Value {
        let sql_str = match &sql { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::sqlite3::execute(conn, &sql_str)
    }

    pub fn fetchall(cursor: &Value) -> Value {
        crate::wrappers::sqlite3::fetch_all(cursor)
    }

    pub fn commit(conn: &Value) -> Value {
        crate::wrappers::sqlite3::execute(conn, "COMMIT");
        Value::None
    }

    pub fn close(conn: &Value) -> Value {
        crate::wrappers::sqlite3::close(conn);
        Value::None
    }
}

pub mod git {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::git::init()
    }

    pub fn shutdown() -> Value {
        crate::wrappers::git::shutdown()
    }

    pub fn clone_(url: Value, path: Value) -> Value {
        let u = match &url { Value::Str(s) => s.clone(), _ => return Value::Int(-1) };
        let p = match &path { Value::Str(s) => s.clone(), _ => return Value::Int(-1) };
        crate::wrappers::git::clone_(&u, &p)
    }

    pub fn open(path: Value) -> Value {
        let p = match &path { Value::Str(s) => s.clone(), _ => return Value::None };
        crate::wrappers::git::open(&p)
    }

    pub fn free(repo: &Value) -> Value {
        crate::wrappers::git::free(repo);
        Value::None
    }

    pub fn commit_id(repo: &Value, branch: Value) -> Value {
        let b = match &branch { Value::Str(s) => Some(s.clone()), _ => None };
        crate::wrappers::git::commit_id(repo, b.as_deref())
    }

    pub fn branch_list(repo: &Value) -> Value {
        crate::wrappers::git::branch_list(repo)
    }

    pub fn status(repo: &Value, filepath: Value) -> Value {
        let p = match &filepath { Value::Str(s) => s.clone(), _ => return Value::Int(-1) };
        crate::wrappers::git::status(repo, &p)
    }

    pub fn add(repo: &Value, filepath: Value) -> Value {
        let p = match &filepath { Value::Str(s) => s.clone(), _ => return Value::Int(-1) };
        crate::wrappers::git::add(repo, &p)
    }

    pub fn commit(repo: &Value, message: Value, name: Value, email: Value) -> Value {
        let msg = match &message { Value::Str(s) => s.clone(), _ => String::new() };
        let nm = match &name { Value::Str(s) => s.clone(), _ => String::new() };
        let em = match &email { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::git::commit(repo, &msg, &nm, &em)
    }

    pub fn push(repo: &Value, remote_name: Value, refspec: Value) -> Value {
        let rn = match &remote_name { Value::Str(s) => s.clone(), _ => String::new() };
        let rs = match &refspec { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::git::push(repo, &rn, &rs)
    }

    pub fn pull(repo: &Value, remote_name: Value, merge_branch: Value) -> Value {
        let rn = match &remote_name { Value::Str(s) => s.clone(), _ => String::new() };
        let mb = match &merge_branch { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::git::pull(repo, &rn, &mb)
    }

    pub fn diff_stats(repo: &Value) -> Value {
        crate::wrappers::git::diff_stats(repo)
    }

    pub fn log(repo: &Value, max_count: Value) -> Value {
        let mc = match max_count { Value::Int(i) => i, _ => 10 };
        crate::wrappers::git::log(repo, mc)
    }
}

pub mod gi {
    use super::Value;

    pub mod repository {
        use super::super::Value;

        pub mod Gtk {
            use super::super::super::Value;

            pub fn Window() -> Value {
                crate::wrappers::gtk4::construct("GtkWindow", vec![])
            }

            pub fn Button() -> Value {
                crate::wrappers::gtk4::construct("GtkButton", vec![])
            }

            pub fn Label() -> Value {
                crate::wrappers::gtk4::construct("GtkLabel", vec![])
            }

            pub fn Entry() -> Value {
                crate::wrappers::gtk4::construct("GtkEntry", vec![])
            }

            pub fn Box_() -> Value {
                crate::wrappers::gtk4::construct("GtkBox", vec![])
            }

            pub fn ScrolledWindow() -> Value {
                crate::wrappers::gtk4::construct("GtkScrolledWindow", vec![])
            }

            pub fn TextView() -> Value {
                crate::wrappers::gtk4::construct("GtkTextView", vec![])
            }

            pub fn HeaderBar() -> Value {
                crate::wrappers::gtk4::construct("GtkHeaderBar", vec![])
            }

            pub fn Application() -> Value {
                crate::wrappers::gtk4::construct("GtkApplication", vec![])
            }

            pub mod Orientation {
                use super::super::super::super::Value;
                pub fn VERTICAL() -> Value { Value::Int(1) }
                pub fn HORIZONTAL() -> Value { Value::Int(0) }
            }

            pub fn main() -> Value {
                Value::None
            }

            pub fn main_quit() -> Value {
                Value::None
            }
        }
    }
}

pub mod ffmpeg {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::ffmpeg::init()
    }

    pub fn open(path: Value) -> Value {
        let p = match &path { Value::Str(s) => s.clone(), _ => return Value::None };
        crate::wrappers::ffmpeg::open(&p)
    }

    pub fn close(ctx: &Value) -> Value {
        crate::wrappers::ffmpeg::close(ctx);
        Value::None
    }

    pub fn find_stream(ctx: &Value, type_: Value) -> Value {
        let t = match type_ { Value::Int(i) => i, _ => -1 };
        crate::wrappers::ffmpeg::find_stream(ctx, t)
    }

    pub fn get_codec_params(ctx: &Value, stream_idx: Value) -> Value {
        let si = match stream_idx { Value::Int(i) => i, _ => -1 };
        crate::wrappers::ffmpeg::get_codec_params(ctx, si)
    }

    pub fn read_frame(ctx: &Value) -> Value {
        crate::wrappers::ffmpeg::read_frame(ctx)
    }

    pub fn seek(ctx: &Value, timestamp: Value) -> Value {
        let ts = match timestamp { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        crate::wrappers::ffmpeg::seek(ctx, ts)
    }

    pub fn get_duration(ctx: &Value) -> Value {
        crate::wrappers::ffmpeg::get_duration(ctx)
    }

    pub fn extract_thumbnail(ctx: &Value, time_sec: Value) -> Value {
        let ts = match time_sec { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        crate::wrappers::ffmpeg::extract_thumbnail(ctx, ts)
    }

    pub fn get_metadata(ctx: &Value, key: Value) -> Value {
        let k = match &key { Value::Str(s) => s.clone(), _ => return Value::None };
        crate::wrappers::ffmpeg::get_metadata(ctx, &k)
    }

    pub fn get_all_metadata(ctx: &Value) -> Value {
        crate::wrappers::ffmpeg::get_all_metadata(ctx)
    }
}

pub mod vulkan {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::vulkan::init()
    }

    pub fn create_instance(app_name: Value, engine_name: Value) -> Value {
        let an = match &app_name { Value::Str(s) => s.clone(), _ => String::new() };
        let en = match &engine_name { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::vulkan::create_instance(&an, &en)
    }

    pub fn destroy_instance(instance: &Value) -> Value {
        crate::wrappers::vulkan::destroy_instance(instance);
        Value::None
    }

    pub fn enumerate_physical_devices(instance: &Value) -> Value {
        crate::wrappers::vulkan::enumerate_physical_devices(instance)
    }

    pub fn get_device_properties(device: &Value) -> Value {
        crate::wrappers::vulkan::get_device_properties(device)
    }

    pub fn create_device(device: &Value, queue_family_index: Value) -> Value {
        let qfi = match queue_family_index { Value::Int(i) => i, _ => 0 };
        crate::wrappers::vulkan::create_device(device, qfi)
    }

    pub fn destroy_device(device: &Value) -> Value {
        crate::wrappers::vulkan::destroy_device(device);
        Value::None
    }

    pub fn create_swapchain(device: &Value, surface: &Value, width: Value, height: Value, format: Value) -> Value {
        let w = match width { Value::Int(i) => i, _ => 800 };
        let h = match height { Value::Int(i) => i, _ => 600 };
        let f = match format { Value::Int(i) => i, _ => 44 };
        crate::wrappers::vulkan::create_swapchain(device, surface, w, h, f)
    }

    pub fn destroy_swapchain(device: &Value, swapchain: &Value) -> Value {
        crate::wrappers::vulkan::destroy_swapchain(device, swapchain);
        Value::None
    }

    pub fn create_shader_module(device: &Value, code: Value) -> Value {
        let c = match &code { Value::Bytes(b) => b.clone(), _ => vec![] };
        crate::wrappers::vulkan::create_shader_module(device, &c)
    }

    pub fn destroy_shader_module(device: &Value, shader: &Value) -> Value {
        crate::wrappers::vulkan::destroy_shader_module(device, shader);
        Value::None
    }

    pub fn create_pipeline(device: &Value, vert_shader: &Value, frag_shader: &Value, width: Value, height: Value) -> Value {
        let w = match width { Value::Int(i) => i, _ => 800 };
        let h = match height { Value::Int(i) => i, _ => 600 };
        crate::wrappers::vulkan::create_pipeline(device, vert_shader, frag_shader, w, h)
    }

    pub fn destroy_pipeline(device: &Value, pipeline: &Value) -> Value {
        crate::wrappers::vulkan::destroy_pipeline(device, pipeline);
        Value::None
    }

    pub fn create_command_buffer(device: &Value) -> Value {
        crate::wrappers::vulkan::create_command_buffer(device)
    }

    pub fn begin_command_buffer(cmd: &Value) -> Value {
        crate::wrappers::vulkan::begin_command_buffer(cmd);
        Value::None
    }

    pub fn cmd_bind_pipeline(cmd: &Value, pipeline: &Value) -> Value {
        crate::wrappers::vulkan::cmd_bind_pipeline(cmd, pipeline);
        Value::None
    }

    pub fn cmd_draw(cmd: &Value, vertex_count: Value, instance_count: Value) -> Value {
        let vc = match vertex_count { Value::Int(i) => i, _ => 3 };
        let ic = match instance_count { Value::Int(i) => i, _ => 1 };
        crate::wrappers::vulkan::cmd_draw(cmd, vc, ic);
        Value::None
    }

    pub fn end_command_buffer(cmd: &Value) -> Value {
        crate::wrappers::vulkan::end_command_buffer(cmd);
        Value::None
    }

    pub fn queue_submit(device: &Value, cmd: &Value) -> Value {
        crate::wrappers::vulkan::queue_submit(device, cmd);
        Value::None
    }

    pub fn device_wait_idle(device: &Value) -> Value {
        crate::wrappers::vulkan::device_wait_idle(device);
        Value::None
    }

    pub fn get_physical_device_memory_properties(device: &Value) -> Value {
        crate::wrappers::vulkan::get_physical_device_memory_properties(device)
    }
}

pub mod font {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::font::ft_init()
    }

    pub fn load_face(path: Value, index: Value) -> Value {
        let p = match &path { Value::Str(s) => s.clone(), _ => return Value::None };
        let idx = match index { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::font::load_face(&p, idx)
    }

    pub fn done_face(face: &Value) -> Value {
        crate::wrappers::font::done_face(face)
    }

    pub fn set_size(face: &Value, size: Value, dpi: Value) -> Value {
        let sz = match size { Value::Int(i) => i as i32, _ => 12 };
        let dp = match dpi { Value::Int(i) => i as i32, _ => 72 };
        crate::wrappers::font::set_size(face, sz, dp)
    }

    pub fn get_glyph(face: &Value, charcode: Value) -> Value {
        let ch = match charcode { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::font::get_glyph(face, ch)
    }

    pub fn get_kerning(face: &Value, left: Value, right: Value) -> Value {
        let l = match left { Value::Int(i) => i as u32, _ => 0 };
        let r = match right { Value::Int(i) => i as u32, _ => 0 };
        crate::wrappers::font::get_kerning(face, l, r)
    }

    pub fn get_name(face: &Value) -> Value {
        crate::wrappers::font::get_name(face)
    }

    pub fn get_num_glyphs(face: &Value) -> Value {
        crate::wrappers::font::get_num_glyphs(face)
    }

    pub fn hb_create_font(face: &Value) -> Value {
        crate::wrappers::font::hb_create_font_face(face)
    }

    pub fn hb_destroy_font(font: &Value) -> Value {
        crate::wrappers::font::hb_destroy_font_face(font)
    }

    pub fn hb_buffer_create() -> Value {
        crate::wrappers::font::hb_buffer_create_()
    }

    pub fn hb_buffer_destroy(buf: &Value) -> Value {
        crate::wrappers::font::hb_buffer_destroy_(buf)
    }

    pub fn hb_buffer_add_utf8(buf: &Value, text: Value) -> Value {
        let t = match &text { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::font::hb_buffer_add_utf8_(buf, &t)
    }

    pub fn hb_buffer_set_script(buf: &Value, script: Value) -> Value {
        let s = match script { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::font::hb_buffer_set_script_(buf, s)
    }

    pub fn hb_buffer_set_language(buf: &Value, lang: Value) -> Value {
        let l = match &lang { Value::Str(s) => s.clone(), _ => String::new() };
        crate::wrappers::font::hb_buffer_set_language_(buf, &l)
    }

    pub fn hb_buffer_set_direction(buf: &Value, dir: Value) -> Value {
        let d = match dir { Value::Int(i) => i as i32, _ => 0 };
        crate::wrappers::font::hb_buffer_set_direction_(buf, d)
    }

    pub fn hb_shape(font: &Value, buf: &Value) -> Value {
        crate::wrappers::font::hb_shape_(font, buf)
    }

    pub fn hb_buffer_get_glyph_infos(buf: &Value) -> Value {
        crate::wrappers::font::hb_buffer_get_glyph_infos_(buf)
    }
}

pub mod torch {
    use super::Value;

    pub fn tensor(data: Value, dtype: Value) -> Value {
        let dt = match dtype { Value::Int(i) => i as i32, _ => 0 };
        match &data {
            Value::List(items) => {
                let flat = flatten_floats(items);
                let shape = vec![flat.0.len() as i64];
                crate::wrappers::torch::tensor(&flat.0, &shape, dt)
            }
            Value::Tuple(items) => {
                let flat = flatten_floats(items);
                let shape = vec![flat.0.len() as i64];
                crate::wrappers::torch::tensor(&flat.0, &shape, dt)
            }
            Value::Float(f) => {
                let data = vec![*f as f32];
                crate::wrappers::torch::tensor(&data, &[1i64], dt)
            }
            Value::Int(i) => {
                let data = vec![*i as f32];
                crate::wrappers::torch::tensor(&data, &[1i64], dt)
            }
            _ => Value::None,
        }
    }

    fn flatten_floats(items: &[Value]) -> (Vec<f32>, Vec<i64>) {
        let mut floats = Vec::new();
        flatten(items, &mut floats);
        let len = floats.len() as i64;
        (floats, vec![len])
    }

    fn flatten(items: &[Value], out: &mut Vec<f32>) {
        for item in items {
            match item {
                Value::Float(f) => out.push(*f as f32),
                Value::Int(i) => out.push(*i as f32),
                Value::List(sub) => flatten(sub, out),
                Value::Tuple(sub) => flatten(sub, out),
                _ => {}
            }
        }
    }

    pub fn zeros(shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::torch::zeros(&dims)
    }

    pub fn ones(shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::torch::ones(&dims)
    }

    pub fn rand(shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::torch::rand(&dims)
    }

    pub fn clone(tensor: &Value) -> Value {
        crate::wrappers::torch::clone(tensor)
    }

    pub fn add(a: &Value, b: &Value) -> Value {
        crate::wrappers::torch::add(a, b)
    }

    pub fn sub(a: &Value, b: &Value) -> Value {
        crate::wrappers::torch::sub(a, b)
    }

    pub fn mul(a: &Value, b: &Value) -> Value {
        crate::wrappers::torch::mul(a, b)
    }

    pub fn div(a: &Value, b: &Value) -> Value {
        crate::wrappers::torch::div(a, b)
    }

    pub fn matmul(a: &Value, b: &Value) -> Value {
        crate::wrappers::torch::matmul(a, b)
    }

    pub fn relu(tensor: &Value) -> Value {
        crate::wrappers::torch::relu(tensor)
    }

    pub fn sigmoid(tensor: &Value) -> Value {
        crate::wrappers::torch::sigmoid(tensor)
    }

    pub fn tanh(tensor: &Value) -> Value {
        crate::wrappers::torch::tanh(tensor)
    }

    pub fn softmax(tensor: &Value, dim: Value) -> Value {
        let d = match dim { Value::Int(i) => i as i32, _ => -1 };
        crate::wrappers::torch::softmax(tensor, d)
    }

    pub fn sum(tensor: &Value, dim: Value) -> Value {
        let d = match dim { Value::Int(i) => i as i32, _ => -1 };
        crate::wrappers::torch::sum(tensor, d)
    }

    pub fn mean(tensor: &Value, dim: Value) -> Value {
        let d = match dim { Value::Int(i) => i as i32, _ => -1 };
        crate::wrappers::torch::mean(tensor, d)
    }

    pub fn reshape(tensor: &Value, shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::torch::reshape(tensor, &dims)
    }

    pub fn view(tensor: &Value, shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::torch::view(tensor, &dims)
    }

    pub fn dim(tensor: &Value) -> Value {
        crate::wrappers::torch::dim(tensor)
    }

    pub fn sizes(tensor: &Value) -> Value {
        crate::wrappers::torch::sizes(tensor)
    }

    pub fn item(tensor: &Value) -> Value {
        crate::wrappers::torch::item(tensor)
    }

    pub fn to_string(tensor: &Value) -> Value {
        crate::wrappers::torch::to_string(tensor)
    }

    pub fn requires_grad(tensor: &Value, req: Value) -> Value {
        let r = match req { Value::Bool(b) => b, _ => false };
        crate::wrappers::torch::requires_grad(tensor, r);
        Value::None
    }

    pub fn backward(tensor: &Value) -> Value {
        crate::wrappers::torch::backward(tensor);
        Value::None
    }

    pub fn grad(tensor: &Value) -> Value {
        crate::wrappers::torch::grad(tensor)
    }

    pub fn linear(in_features: Value, out_features: Value, bias: Value) -> Value {
        let inf = match in_features { Value::Int(i) => i as i32, _ => 0 };
        let outf = match out_features { Value::Int(i) => i as i32, _ => 0 };
        let b = match bias { Value::Bool(b) => b, Value::Int(i) => i != 0, _ => true };
        crate::wrappers::torch::linear(inf, outf, b)
    }

    pub fn linear_forward(module: &Value, input: &Value) -> Value {
        crate::wrappers::torch::linear_forward(module, input)
    }
}

pub mod urllib {
    use super::Value;

    pub fn request(url: Value, method: Value, data: Value, headers: Value) -> Value {
        let url_str = match &url { Value::Str(s) => s.clone(), _ => return Value::Tuple(vec![Value::Int(-1), Value::Str("Invalid URL".to_string())]) };
        let method_str = match &method { Value::Str(s) => s.clone(), _ => "GET".to_string() };
        let data_str = match &data { Value::Str(s) => Some(s.clone()), _ => None };
        let hdrs = match &headers {
            Value::List(items) => Some(items.iter().filter_map(|v| match v {
                Value::Tuple(items) if items.len() == 2 => {
                    let k = match &items[0] { Value::Str(s) => s.clone(), _ => return None };
                    let v = match &items[1] { Value::Str(s) => s.clone(), _ => return None };
                    Some((k, v))
                }
                _ => None
            }).collect::<Vec<(String,String)>>()),
            _ => None
        };
        crate::wrappers::curl::request(&url_str, &method_str, data_str.as_deref(), hdrs)
    }

    pub fn urlopen(url: Value) -> Value {
        let url_str = match &url { Value::Str(s) => s.clone(), _ => return Value::Tuple(vec![Value::Int(-1), Value::Str("Invalid URL".to_string())]) };
        crate::wrappers::curl::get(&url_str)
    }
}

pub mod lvgl {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::lvgl::init()
    }

    pub fn create_display(width: Value, height: Value, buf1: Value, buf2: Value) -> Value {
        let w = match width { Value::Int(i) => i, _ => 320 };
        let h = match height { Value::Int(i) => i, _ => 240 };
        let b1 = match &buf1 { Value::Ptr(p) => *p, _ => std::ptr::null_mut() };
        let b2 = match &buf2 { Value::Ptr(p) => *p, _ => std::ptr::null_mut() };
        crate::wrappers::lvgl::create_display(w, h, b1, b2)
    }

    pub fn tick_inc(ms: Value) -> Value {
        let m = match ms { Value::Int(i) => i, _ => 0 };
        crate::wrappers::lvgl::tick_inc(m);
        Value::None
    }

    pub fn task_handler() -> Value {
        crate::wrappers::lvgl::task_handler();
        Value::None
    }

    pub fn scr_act() -> Value {
        crate::wrappers::lvgl::scr_act()
    }

    pub fn disp_get_default() -> Value {
        crate::wrappers::lvgl::disp_get_default()
    }

    pub fn Obj(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Obj", vec![parent])
    }

    pub fn Btn(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Btn", vec![parent])
    }

    pub fn Label(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Label", vec![parent])
    }

    pub fn Slider(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Slider", vec![parent])
    }

    pub fn Arc(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Arc", vec![parent])
    }

    pub fn Bar(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Bar", vec![parent])
    }

    pub fn Dropdown(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Dropdown", vec![parent])
    }

    pub fn TextArea(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("TextArea", vec![parent])
    }

    pub fn Checkbox(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Checkbox", vec![parent])
    }

    pub fn Switch(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Switch", vec![parent])
    }

    pub fn Chart(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Chart", vec![parent])
    }

    pub fn Image(parent: Value) -> Value {
        crate::wrappers::lvgl::construct("Image", vec![parent])
    }

    pub fn set_pos(obj: &Value, x: Value, y: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_pos", vec![x, y])
    }

    pub fn set_size(obj: &Value, w: Value, h: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_size", vec![w, h])
    }

    pub fn set_align(obj: &Value, align: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_align", vec![align])
    }

    pub fn center(obj: &Value) -> Value {
        crate::wrappers::lvgl::method(obj, "center", vec![])
    }

    pub fn add_flag(obj: &Value, flag: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "add_flag", vec![flag])
    }

    pub fn clear_flag(obj: &Value, flag: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "clear_flag", vec![flag])
    }

    pub fn set_text(obj: &Value, text: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_text", vec![text])
    }

    pub fn set_value(obj: &Value, value: Value, anim: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_value", vec![value, anim])
    }

    pub fn get_value(obj: &Value) -> Value {
        crate::wrappers::lvgl::method(obj, "get_value", vec![])
    }

    pub fn get_text(obj: &Value) -> Value {
        crate::wrappers::lvgl::method(obj, "get_text", vec![])
    }

    pub fn set_options(obj: &Value, options: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_options", vec![options])
    }

    pub fn get_selected(obj: &Value) -> Value {
        crate::wrappers::lvgl::method(obj, "get_selected", vec![])
    }

    pub fn set_range(obj: &Value, min: Value, max: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_range", vec![min, max])
    }

    pub fn set_bg_color(obj: &Value, r: Value, g: Value, b: Value, sel: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_bg_color", vec![r, g, b, sel])
    }

    pub fn set_border_color(obj: &Value, r: Value, g: Value, b: Value, sel: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_border_color", vec![r, g, b, sel])
    }

    pub fn set_text_color(obj: &Value, r: Value, g: Value, b: Value, sel: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_text_color", vec![r, g, b, sel])
    }

    pub fn set_radius(obj: &Value, r: Value, sel: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_radius", vec![r, sel])
    }

    pub fn set_pad(obj: &Value, pad: Value, sel: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "set_pad", vec![pad, sel])
    }

    pub fn add_event_cb(obj: &Value, cb_id: Value, event_code: Value) -> Value {
        crate::wrappers::lvgl::method(obj, "add_event_cb", vec![cb_id, event_code])
    }
}

pub mod cuda {
    use super::Value;

    pub fn init() -> Value {
        crate::wrappers::cuda::init()
    }

    pub fn device_count() -> Value {
        crate::wrappers::cuda::device_count()
    }

    pub fn device_name(device: Value) -> Value {
        let d = match device { Value::Int(i) => i, _ => return Value::Str(String::new()) };
        crate::wrappers::cuda::device_name(d)
    }

    pub fn device_props(device: Value) -> Value {
        let d = match device { Value::Int(i) => i, _ => return Value::None };
        crate::wrappers::cuda::device_props(d)
    }

    pub fn set_device(device: Value) -> Value {
        let d = match device { Value::Int(i) => i, _ => return Value::Int(-1) };
        crate::wrappers::cuda::set_device(d)
    }

    pub fn malloc(size: Value) -> Value {
        let s = match size { Value::Int(i) => i, _ => return Value::None };
        crate::wrappers::cuda::malloc(s)
    }

    pub fn free(ptr: &Value) -> Value {
        crate::wrappers::cuda::free(ptr)
    }

    pub fn memcpy_host_to_device(host: Value, dev: &Value) -> Value {
        let h = match &host { Value::Bytes(b) => b.clone(), _ => return Value::Int(-1) };
        crate::wrappers::cuda::memcpy_host_to_device(&h, dev)
    }

    pub fn memcpy_device_to_host(dev: &Value, size: Value) -> Value {
        let s = match size { Value::Int(i) => i, _ => return Value::Bytes(vec![]) };
        crate::wrappers::cuda::memcpy_device_to_host(dev, s)
    }

    pub fn memcpy_device_to_device(src: &Value, dst: &Value, size: Value) -> Value {
        let s = match size { Value::Int(i) => i, _ => return Value::Int(-1) };
        crate::wrappers::cuda::memcpy_device_to_device(src, dst, s)
    }

    pub fn malloc_host(size: Value) -> Value {
        let s = match size { Value::Int(i) => i, _ => return Value::None };
        crate::wrappers::cuda::malloc_host(s)
    }

    pub fn free_host(ptr: &Value) -> Value {
        crate::wrappers::cuda::free_host(ptr)
    }

    pub fn memset(ptr: &Value, val: Value, size: Value) -> Value {
        let v = match val { Value::Int(i) => i, _ => return Value::Int(-1) };
        let s = match size { Value::Int(i) => i, _ => return Value::Int(-1) };
        crate::wrappers::cuda::memset(ptr, v, s)
    }

    pub fn synchronize() -> Value {
        crate::wrappers::cuda::synchronize()
    }

    pub fn get_last_error() -> Value {
        crate::wrappers::cuda::get_last_error()
    }

    pub fn launch_vector_add(a_dev: &Value, b_dev: &Value, c_dev: &Value, n: Value) -> Value {
        let nv = match n { Value::Int(i) => i, _ => return Value::Int(-1) };
        crate::wrappers::cuda::launch_vector_add(a_dev, b_dev, c_dev, nv)
    }

    pub fn cublas_create() -> Value {
        crate::wrappers::cuda::cublas_create()
    }

    pub fn cublas_destroy(handle: &Value) -> Value {
        crate::wrappers::cuda::cublas_destroy(handle)
    }

    pub fn cublas_sgemm(handle: &Value, transa: Value, transb: Value, m: Value, n: Value, k: Value, alpha: Value, A: &Value, lda: Value, B: &Value, ldb: Value, beta: Value, C: &Value, ldc: Value) -> Value {
        let ta = match &transa { Value::Str(s) => s.clone(), _ => "n".to_string() };
        let tb = match &transb { Value::Str(s) => s.clone(), _ => "n".to_string() };
        let mv = match m { Value::Int(i) => i, _ => return Value::Int(-1) };
        let nv = match n { Value::Int(i) => i, _ => return Value::Int(-1) };
        let kv = match k { Value::Int(i) => i, _ => return Value::Int(-1) };
        let av = match alpha { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 1.0 };
        let lda_v = match lda { Value::Int(i) => i, _ => mv };
        let ldb_v = match ldb { Value::Int(i) => i, _ => kv };
        let bv = match beta { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let ldc_v = match ldc { Value::Int(i) => i, _ => nv };
        crate::wrappers::cuda::cublas_sgemm(handle, &ta, &tb, mv, nv, kv, av, A, lda_v, B, ldb_v, bv, C, ldc_v)
    }

    pub fn cublas_sdot(handle: &Value, n: Value, x: &Value, incx: Value, y: &Value, incy: Value) -> Value {
        let nv = match n { Value::Int(i) => i, _ => 0 };
        let incxv = match incx { Value::Int(i) => i, _ => 1 };
        let incyv = match incy { Value::Int(i) => i, _ => 1 };
        crate::wrappers::cuda::cublas_sdot(handle, nv, x, incxv, y, incyv)
    }
}

pub mod numpy {

    use super::Value;

    pub fn zeros(shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::numpy::zeros(&dims)
    }

    pub fn ones(shape: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::numpy::ones(&dims)
    }

    pub fn eye(n: Value) -> Value {
        let nv = match n { Value::Int(i) => i, _ => return Value::None };
        crate::wrappers::numpy::eye(nv)
    }

    pub fn arange(start: Value, stop: Value, step: Value) -> Value {
        let s = match start { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let e = match stop { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let st = match step { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 1.0 };
        crate::wrappers::numpy::arange(s, e, st)
    }

    pub fn linspace(start: Value, stop: Value, num: Value) -> Value {
        let s = match start { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let e = match stop { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 1.0 };
        let n = match num { Value::Int(i) => i, _ => 50 };
        crate::wrappers::numpy::linspace(s, e, n)
    }

    pub fn full(shape: Value, value: Value) -> Value {
        let dims = match &shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        let v = match value { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        crate::wrappers::numpy::full(&dims, v)
    }

    pub fn copy(arr: &Value) -> Value {
        crate::wrappers::numpy::copy(arr)
    }

    pub fn reshape(arr: &Value, new_shape: Value) -> Value {
        let dims = match &new_shape {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::numpy::reshape(arr, &dims)
    }

    pub fn transpose(arr: &Value) -> Value {
        crate::wrappers::numpy::transpose(arr)
    }

    pub fn concatenate(arrs: Value, axis: Value) -> Value {
        let items = match &arrs { Value::List(v) => v.clone(), Value::Tuple(v) => v.clone(), _ => return Value::None };
        let ax = match axis { Value::Int(i) => i, _ => 0 };
        crate::wrappers::numpy::concatenate(&items, ax)
    }

    pub fn stack(arrs: Value, axis: Value) -> Value {
        let items = match &arrs { Value::List(v) => v.clone(), Value::Tuple(v) => v.clone(), _ => return Value::None };
        let ax = match axis { Value::Int(i) => i, _ => 0 };
        crate::wrappers::numpy::stack(&items, ax)
    }

    pub fn add(a: &Value, b: &Value) -> Value {
        crate::wrappers::numpy::add(a, b)
    }

    pub fn sub(a: &Value, b: &Value) -> Value {
        crate::wrappers::numpy::sub(a, b)
    }

    pub fn mul(a: &Value, b: &Value) -> Value {
        crate::wrappers::numpy::mul(a, b)
    }

    pub fn div(a: &Value, b: &Value) -> Value {
        crate::wrappers::numpy::div(a, b)
    }

    pub fn dot(a: &Value, b: &Value) -> Value {
        crate::wrappers::numpy::dot(a, b)
    }

    pub fn matmul(a: &Value, b: &Value) -> Value {
        crate::wrappers::numpy::matmul(a, b)
    }

    pub fn sum(arr: &Value, axis: Value) -> Value {
        let ax = match axis { Value::Int(i) => i, _ => -1 };
        crate::wrappers::numpy::sum(arr, ax)
    }

    pub fn mean(arr: &Value, axis: Value) -> Value {
        let ax = match axis { Value::Int(i) => i, _ => -1 };
        crate::wrappers::numpy::mean(arr, ax)
    }

    pub fn std(arr: &Value, axis: Value) -> Value {
        let ax = match axis { Value::Int(i) => i, _ => -1 };
        crate::wrappers::numpy::std(arr, ax)
    }

    pub fn min(arr: &Value, axis: Value) -> Value {
        let ax = match axis { Value::Int(i) => i, _ => -1 };
        crate::wrappers::numpy::min(arr, ax)
    }

    pub fn max(arr: &Value, axis: Value) -> Value {
        let ax = match axis { Value::Int(i) => i, _ => -1 };
        crate::wrappers::numpy::max(arr, ax)
    }

    pub fn argmin(arr: &Value) -> Value {
        crate::wrappers::numpy::argmin(arr)
    }

    pub fn argmax(arr: &Value) -> Value {
        crate::wrappers::numpy::argmax(arr)
    }

    pub fn exp(arr: &Value) -> Value {
        crate::wrappers::numpy::exp(arr)
    }

    pub fn log(arr: &Value) -> Value {
        crate::wrappers::numpy::log(arr)
    }

    pub fn sqrt(arr: &Value) -> Value {
        crate::wrappers::numpy::sqrt(arr)
    }

    pub fn sin(arr: &Value) -> Value {
        crate::wrappers::numpy::sin(arr)
    }

    pub fn cos(arr: &Value) -> Value {
        crate::wrappers::numpy::cos(arr)
    }

    pub fn tan(arr: &Value) -> Value {
        crate::wrappers::numpy::tan(arr)
    }

    pub fn abs(arr: &Value) -> Value {
        crate::wrappers::numpy::abs(arr)
    }

    pub fn floor(arr: &Value) -> Value {
        crate::wrappers::numpy::floor(arr)
    }

    pub fn ceil(arr: &Value) -> Value {
        crate::wrappers::numpy::ceil(arr)
    }

    pub fn clip(arr: &Value, min: Value, max: Value) -> Value {
        let mn = match min { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        let mx = match max { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        crate::wrappers::numpy::clip(arr, mn, mx)
    }

    pub fn where_(cond: &Value, x: &Value, y: &Value) -> Value {
        crate::wrappers::numpy::where_(cond, x, y)
    }

    pub fn sort(arr: &Value, axis: Value) -> Value {
        let ax = match axis { Value::Int(i) => i, _ => -1 };
        crate::wrappers::numpy::sort(arr, ax)
    }

    pub fn unique(arr: &Value) -> Value {
        crate::wrappers::numpy::unique(arr)
    }

    pub fn to_string(arr: &Value) -> Value {
        crate::wrappers::numpy::to_string(arr)
    }

    pub fn shape(arr: &Value) -> Value {
        crate::wrappers::numpy::shape(arr)
    }

    pub fn ndim(arr: &Value) -> Value {
        crate::wrappers::numpy::ndim(arr)
    }

    pub fn size(arr: &Value) -> Value {
        crate::wrappers::numpy::size(arr)
    }

    pub fn item(arr: &Value, indices: Value) -> Value {
        let idxs = match &indices {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        crate::wrappers::numpy::item(arr, &idxs)
    }

    pub fn set_item(arr: &Value, indices: Value, value: Value) -> Value {
        let idxs = match &indices {
            Value::Tuple(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::List(items) => items.iter().map(|v| match v { Value::Int(i) => *i, _ => 0 }).collect::<Vec<_>>(),
            Value::Int(i) => vec![*i],
            _ => return Value::None,
        };
        let v = match value { Value::Float(f) => f, Value::Int(i) => i as f64, _ => 0.0 };
        crate::wrappers::numpy::set_item(arr, &idxs, v);
        Value::None
    }

    pub fn tolist(arr: &Value) -> Value {
        crate::wrappers::numpy::to_float_array(arr)
    }

    pub fn free(arr: &Value) -> Value {
        crate::wrappers::numpy::free(arr);
        Value::None
    }
}
