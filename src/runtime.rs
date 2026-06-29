use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Ptr(*mut c_void),
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Dict(BTreeMap<String, Value>),
    Set(Vec<Value>),
    Slice(Box<Value>, Box<Value>, Box<Value>),
    Range(i64, i64, i64),
    Bytes(Vec<u8>),
}

impl Value {
    pub fn repr(&self) -> String {
        match self {
            Value::Str(s) => {
                let escaped = s.replace('\\', "\\\\").replace('\'', "\\'").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t");
                format!("'{}'", escaped)
            }
            Value::List(items) => {
                let inner: Vec<String> = items.iter().map(|i| i.repr()).collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Tuple(items) => {
                let inner: Vec<String> = items.iter().map(|i| i.repr()).collect();
                if inner.len() == 1 {
                    format!("({},)", inner[0])
                } else {
                    format!("({})", inner.join(", "))
                }
            }
            Value::Dict(items) => {
                let inner: Vec<String> = items.iter().map(|(k, v)| format!("'{}': {}", k, v.repr())).collect();
                format!("{{{}}}", inner.join(", "))
            }
            Value::Set(items) => {
                let inner: Vec<String> = items.iter().map(|i| i.repr()).collect();
                format!("{{{}}}", inner.join(", "))
            }
            _ => self.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, "None"),
            Value::Ptr(_) => write!(f, "<ptr>"),
            Value::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}.0", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Str(s) => write!(f, "{}", s),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item.repr())?;
                }
                write!(f, "]")
            }
            Value::Tuple(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item.repr())?;
                }
                if items.len() == 1 { write!(f, ",")?; }
                write!(f, ")")
            }
            Value::Dict(items) => {
                write!(f, "{{")?;
                for (i, (k, v)) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "'{}': {}", k, v.repr())?;
                }
                write!(f, "}}")
            }
            Value::Set(items) => {
                write!(f, "{{")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item.repr())?;
                }
                write!(f, "}}")
            }
            Value::Slice(start, stop, step) => {
                write!(f, "slice({}, {}, {})", start, stop, step)
            }
            Value::Range(start, stop, step) => {
                write!(f, "range({}, {}, {})", start, stop, step)
            }
            Value::Bytes(data) => write!(f, "<bytes len={}>", data.len()),
        }
    }
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::Ptr(_) => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::List(items) => !items.is_empty(),
            Value::Tuple(items) => !items.is_empty(),
            Value::Dict(items) => !items.is_empty(),
            Value::Set(items) => !items.is_empty(),
            Value::Slice(_, _, _) => true,
            Value::Range(_, _, _) => true,
            Value::Bytes(data) => !data.is_empty(),
        }
    }

    pub fn eq_val(&self, other: &Value) -> Value { Value::Bool(self == other) }
    pub fn ne_val(&self, other: &Value) -> Value { Value::Bool(self != other) }
    pub fn lt_val(&self, other: &Value) -> Value { Value::Bool(match self.partial_cmp(other) { Some(std::cmp::Ordering::Less) => true, _ => false }) }
    pub fn le_val(&self, other: &Value) -> Value { Value::Bool(match self.partial_cmp(other) { Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal) => true, _ => false }) }
    pub fn gt_val(&self, other: &Value) -> Value { Value::Bool(match self.partial_cmp(other) { Some(std::cmp::Ordering::Greater) => true, _ => false }) }
    pub fn ge_val(&self, other: &Value) -> Value { Value::Bool(match self.partial_cmp(other) { Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal) => true, _ => false }) }

    pub fn to_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::Float(n) => *n as i64,
            Value::Bool(b) => if *b { 1 } else { 0 },
            Value::Str(s) => s.parse().unwrap_or(0),
            Value::None => 0,
            _ => 0,
        }
    }

    pub fn to_float(&self) -> f64 {
        match self {
            Value::Int(n) => *n as f64,
            Value::Float(n) => *n,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::Str(s) => s.parse().unwrap_or(0.0),
            Value::None => 0.0,
            _ => 0.0,
        }
    }

    // --- Context manager protocol (__enter__ / __exit__) ---
    /// `__enter__()` — called when entering a `with` block.
    /// Dict returns self; Ptr dispatches to attribute; default returns self.
    pub fn __enter__(&self) -> Value {
        match self {
            Value::Dict(_) => {
                // dict `__enter__` returns self
                self.clone()
            }
            Value::Ptr(_) => {
                // Custom Python class — try attribute dispatch
                let attr = self.get_attr("__enter__");
                if attr.is_none() {
                    panic!("TypeError: '{}' object does not support the context manager protocol",
                           self.type_name());
                }
                attr
            }
            _ => {
                self.clone()
            }
        }
    }

    /// `__exit__(exc_type, exc_val, exc_tb)` — called when leaving a `with` block.
    /// Returns `false` to not suppress exceptions by default.
    pub fn __exit__(&self, _exc_type: Value, _exc_val: Value, _exc_tb: Value) -> Value {
        match self {
            Value::Ptr(_) => {
                // Custom Python class — try attribute dispatch
                let _attr = self.get_attr("__exit__");
                Value::Bool(false)
            }
            _ => Value::Bool(false),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    pub fn to_list(&self) -> Vec<Value> {
        match self {
            Value::List(items) => items.clone(),
            Value::Tuple(items) => items.clone(),
            Value::Set(items) => items.clone(),
            Value::Str(s) => s.chars().map(|c| Value::Str(c.to_string())).collect(),
            _ => vec![self.clone()],
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Value::None => "NoneType".to_string(),
            Value::Ptr(_) => "ptr".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Int(_) => "int".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::Str(_) => "str".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Tuple(_) => "tuple".to_string(),
            Value::Dict(_) => "dict".to_string(),
            Value::Set(_) => "set".to_string(),
            Value::Slice(_, _, _) => "slice".to_string(),
            Value::Range(_, _, _) => "range".to_string(),
            Value::Bytes(_) => "bytes".to_string(),
        }
    }

    pub fn type_repr(&self) -> String {
        format!("<class '{}'>", self.type_name())
    }

    pub fn len_(&self) -> Value {
        match self {
            Value::Str(s) => Value::Int(s.len() as i64),
            Value::List(items) => Value::Int(items.len() as i64),
            Value::Tuple(items) => Value::Int(items.len() as i64),
            Value::Dict(items) => Value::Int(items.len() as i64),
            Value::Set(items) => Value::Int(items.len() as i64),
            Value::Range(start, stop, step) => {
                if *step == 0 { return Value::Int(0); }
                let len = if *step > 0 {
                    if *start >= *stop { 0 } else { (*stop - *start - 1) / *step + 1 }
                } else {
                    if *start <= *stop { 0 } else { (*start - *stop - 1) / (-*step) + 1 }
                };
                Value::Int(len)
            }
            Value::Bytes(data) => Value::Int(data.len() as i64),
            _ => Value::Int(0),
        }
    }

    pub fn index(&self, idx: &Value) -> Value {
        match (self, idx) {
            (Value::List(items), Value::Int(i)) => {
                let i = if *i < 0 { items.len() as i64 + *i } else { *i } as usize;
                items.get(i).cloned().unwrap_or(Value::None)
            }
            (Value::Tuple(items), Value::Int(i)) => {
                let i = if *i < 0 { items.len() as i64 + *i } else { *i } as usize;
                items.get(i).cloned().unwrap_or(Value::None)
            }
            (Value::Str(s), Value::Int(i)) => {
                let i = if *i < 0 { s.len() as i64 + *i } else { *i } as usize;
                s.chars().nth(i).map(|c| Value::Str(c.to_string())).unwrap_or(Value::None)
            }
            (Value::Dict(d), _) => {
                let key = format!("{}", idx);
                d.get(&key).cloned().unwrap_or(Value::None)
            }
            (Value::List(items), Value::Slice(start, stop, step)) => {
                let len = items.len() as i64;
                let s = start.as_ref().to_int();
                let e = stop.as_ref().to_int();
                let st = step.as_ref().to_int();
                let s = if s == 0 && matches!(start.as_ref(), Value::None) { 0 } else { if s < 0 { len + s } else { s } };
                let e = if e == 0 && matches!(stop.as_ref(), Value::None) { len } else { if e < 0 { len + e } else { e } };
                let st = if st == 0 && matches!(step.as_ref(), Value::None) { 1 } else { st };

                let mut result = Vec::new();
                if st > 0 {
                    let mut i = s;
                    while i < e && i < len {
                        result.push(items[i as usize].clone());
                        i += st;
                    }
                } else if st < 0 {
                    let mut i = if e >= len { len - 1 } else { e - 1 };
                    let end = s;
                    while i >= end && i >= 0 {
                        result.push(items[i as usize].clone());
                        i += st;
                    }
                }
                Value::List(result)
            }
            _ => Value::None,
        }
    }

    pub fn set_item(&mut self, idx: &Value, val: Value) {
        match (self, idx) {
            (Value::List(items), Value::Int(i)) => {
                let i = if *i < 0 { items.len() as i64 + *i } else { *i } as usize;
                if i < items.len() {
                    items[i] = val;
                }
            }
            (Value::Dict(d), _) => {
                let key = format!("{}", idx);
                d.insert(key, val);
            }
            _ => {}
        }
    }

    pub fn get_attr(&self, name: &str) -> Value {
        match self {
            Value::Dict(d) => d.get(name).cloned().unwrap_or(Value::None),
            _ => Value::None,
        }
    }

    pub fn set_attr(&mut self, name: &str, val: Value) {
        if let Value::Dict(ref mut d) = self {
            d.insert(name.to_string(), val);
        }
    }

    pub fn is_(&self, other: &Value) -> Value {
        Value::Bool(std::ptr::eq(self, other))
    }

    pub fn is_not(&self, other: &Value) -> Value {
        Value::Bool(!std::ptr::eq(self, other))
    }

    pub fn is_in(&self, other: &Value) -> Value {
        match other {
            Value::List(items) => Value::Bool(items.contains(self)),
            Value::Tuple(items) => Value::Bool(items.contains(self)),
            Value::Set(items) => Value::Bool(items.contains(self)),
            Value::Str(s) => {
                if let Value::Str(pat) = self {
                    Value::Bool(s.contains(pat))
                } else {
                    Value::Bool(false)
                }
            }
            Value::Dict(d) => {
                let key = format!("{}", self);
                Value::Bool(d.contains_key(&key))
            }
            _ => Value::Bool(false),
        }
    }

    pub fn not_in(&self, other: &Value) -> Value {
        let result = self.is_in(other);
        match result {
            Value::Bool(b) => Value::Bool(!b),
            _ => Value::Bool(true),
        }
    }

    pub fn floordiv(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 { panic!("Division by zero"); }
                Value::Int(a.div_euclid(*b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 { panic!("Division by zero"); }
                Value::Float((a / b).floor())
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 { panic!("Division by zero"); }
                Value::Float((*a as f64 / b).floor())
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 { panic!("Division by zero"); }
                Value::Float((a / *b as f64).floor())
            }
            _ => panic!("TypeError: unsupported operand type(s) for //"),
        }
    }

    pub fn pow(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b < 0 {
                    Value::Float((*a as f64).powi(*b as i32))
                } else {
                    Value::Int(a.pow(*b as u32))
                }
            }
            (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).powf(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.powi(*b as i32)),
            _ => panic!("TypeError: unsupported operand type(s) for **"),
        }
    }

    pub fn lshift(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a << b),
            _ => panic!("TypeError: unsupported operand type(s) for <<"),
        }
    }

    pub fn rshift(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a >> b),
            _ => panic!("TypeError: unsupported operand type(s) for >>"),
        }
    }

    pub fn bitor(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a | b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(*a || *b),
            (Value::Set(a), Value::Set(b)) => {
                let mut result = a.clone();
                for item in b {
                    if !result.contains(item) {
                        result.push(item.clone());
                    }
                }
                Value::Set(result)
            }
            _ => panic!("TypeError: unsupported operand type(s) for |"),
        }
    }

    pub fn bitxor(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a ^ b),
            _ => panic!("TypeError: unsupported operand type(s) for ^"),
        }
    }

    pub fn bitand(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a & b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(*a && *b),
            (Value::Bool(a), Value::Int(b)) => Value::Bool(*a && *b != 0),
            (Value::Int(a), Value::Bool(b)) => Value::Bool(*a != 0 && *b),
            _ => panic!("TypeError: unsupported operand type(s) for &"),
        }
    }

    pub fn matmult(&self, other: &Value) -> Value {
        panic!("TypeError: unsupported operand type(s) for @");
    }

    pub fn not_(&self) -> Value {
        Value::Bool(!self.to_bool())
    }

    // --- Container methods ---
    pub fn append(&mut self, val: Value) -> Value {
        match self {
            Value::List(ref mut items) => { items.push(val); Value::None }
            _ => panic!("TypeError: '{}' object has no attribute 'append'", self.type_name()),
        }
    }

    pub fn pop(&mut self, args: Vec<Value>) -> Value {
        match self {
            Value::List(ref mut items) => {
                let idx = args.first().map(|v| v.to_int()).unwrap_or(-1);
                if items.is_empty() { panic!("IndexError: pop from empty list"); }
                let i = if idx < 0 { items.len() as i64 + idx } else { idx } as usize;
                if i >= items.len() { panic!("IndexError: pop index out of range"); }
                items.remove(i)
            }
            Value::Dict(ref mut d) => {
                let key = args.first().map(|v| format!("{}", v)).unwrap_or_default();
                d.remove(&key).unwrap_or(Value::None)
            }
            _ => panic!("TypeError: '{}' object has no attribute 'pop'", self.type_name()),
        }
    }

    pub fn extend(&mut self, other: Value) -> Value {
        match self {
            Value::List(ref mut items) => {
                items.extend(other.to_list());
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'extend'", self.type_name()),
        }
    }

    pub fn insert(&mut self, args: Vec<Value>) -> Value {
        match self {
            Value::List(ref mut items) => {
                let idx = args.get(0).map(|v| v.to_int()).unwrap_or(0);
                let val = args.get(1).cloned().unwrap_or(Value::None);
                let i = if idx < 0 { 0.max(items.len() as i64 + idx) } else { idx } as usize;
                let i = i.min(items.len());
                items.insert(i, val);
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'insert'", self.type_name()),
        }
    }

    pub fn remove(&mut self, val: Value) -> Value {
        match self {
            Value::List(ref mut items) => {
                if let Some(pos) = items.iter().position(|x| *x == val) {
                    items.remove(pos);
                } else {
                    panic!("ValueError: list.remove(x): x not in list");
                }
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'remove'", self.type_name()),
        }
    }

    pub fn clear(&mut self) -> Value {
        match self {
            Value::List(ref mut items) => { items.clear(); Value::None }
            Value::Dict(ref mut d) => { d.clear(); Value::None }
            _ => panic!("TypeError: '{}' object has no attribute 'clear'", self.type_name()),
        }
    }

    pub fn sort(&mut self) -> Value {
        match self {
            Value::List(ref mut items) => {
                items.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'sort'", self.type_name()),
        }
    }

    pub fn reverse(&mut self) -> Value {
        match self {
            Value::List(ref mut items) => { items.reverse(); Value::None }
            _ => panic!("TypeError: '{}' object has no attribute 'reverse'", self.type_name()),
        }
    }

    pub fn count(&self, val: Value) -> Value {
        match self {
            Value::List(items) => Value::Int(items.iter().filter(|x| **x == val).count() as i64),
            Value::Str(s) => {
                if let Value::Str(pat) = &val {
                    Value::Int(s.matches(pat).count() as i64)
                } else {
                    Value::Int(0)
                }
            }
            _ => panic!("TypeError: '{}' object has no attribute 'count'", self.type_name()),
        }
    }

    pub fn index_val(&self, val: Value) -> Value {
        match self {
            Value::List(items) => {
                items.iter().position(|x| *x == val).map(|i| Value::Int(i as i64)).unwrap_or_else(|| {
                    panic!("ValueError: {} is not in list", val);
                })
            }
            Value::Str(s) => {
                if let Value::Str(pat) = &val {
                    s.find(pat).map(|i| Value::Int(i as i64)).unwrap_or_else(|| {
                        Value::Int(-1)
                    })
                } else {
                    Value::Int(-1)
                }
            }
            _ => panic!("TypeError: '{}' object has no attribute 'index'", self.type_name()),
        }
    }

    // --- Dict methods ---
    pub fn keys(&self) -> Value {
        match self {
            Value::Dict(d) => {
                let items: Vec<String> = d.keys().map(|k| Value::Str(k.clone()).repr()).collect();
                Value::Str(format!("dict_keys([{}])", items.join(", ")))
            }
            _ => panic!("TypeError: '{}' object has no attribute 'keys'", self.type_name()),
        }
    }

    pub fn values(&self) -> Value {
        match self {
            Value::Dict(d) => {
                let items: Vec<String> = d.values().map(|v| v.repr()).collect();
                Value::Str(format!("dict_values([{}])", items.join(", ")))
            }
            _ => panic!("TypeError: '{}' object has no attribute 'values'", self.type_name()),
        }
    }

    pub fn items(&self) -> Value {
        match self {
            Value::Dict(d) => {
                let items: Vec<String> = d.iter()
                    .map(|(k, v)| format!("({}, {})", Value::Str(k.clone()).repr(), v.repr()))
                    .collect();
                Value::Str(format!("dict_items([{}])", items.join(", ")))
            }
            _ => panic!("TypeError: '{}' object has no attribute 'items'", self.type_name()),
        }
    }

    pub fn dict_get(&self, args: Vec<Value>) -> Value {
        match self {
            Value::Dict(d) => {
                let key = args.get(0).map(|v| format!("{}", v)).unwrap_or_default();
                d.get(&key).cloned().or_else(|| args.get(1).cloned()).unwrap_or(Value::None)
            }
            _ => panic!("TypeError: '{}' object has no attribute 'get'", self.type_name()),
        }
    }

    pub fn update(&mut self, other: Value) -> Value {
        match self {
            Value::Dict(ref mut d) => {
                if let Value::Dict(od) = other {
                    for (k, v) in od { d.insert(k, v); }
                }
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'update'", self.type_name()),
        }
    }

    pub fn setdefault(&mut self, args: Vec<Value>) -> Value {
        match self {
            Value::Dict(ref mut d) => {
                let key = args.get(0).map(|v| format!("{}", v)).unwrap_or_default();
                if !d.contains_key(&key) {
                    let val = args.get(1).cloned().unwrap_or(Value::None);
                    d.insert(key.clone(), val.clone());
                    val
                } else {
                    d.get(&key).cloned().unwrap_or(Value::None)
                }
            }
            _ => panic!("TypeError: '{}' object has no attribute 'setdefault'", self.type_name()),
        }
    }

    // --- Set methods ---
    pub fn add_set(&mut self, val: Value) -> Value {
        match self {
            Value::Set(ref mut items) => {
                if !items.contains(&val) { items.push(val); }
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'add'", self.type_name()),
        }
    }

    pub fn discard(&mut self, val: Value) -> Value {
        match self {
            Value::Set(ref mut items) => {
                items.retain(|x| *x != val);
                Value::None
            }
            _ => panic!("TypeError: '{}' object has no attribute 'discard'", self.type_name()),
        }
    }

    // --- Str methods ---
    pub fn split(&self, args: Vec<Value>) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let sep = args.first().map(|v| match v { Value::None => None, _ => Some(format!("{}", v)) }).unwrap_or(None);
        let maxsplit = args.get(1).map(|v| v.to_int() as usize).unwrap_or(usize::MAX);
        match sep {
            None => Value::List(s.split_whitespace().map(|x| Value::Str(x.to_string())).collect()),
            Some(sep) => {
                let parts: Vec<Value> = if maxsplit < usize::MAX {
                    let mut result = Vec::new();
                    let mut remaining = s.as_str();
                    let mut count = 0;
                    while let Some(pos) = remaining.find(&sep) {
                        if count >= maxsplit { break; }
                        result.push(Value::Str(remaining[..pos].to_string()));
                        remaining = &remaining[pos + sep.len()..];
                        count += 1;
                    }
                    result.push(Value::Str(remaining.to_string()));
                    result
                } else {
                    s.split(&sep).map(|x| Value::Str(x.to_string())).collect()
                };
                Value::List(parts)
            }
        }
    }

    pub fn join(&self, items: Value) -> Value {
        let sep = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let parts: Vec<String> = match items {
            Value::List(ref v) | Value::Tuple(ref v) => v.iter().map(|x| format!("{}", x)).collect(),
            _ => panic!("TypeError"),
        };
        Value::Str(parts.join(sep))
    }

    pub fn strip(&self) -> Value {
        match self { Value::Str(s) => Value::Str(s.trim().to_string()), _ => panic!("TypeError") }
    }

    pub fn lstrip(&self) -> Value {
        match self { Value::Str(s) => Value::Str(s.trim_start().to_string()), _ => panic!("TypeError") }
    }

    pub fn rstrip(&self) -> Value {
        match self { Value::Str(s) => Value::Str(s.trim_end().to_string()), _ => panic!("TypeError") }
    }

    pub fn replace(&self, args: Vec<Value>) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let old = args.get(0).map(|v| format!("{}", v)).unwrap_or_default();
        let new = args.get(1).map(|v| format!("{}", v)).unwrap_or_default();
        let count = args.get(2).map(|v| v.to_int() as usize).unwrap_or(usize::MAX);
        if count < usize::MAX {
            let mut result = s.clone();
            let mut n = 0;
            while let Some(pos) = result.find(&old) {
                if n >= count { break; }
                result.replace_range(pos..pos + old.len(), &new);
                n += 1;
            }
            Value::Str(result)
        } else {
            Value::Str(s.replace(&old, &new))
        }
    }

    pub fn upper(&self) -> Value {
        match self { Value::Str(s) => Value::Str(s.to_uppercase()), _ => panic!("TypeError") }
    }

    pub fn lower(&self) -> Value {
        match self { Value::Str(s) => Value::Str(s.to_lowercase()), _ => panic!("TypeError") }
    }

    pub fn startswith(&self, val: Value) -> Value {
        match self {
            Value::Str(s) => {
                let prefix = format!("{}", val);
                Value::Bool(s.starts_with(&prefix))
            }
            _ => panic!("TypeError"),
        }
    }

    pub fn endswith(&self, val: Value) -> Value {
        match self {
            Value::Str(s) => {
                let suffix = format!("{}", val);
                Value::Bool(s.ends_with(&suffix))
            }
            _ => panic!("TypeError"),
        }
    }

    pub fn find(&self, val: Value) -> Value {
        match self {
            Value::Str(s) => {
                let pat = format!("{}", val);
                Value::Int(s.find(&pat).map(|i| i as i64).unwrap_or(-1))
            }
            _ => panic!("TypeError"),
        }
    }

    pub fn rfind(&self, val: Value) -> Value {
        match self {
            Value::Str(s) => {
                let pat = format!("{}", val);
                Value::Int(s.rfind(&pat).map(|i| i as i64).unwrap_or(-1))
            }
            _ => panic!("TypeError"),
        }
    }

    pub fn isdigit(&self) -> Value {
        match self { Value::Str(s) => Value::Bool(s.chars().all(|c| c.is_ascii_digit())), _ => panic!("TypeError") }
    }

    pub fn isalpha(&self) -> Value {
        match self { Value::Str(s) => Value::Bool(s.chars().all(|c| c.is_alphabetic())), _ => panic!("TypeError") }
    }

    pub fn isalnum(&self) -> Value {
        match self { Value::Str(s) => Value::Bool(s.chars().all(|c| c.is_alphanumeric())), _ => panic!("TypeError") }
    }

    pub fn isspace(&self) -> Value {
        match self { Value::Str(s) => Value::Bool(s.chars().all(|c| c.is_whitespace())), _ => panic!("TypeError") }
    }

    pub fn capitalize(&self) -> Value {
        match self {
            Value::Str(s) => {
                let mut c = s.chars();
                match c.next() {
                    None => Value::Str(String::new()),
                    Some(f) => Value::Str(f.to_uppercase().to_string() + c.as_str()),
                }
            }
            _ => panic!("TypeError"),
        }
    }

    pub fn title(&self) -> Value {
        match self { Value::Str(s) => Value::Str(s.to_string()), _ => panic!("TypeError") }
    }

    pub fn swapcase(&self) -> Value {
        match self {
            Value::Str(s) => Value::Str(s.chars().map(|c| {
                if c.is_uppercase() { c.to_lowercase().to_string() }
                else { c.to_uppercase().to_string() }
            }).collect()),
            _ => panic!("TypeError"),
        }
    }

    pub fn zfill(&self, width: Value) -> Value {
        if let Value::Str(s) = self {
            let w = width.to_int() as usize;
            if s.len() >= w { return Value::Str(s.clone()); }
            Value::Str(format!("{:0>width$}", s, width = w))
        } else { panic!("TypeError") }
    }

    pub fn center(&self, args: Vec<Value>) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let w = args.get(0).map(|v| v.to_int() as usize).unwrap_or(0);
        let fill = args.get(1).map(|v| format!("{}", v)).unwrap_or_else(|| " ".to_string());
        if s.len() >= w { return Value::Str(s.clone()); }
        let pad = w - s.len();
        let left = pad / 2;
        let right = pad - left;
        Value::Str(format!("{}{}{}", fill.repeat(left), s, fill.repeat(right)))
    }

    pub fn ljust(&self, args: Vec<Value>) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let w = args.get(0).map(|v| v.to_int() as usize).unwrap_or(0);
        let fill = args.get(1).map(|v| format!("{}", v)).unwrap_or_else(|| " ".to_string());
        if s.len() >= w { return Value::Str(s.clone()); }
        Value::Str(format!("{}{}", s, fill.repeat(w - s.len())))
    }

    pub fn rjust(&self, args: Vec<Value>) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let w = args.get(0).map(|v| v.to_int() as usize).unwrap_or(0);
        let fill = args.get(1).map(|v| format!("{}", v)).unwrap_or_else(|| " ".to_string());
        if s.len() >= w { return Value::Str(s.clone()); }
        Value::Str(format!("{}{}", fill.repeat(w - s.len()), s))
    }

    pub fn partition(&self, sep: Value) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let sep_str = format!("{}", sep);
        match s.find(&sep_str) {
            Some(pos) => Value::Tuple(vec![
                Value::Str(s[..pos].to_string()),
                Value::Str(sep_str.clone()),
                Value::Str(s[pos + sep_str.len()..].to_string()),
            ]),
            None => Value::Tuple(vec![Value::Str(s.clone()), Value::Str(String::new()), Value::Str(String::new())]),
        }
    }

    pub fn rpartition(&self, sep: Value) -> Value {
        let s = match self { Value::Str(s) => s, _ => panic!("TypeError") };
        let sep_str = format!("{}", sep);
        match s.rfind(&sep_str) {
            Some(pos) => Value::Tuple(vec![
                Value::Str(s[..pos].to_string()),
                Value::Str(sep_str.clone()),
                Value::Str(s[pos + sep_str.len()..].to_string()),
            ]),
            None => Value::Tuple(vec![Value::Str(String::new()), Value::Str(String::new()), Value::Str(s.clone())]),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 + b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a + *b as f64),
            (Value::Str(a), Value::Str(b)) => Value::Str(format!("{}{}", a, b)),
            (Value::List(a), Value::List(b)) => {
                let mut items = a.clone();
                items.extend(b.clone());
                Value::List(items)
            }
            (Value::Tuple(a), Value::Tuple(b)) => {
                let mut items = a.clone();
                items.extend(b.clone());
                Value::Tuple(items)
            }
            _ => panic!("TypeError: unsupported operand type(s) for +"),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 - b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a - *b as f64),
            _ => panic!("TypeError: unsupported operand type(s) for -"),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Int(a), Value::Float(b)) => Value::Float(*a as f64 * b),
            (Value::Float(a), Value::Int(b)) => Value::Float(a * *b as f64),
            (Value::Str(s), Value::Int(n)) | (Value::Int(n), Value::Str(s)) => {
                Value::Str(s.repeat(*n as usize))
            }
            (Value::List(items), Value::Int(n)) | (Value::Int(n), Value::List(items)) => {
                let mut result = Vec::new();
                for _ in 0..*n {
                    result.extend(items.clone());
                }
                Value::List(result)
            }
            _ => panic!("TypeError: unsupported operand type(s) for *"),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;
    fn div(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 { panic!("Division by zero"); }
                Value::Float(*a as f64 / *b as f64)
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 { panic!("Division by zero"); }
                Value::Float(a / b)
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 { panic!("Division by zero"); }
                Value::Float(*a as f64 / b)
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 { panic!("Division by zero"); }
                Value::Float(a / *b as f64)
            }
            _ => panic!("TypeError: unsupported operand type(s) for /"),
        }
    }
}

impl std::ops::Rem for Value {
    type Output = Value;
    fn rem(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 { panic!("Division by zero"); }
                Value::Int(a % b)
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 { panic!("Division by zero"); }
                Value::Float(a % b)
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 { panic!("Division by zero"); }
                Value::Float(*a as f64 % b)
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 { panic!("Division by zero"); }
                Value::Float(a % *b as f64)
            }
            _ => panic!("TypeError: unsupported operand type(s) for %"),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Value;
    fn neg(self) -> Value {
        match self {
            Value::Int(a) => Value::Int(-a),
            Value::Float(a) => Value::Float(-a),
            _ => panic!("TypeError: bad operand type for unary -"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::None, Value::None) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Int(a), Value::Float(b)) => (*a as f64 - b).abs() < f64::EPSILON,
            (Value::Float(a), Value::Int(b)) => (a - *b as f64).abs() < f64::EPSILON,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Bytes(a), Value::Bytes(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Some(a.cmp(b)),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Str(a), Value::Str(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

pub fn print_impl(val: &Value) {
    print!("{}", val);
}

pub fn println_impl(val: &Value) {
    println!("{}", val);
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, other: Value) {
        *self = self.clone() + other;
    }
}

impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.to_list().into_iter()
    }
}
