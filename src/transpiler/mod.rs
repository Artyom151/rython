#![allow(dead_code)]

use crate::ast::*;
use std::collections::{BTreeMap, HashMap, HashSet};

pub const STDLIB_MODULES: &[&str] = &["math", "sys", "os", "json", "time", "PyQt6", "sqlite3", "sdl2", "opengl", "vulkan", "lvgl", "re", "random", "collections", "itertools", "functools", "pathlib", "datetime", "typing", "abc", "urllib", "image", "git", "gi", "ffmpeg", "font", "torch", "numpy", "cuda", "dataclasses"];

pub fn value_method_name(py_name: &str) -> &str {
    match py_name {
        "index" => "index_val",
        "get" => "dict_get",
        "add" => "add_set",
        other => other,
    }
}

pub const VALUE_METHODS: &[&str] = &[
    "append", "pop", "extend", "insert", "remove", "clear", "sort", "reverse",
    "count", "index",
    "keys", "values", "items", "get", "update", "setdefault",
    "add", "discard",
    "split", "join", "strip", "lstrip", "rstrip", "replace",
    "upper", "lower", "capitalize", "title", "swapcase",
    "startswith", "endswith", "find", "rfind",
    "isdigit", "isalpha", "isalnum", "isspace",
    "zfill", "center", "ljust", "rjust",
    "partition", "rpartition",
];

pub const MUTABLE_VALUE_METHODS: &[&str] = &[
    "append", "pop", "extend", "insert", "remove", "clear", "sort", "reverse",
    "update", "setdefault", "add", "discard",
];

pub const VEC_ARGS_VALUE_METHODS: &[&str] = &[
    "pop", "insert", "get", "setdefault", "split", "replace", "center", "ljust", "rjust",
];

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    List(Box<Type>),
    Tuple(Vec<Type>),
    Dict(Box<Type>, Box<Type>),
    Set(Box<Type>),
    None,
    Any,
}

impl Type {
    pub fn rust_type(&self) -> &str {
        match self {
            Type::Int => "i64",
            Type::Float => "f64",
            Type::Bool => "bool",
            Type::Str => "String",
            Type::List(_) => "Vec<Value>",
            Type::Tuple(_) => "Vec<Value>",
            Type::Dict(_, _) => "BTreeMap<String, Value>",
            Type::Set(_) => "Vec<Value>",
            Type::None => "()",
            Type::Any => "Value",
        }
    }

    pub fn wrap_in_value(&self, expr: &str) -> String {
        match self {
            Type::Int => format!("Value::Int({})", expr),
            Type::Float => format!("Value::Float({})", expr),
            Type::Bool => format!("Value::Bool({})", expr),
            Type::Str => format!("Value::Str({})", expr),
            Type::List(_) => format!("Value::List({})", expr),
            Type::Tuple(_) => format!("Value::Tuple({})", expr),
            Type::Dict(_, _) => format!("Value::Dict({})", expr),
            Type::Set(_) => format!("Value::Set({})", expr),
            Type::None => "Value::None".to_string(),
            Type::Any => expr.to_string(),
        }
    }

    pub fn unwrap_from_value(&self, expr: &str) -> String {
        match self {
            Type::Int => format!("({}).to_int()", expr),
            Type::Float => format!("({}).to_float()", expr),
            Type::Bool => format!("({}).to_bool()", expr),
            Type::Str => format!("format!(\"{{}}\", {})", expr),
            Type::List(_) => format!("({}).to_list()", expr),
            Type::Tuple(_) => format!("({}).to_list()", expr),
            Type::Dict(_, _) => format!("format!(\"{{}}\", {})", expr),
            Type::Set(_) => format!("({}).to_list()", expr),
            Type::None => "()".to_string(),
            Type::Any => expr.to_string(),
        }
    }
}

pub struct Transpiler {
    pub(crate) output: String,
    pub(crate) defs_output: String,
    pub(crate) stmts_output: String,
    pub(crate) indent_level: usize,
    pub used_vars: HashSet<String>,
    pub(crate) function_names: HashSet<String>,
    pub(crate) class_methods: BTreeMap<String, String>,
    pub(crate) imported_modules: Vec<String>,
    pub(crate) use_pyqt6: bool,
    pub(crate) use_sdl2: bool,
    pub(crate) use_opengl: bool,
    pub(crate) use_gtk4: bool,
    pub(crate) qt_objects: HashSet<String>,
    pub(crate) sdl_objects: HashSet<String>,
    pub(crate) gl_objects: HashSet<String>,
    pub(crate) gtk_objects: HashSet<String>,
    pub(crate) type_map: HashMap<String, Type>,
    pub(crate) is_generator: bool,
}

impl Transpiler {
    pub fn new() -> Self {
        Transpiler {
            output: String::new(),
            defs_output: String::new(),
            stmts_output: String::new(),
            indent_level: 0,
            used_vars: HashSet::new(),
            function_names: HashSet::new(),
            class_methods: BTreeMap::new(),
            imported_modules: Vec::new(),
            use_pyqt6: false,
            use_sdl2: false,
            use_opengl: false,
            use_gtk4: false,
            qt_objects: HashSet::new(),
            sdl_objects: HashSet::new(),
            gl_objects: HashSet::new(),
            gtk_objects: HashSet::new(),
            type_map: HashMap::new(),
            is_generator: false,
        }
    }

    pub fn transpile(&mut self, program: &Program) -> (String, String) {
        self.output.clear();
        self.defs_output.clear();
        self.stmts_output.clear();

        for stmt in &program.stmts {
            match stmt {
                Stmt::FunctionDef { .. } | Stmt::ClassDef { .. }
                | Stmt::Import(_) | Stmt::ImportFrom { .. } => {
                    self.transpile_stmt(stmt);
                    self.defs_output.push_str(&self.output);
                    self.output.clear();
                }
                _ => {
                    self.transpile_stmt(stmt);
                    self.stmts_output.push_str(&self.output);
                    self.output.clear();
                }
            }
        }

        let mut defs = String::new();
        defs.push_str(&self.defs_output);

        (defs, self.stmts_output.clone())
    }

    pub(crate) fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub(crate) fn writeln(&mut self, s: &str) {
        self.output.push_str(&format!("{}{}\n", self.indent(), s));
    }

    pub(crate) fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    pub(crate) fn write_indent(&mut self) {
        self.output.push_str(&self.indent());
    }

    pub(crate) fn transpile_block(&mut self, stmts: &[Stmt]) {
        self.indent_level += 1;
        for stmt in stmts {
            self.transpile_stmt(stmt);
        }
        self.indent_level -= 1;
    }

    pub(crate) fn transpile_moved(&mut self, expr: &Expr) {
        match expr {
            Expr::Name(s) => self.write(&format!("{}.clone()", s)),
            _ => self.transpile_expr(expr),
        }
    }

    pub(crate) fn has_yield_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Yield(_) => true,
            Expr::NamedExpr(_, v) | Expr::Await(v) | Expr::UnaryOp(_, v) | Expr::Starred(v) => self.has_yield_expr(v),
            Expr::BinOp(l, _, r) => self.has_yield_expr(l) || self.has_yield_expr(r),
            Expr::Compare(l, _, r) => self.has_yield_expr(l) || r.iter().any(|e| self.has_yield_expr(e)),
            Expr::Call(f, a, k) => {
                self.has_yield_expr(f)
                    || a.iter().any(|e| self.has_yield_expr(e))
                    || k.iter().any(|(_, e)| self.has_yield_expr(e))
            }
            Expr::Attribute(o, _) => self.has_yield_expr(o),
            Expr::Subscript(o, i) => self.has_yield_expr(o) || self.has_yield_expr(i),
            Expr::Slice(s, t, u) => {
                s.as_ref().map_or(false, |e| self.has_yield_expr(e))
                    || t.as_ref().map_or(false, |e| self.has_yield_expr(e))
                    || u.as_ref().map_or(false, |e| self.has_yield_expr(e))
            }
            Expr::IfExpr(t, a, b) => self.has_yield_expr(t) || self.has_yield_expr(a) || self.has_yield_expr(b),
            Expr::List(v) | Expr::Tuple(v) | Expr::Set(v) => v.iter().any(|e| self.has_yield_expr(e)),
            Expr::Dict(v) => v.iter().any(|(k, val)| self.has_yield_expr(k) || self.has_yield_expr(val)),
            Expr::ListComp(e, t, i, f) | Expr::SetComp(e, t, i, f) | Expr::Generator(e, t, i, f) => {
                self.has_yield_expr(e) || self.has_yield_expr(t) || self.has_yield_expr(i) || f.iter().any(|c| self.has_yield_expr(c))
            }
            Expr::DictComp(k, v, t, i, f) => {
                self.has_yield_expr(k) || self.has_yield_expr(v) || self.has_yield_expr(t) || self.has_yield_expr(i) || f.iter().any(|c| self.has_yield_expr(c))
            }
            Expr::JoinedStr(parts) => parts.iter().any(|e| self.has_yield_expr(e)),
            Expr::Lambda(a, b) => self.has_yield_expr(b),
            _ => false,
        }
    }

    pub(crate) fn has_yield_stmt(&self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::ExprStmt(e) | Stmt::Return(Some(e)) => self.has_yield_expr(e),
            Stmt::Assign(_, v) | Stmt::AnnAssign(_, _, Some(v)) => self.has_yield_expr(v),
            Stmt::AugAssign(_, _, v) => self.has_yield_expr(v),
            Stmt::For { target, iter, body, orelse } => self.has_yield_expr(target) || self.has_yield_expr(iter) || body.iter().any(|s| self.has_yield_stmt(s)) || orelse.iter().any(|s| self.has_yield_stmt(s)),
            Stmt::While { test, body, orelse } => self.has_yield_expr(test) || body.iter().any(|s| self.has_yield_stmt(s)) || orelse.iter().any(|s| self.has_yield_stmt(s)),
            Stmt::If { test, body, orelse } => self.has_yield_expr(test) || self.has_yield_block(body) || self.has_yield_block(orelse),
            Stmt::With { items, body } => items.iter().any(|(e, _)| self.has_yield_expr(e)) || body.iter().any(|s| self.has_yield_stmt(s)),
            Stmt::Try { body, handlers, orelse, finalbody } => {
                self.has_yield_block(body)
                    || handlers.iter().any(|(_, _, b)| self.has_yield_block(b))
                    || self.has_yield_block(orelse)
                    || self.has_yield_block(finalbody)
            }
            Stmt::FunctionDef { body, .. } => self.has_yield_block(body),
            Stmt::Raise(Some(e), _) | Stmt::Assert(e, _) => self.has_yield_expr(e),
            Stmt::Delete(v) => v.iter().any(|e| self.has_yield_expr(e)),
            _ => false,
        }
    }

    pub(crate) fn has_yield_block(&self, stmts: &[Stmt]) -> bool {
        stmts.iter().any(|s| self.has_yield_stmt(s))
    }

    pub(crate) fn transpile_range_arg(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLiteral(n) => self.write(&format!("{}usize", n)),
            _ => {
                self.transpile_expr(expr);
                self.write(".to_int() as usize");
            }
        }
    }
}

mod type_infer;
mod stmt;
mod expr;
mod exec_eval;
mod ffi;
