#![allow(unused_variables)]

use crate::ast::*;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub const STDLIB_MODULES: &[&str] = &["math", "sys", "os", "json", "time", "PyQt6", "sqlite3", "sdl2", "opengl", "vulkan", "lvgl", "re", "random", "collections", "itertools", "functools", "pathlib", "datetime", "typing", "abc", "urllib", "image", "git", "gi", "ffmpeg", "font", "torch", "numpy", "cuda"];

pub fn value_method_name(py_name: &str) -> &str {
    // Map Python method names to Rust Value method names
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

// Methods that mutate self (transpiler should NOT clone receiver)
pub const MUTABLE_VALUE_METHODS: &[&str] = &[
    "append", "pop", "extend", "insert", "remove", "clear", "sort", "reverse",
    "update", "setdefault", "add", "discard",
];

// Methods that take Vec<Value> instead of individual Value args
pub const VEC_ARGS_VALUE_METHODS: &[&str] = &[
    "pop", "insert", "get", "setdefault", "split", "replace", "center", "ljust", "rjust",
];

pub struct Transpiler {
    output: String,
    defs_output: String,
    stmts_output: String,
    indent_level: usize,
    used_vars: HashSet<String>,
    function_names: HashSet<String>,
    class_methods: BTreeMap<String, String>, 
    imported_modules: Vec<String>,
    use_pyqt6: bool,
    use_sdl2: bool,
    use_opengl: bool,
    use_gtk4: bool,
    qt_objects: HashSet<String>,
    sdl_objects: HashSet<String>,
    gl_objects: HashSet<String>,
    gtk_objects: HashSet<String>,
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

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn writeln(&mut self, s: &str) {
        self.output.push_str(&format!("{}{}\n", self.indent(), s));
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn write_indent(&mut self) {
        self.output.push_str(&self.indent());
    }

    fn transpile_block(&mut self, stmts: &[Stmt]) {
        self.indent_level += 1;
        for stmt in stmts {
            self.transpile_stmt(stmt);
        }
        self.indent_level -= 1;
    }

    fn transpile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunctionDef { name, args, vararg, kwarg, body, decorators, returns, is_async } => {
                self.transpile_func_def(name, args, vararg, kwarg, body, decorators, returns, *is_async);
            }
            Stmt::ClassDef { name, bases, body, decorators } => {
                self.transpile_class_def(name, bases, body, decorators);
            }
            Stmt::Return(value) => {
                self.write_indent();
                self.write("return ");
                if let Some(v) = value {
                    self.transpile_expr(v);
                } else {
                    self.write("Value::None");
                }
                self.writeln(";");
            }
            Stmt::Delete(targets) => {
                for target in targets {
                    self.write_indent();
                    if let Expr::Name(n) = target {
                        self.write(&format!("{} = Value::None; // deleted", n));
                    } else if let Expr::Subscript(obj, idx) = target {
                        self.write("// del ");
                        self.transpile_expr(obj);
                        self.write("[");
                        self.transpile_expr(idx);
                        self.writeln("]");
                    }
                    self.writeln("");
                }
            }
            Stmt::Assign(targets, value) => {
                if targets.len() == 1 {
                    let target = &targets[0];
                    
                    if let Expr::Tuple(items) = target {
                        
                        self.write_indent();
                        self.write("let mut __tmp = ");
                        self.transpile_expr(value);
                        self.writeln(";");
                        for (i, item) in items.iter().enumerate() {
                            self.write_indent();
                            if let Expr::Name(n) = item {
                                if !self.used_vars.contains(n.as_str()) {
                                    self.used_vars.insert(n.clone());
                                    self.write("let mut ");
                                }
                                self.write(&format!("{} = __tmp.index(&Value::Int({}));", n, i));
                            }
                            self.writeln("");
                        }
                    } else {
                        self.write_indent();
                        if let Expr::Name(n) = target {
                            if !self.used_vars.contains(n.as_str()) {
                                self.used_vars.insert(n.clone());
                                self.write("let mut ");
                            }
                            self.write(&format!("{} = ", n));
                            self.transpile_expr(value);
                            self.writeln(";");
                        } else if let Expr::Subscript(obj, idx) = target {
                            self.transpile_expr(obj);
                            self.write("[");
                            self.transpile_expr(idx);
                            self.write("] = ");
                            self.transpile_expr(value);
                            self.writeln(";");
                    } else if let Expr::Attribute(obj, attr) = target {
                        self.transpile_expr(obj);
                        self.write(&format!(".set_attr(\"{}\", ", attr));
                        self.transpile_moved(value);
                        self.writeln(");");
                        } else {
                            self.transpile_expr(target);
                            self.write(" = ");
                            self.transpile_expr(value);
                            self.writeln(";");
                        }
                    }
                } else {
                    
                    self.write_indent();
                    self.write("let mut __tmp = ");
                    self.transpile_expr(value);
                    self.writeln(";");
                    for (i, target) in targets.iter().enumerate() {
                        self.write_indent();
                        if let Expr::Name(n) = target {
                            if !self.used_vars.contains(n.as_str()) {
                                self.used_vars.insert(n.clone());
                                self.write("let mut ");
                            }
                            self.write(&format!("{} = __tmp.index(&Value::Int({}));", n, i));
                        }
                        self.writeln("");
                    }
                }
            }
            Stmt::AnnAssign(target, _ann, value) => {
                if let Some(v) = value {
                    self.write_indent();
                    if let Expr::Name(n) = target {
                        if !self.used_vars.contains(n.as_str()) {
                            self.used_vars.insert(n.clone());
                            self.write("let mut ");
                        }
                        self.write(&format!("{} = ", n));
                        self.transpile_expr(v);
                        self.writeln(";");
                    } else {
                        self.transpile_expr(target);
                        self.write(" = ");
                        self.transpile_expr(v);
                        self.writeln(";");
                    }
                }
            }
            Stmt::AugAssign(target, op, value) => {
                self.write_indent();
                if let Expr::Name(n) = target {
                    self.write(n);
                    match op {
                        BinOpKind::Add => self.write(" += "),
                        BinOpKind::Sub => self.write(" -= "),
                        BinOpKind::Mul => self.write(" *= "),
                        BinOpKind::Div => self.write(" /= "),
                        BinOpKind::FloorDiv => self.write(" = "),
                        BinOpKind::Mod => self.write(" %= "),
                        BinOpKind::Pow => self.write(" = "),
                        _ => self.write(" = "),
                    }
                    if matches!(op, BinOpKind::FloorDiv | BinOpKind::Pow) {
                        self.write(&format!("{}", n));
                        match op {
                            BinOpKind::FloorDiv => self.write(".floordiv(&"),
                            BinOpKind::Pow => self.write(".pow("),
                            _ => unreachable!(),
                        }
                        self.transpile_expr(value);
                        self.write(")");
                    } else {
                        self.transpile_expr(value);
                    }
                    self.writeln(";");
                }
            }
            Stmt::For { target, iter, body, orelse } => {
                self.write_indent();
                if let Expr::Tuple(items) = target {
                    // Tuple destructuring: for a, b in iter => __tup = iter; let (a, b) = ...
                    let tmp = "__for_tup";
                    self.write(&format!("for {} in ", tmp));
                    self.transpile_expr(iter);
                    self.writeln(" {");
                    for (i, item) in items.iter().enumerate() {
                        self.write_indent();
                        if let Expr::Name(n) = item {
                            if !self.used_vars.contains(n.as_str()) {
                                self.used_vars.insert(n.clone());
                                self.write(&format!("let mut {} = ", n));
                            } else {
                                self.write(&format!("{} = ", n));
                            }
                            self.write(&format!("{}.index(&Value::Int({}));", tmp, i));
                            self.writeln("");
                        }
                    }
                } else {
                    if let Expr::Name(n) = target {
                        if !self.used_vars.contains(n.as_str()) {
                            self.used_vars.insert(n.clone());
                        }
                    }
                    self.write("for ");
                    self.transpile_expr(target);
                    self.write(" in ");
                    self.transpile_expr(iter);
                    self.writeln(" {");
                }
                self.transpile_block(body);
                self.write_indent();
                self.writeln("}");
                if !orelse.is_empty() {
                    self.writeln("// else clause (not supported in Rust)");
                }
            }
            Stmt::While { test, body, orelse } => {
                self.write_indent();
                self.write("while ");
                self.transpile_expr(test);
                self.writeln(".to_bool() {");
                self.transpile_block(body);
                self.write_indent();
                self.writeln("}");
                if !orelse.is_empty() {
                    self.writeln("// else clause (not supported in Rust)");
                }
            }
            Stmt::If { test, body, orelse } => {
                self.write_indent();
                self.write("if ");
                self.transpile_expr(test);
                self.writeln(".to_bool() {");
                self.transpile_block(body);
                if !orelse.is_empty() {
                    
                    if orelse.len() == 1 && matches!(orelse[0], Stmt::If { .. }) {
                        self.write_indent();
                        self.writeln("} else ");
                        self.transpile_stmt(&orelse[0]);
                    } else {
                        self.write_indent();
                        self.writeln("} else {");
                        self.transpile_block(orelse);
                        self.write_indent();
                        self.writeln("}");
                    }
                } else {
                    self.write_indent();
                    self.writeln("}");
                }
            }
            Stmt::With { items, body } => {
                self.writeln("// with statement (not fully supported in Rust)");
                self.transpile_block(body);
            }
            Stmt::Try { body, handlers, orelse, finalbody } => {
                self.write_indent();
                self.writeln("let __prev_hook = std::panic::take_hook();");
                self.write_indent();
                self.writeln("std::panic::set_hook(Box::new(|_| {}));");
                self.write_indent();
                self.writeln("let __try_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {");
                self.transpile_block(body);
                self.write_indent();
                self.writeln("}));");
                self.write_indent();
                self.writeln("std::panic::set_hook(__prev_hook);");
                self.write_indent();
                self.writeln("match __try_result {");
                self.write_indent();
                self.writeln("Ok(_) => {},");
                if handlers.is_empty() {
                    self.write_indent();
                    self.writeln("Err(_) => {},");
                }
                for (i, (exc_type, exc_var, handler_body)) in handlers.iter().enumerate() {
                    self.write_indent();
                    if i == 0 {
                        self.write("Err(__e");
                        if let Some(var) = exc_var {
                            self.write(&format!(") => {{ let mut {} = if let Some(s) = __e.downcast_ref::<&str>() {{ Value::Str(s.to_string()) }} else if let Some(s) = __e.downcast_ref::<String>() {{ Value::Str(s.clone()) }} else {{ Value::Str(\"exception\".to_string()) }};", var));
                        } else {
                            self.write(") => {");
                        }
                    } else {
                        self.write("_ => {");
                    }
                    self.writeln("");
                    self.transpile_block(handler_body);
                    self.write_indent();
                    self.writeln("}");
                }
                self.write_indent();
                self.writeln("}");
                if !orelse.is_empty() {
                    self.write_indent();
                    self.writeln("// else clause (try with else)");
                }
                if !finalbody.is_empty() {
                    self.write_indent();
                    self.writeln("// finally block");
                    self.transpile_block(finalbody);
                }
            }
            Stmt::Raise(exc, cause) => {
                self.write_indent();
                self.write("panic!(");
                if let Some(e) = exc {
                    self.write("\"raised: ");
                    self.transpile_expr(e);
                    self.write("\"");
                } else {
                    self.write("\"raised exception\"");
                }
                self.writeln(");");
            }
            Stmt::Assert(test, msg) => {
                self.write_indent();
                self.write("assert!(");
                self.transpile_expr(test);
                self.write(".to_bool()");
                if let Some(m) = msg {
                    self.write(", ");
                    self.transpile_expr(m);
                }
                self.writeln(");");
            }
            Stmt::Import(names) => {
                for (name, alias) in names {
                    let import_name = alias.as_deref().unwrap_or(name);
                    if !self.imported_modules.contains(name) {
                        self.imported_modules.push(name.clone());
                    }
                    if STDLIB_MODULES.contains(&name.as_str()) {
                        
                    } else {
                        
                    }
                }
            }
            Stmt::ImportFrom { module, names, level } => {
                if let Some(mod_name) = module {
                    if mod_name == "PyQt6" {
                        self.use_pyqt6 = true;
                        let pyqt = "PyQt6".to_string();
                        if !self.imported_modules.contains(&pyqt) {
                            self.imported_modules.push(pyqt);
                        }
                        self.writeln("use wrappers::qt6::construct;");
                        self.writeln("use wrappers::qt6::method;");
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.qt_objects.insert(import_name.to_string());
                        }
                    } else if mod_name == "sdl2" {
                        self.use_sdl2 = true;
                        let sdl2 = "sdl2".to_string();
                        if !self.imported_modules.contains(&sdl2) {
                            self.imported_modules.push(sdl2);
                        }
                        self.writeln("use wrappers::sdl2::construct;");
                        self.writeln("use wrappers::sdl2::method;");
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.sdl_objects.insert(import_name.to_string());
                        }
                    } else if mod_name == "opengl" {
                        self.use_opengl = true;
                        let opengl = "opengl".to_string();
                        if !self.imported_modules.contains(&opengl) {
                            self.imported_modules.push(opengl);
                        }
                        self.writeln("use wrappers::opengl::construct;");
                        self.writeln("use wrappers::opengl::method;");
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.gl_objects.insert(import_name.to_string());
                        }
                    } else if mod_name == "gi.repository" || mod_name == "gi" {
                        self.use_gtk4 = true;
                        let gi = "gi".to_string();
                        if !self.imported_modules.contains(&gi) {
                            self.imported_modules.push(gi);
                        }
                        self.writeln("use wrappers::gtk4::construct;");
                        self.writeln("use wrappers::gtk4::method;");
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.gtk_objects.insert(import_name.to_string());
                        }
                    } else if mod_name == "vulkan" {
                        if !self.imported_modules.contains(mod_name) {
                            self.imported_modules.push(mod_name.clone());
                        }
                        if STDLIB_MODULES.contains(&mod_name.as_str()) {
                            for (name, alias) in names {
                                self.writeln(&format!("use {}::{};", mod_name, name));
                            }
                        } else {
                            for (name, alias) in names {
                                self.writeln(&format!("use {}::{};", mod_name, name));
                            }
                        }
                    } else if mod_name == "urllib" {
                        if !self.imported_modules.contains(mod_name) {
                            self.imported_modules.push(mod_name.clone());
                        }
                        if STDLIB_MODULES.contains(&mod_name.as_str()) {
                            for (name, alias) in names {
                                self.writeln(&format!("use {}::{};", mod_name, name));
                            }
                        } else {
                            for (name, alias) in names {
                                self.writeln(&format!("use {}::{};", mod_name, name));
                            }
                        }
                    } else if mod_name == "numpy" {
                        if !self.imported_modules.contains(mod_name) {
                            self.imported_modules.push(mod_name.clone());
                        }
                        for (name, alias) in names {
                            self.writeln(&format!("use {}::{};", mod_name, name));
                        }
                    } else if mod_name == "cuda" {
                        if !self.imported_modules.contains(mod_name) {
                            self.imported_modules.push(mod_name.clone());
                        }
                        for (name, alias) in names {
                            self.writeln(&format!("use {}::{};", mod_name, name));
                        }
                    } else {
                        if !self.imported_modules.contains(mod_name) {
                            self.imported_modules.push(mod_name.clone());
                        }
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.writeln(&format!("use {}::{};", mod_name, import_name));
                        }
                    }
                } else {
                    self.writeln("// relative import (unsupported)");
                }
            }
            Stmt::Global(names) => {
                self.writeln(&format!("// global {}", names.join(", ")));
            }
            Stmt::Nonlocal(names) => {
                self.writeln(&format!("// nonlocal {}", names.join(", ")));
            }
            Stmt::ExprStmt(expr) => {
                self.write_indent();
                self.transpile_expr(expr);
                self.writeln(";");
            }
            Stmt::Pass => {
                self.writeln("// pass");
            }
            Stmt::Break => {
                self.write_indent();
                self.writeln("break;");
            }
            Stmt::Continue => {
                self.write_indent();
                self.writeln("continue;");
            }
        }
    }

    fn transpile_func_def(&mut self, name: &str, args: &[(String, Option<Expr>)], vararg: &Option<String>, kwarg: &Option<String>, body: &[Stmt], _decorators: &[Expr], _returns: &Option<Expr>, _is_async: bool) {
        self.function_names.insert(name.to_string());

        self.write_indent();
        self.write("pub fn ");
        self.write(name);
        self.write("(");

        let mut all_args = Vec::new();
        for (arg_name, _default) in args {
            all_args.push(arg_name.clone());
        }
        if let Some(v) = vararg {
            all_args.push(v.clone());
        }
        if let Some(kw) = kwarg {
            all_args.push(kw.clone());
        }

        for (i, arg_name) in all_args.iter().enumerate() {
            if i > 0 { self.write(", "); }
            if !self.used_vars.contains(arg_name.as_str()) {
                self.used_vars.insert(arg_name.clone());
            }
            self.write(arg_name);
            self.write(": Value");
        }

        self.writeln(") -> Value {");

        
        for (arg_name, default) in args {
            if let Some(default_val) = default {
                self.write_indent();
                self.write(&format!("let mut {} = if {} == Value::None {{ ", arg_name, arg_name));
                self.transpile_expr(default_val);
                self.writeln(&format!(" }} else {{ {} }};", arg_name));
            }
        }

        self.indent_level += 1;
        for stmt in body {
            self.transpile_stmt(stmt);
        }
        
        let has_return = body.iter().any(|s| matches!(s, Stmt::Return(_)));
        if !has_return {
            self.writeln("Value::None");
        }
        self.indent_level -= 1;

        self.write_indent();
        self.writeln("}");
        self.writeln("");
    }

    fn transpile_class_def(&mut self, name: &str, _bases: &[Expr], body: &[Stmt], _decorators: &[Expr]) {
        
        let mut methods = Vec::new();
        for stmt in body {
            if let Stmt::FunctionDef { name: mname, args, vararg, kwarg, body, decorators, returns, is_async } = stmt {
                methods.push((mname.clone(), args.clone(), vararg.clone(), kwarg.clone(), body.clone(), decorators.clone(), returns.clone(), is_async.clone()));
            }
        }

        
        self.writeln(&format!("// Class: {}", name));
            self.write(&format!("pub fn {}_new(", name));
        let init_method = methods.iter().find(|(n,_,_,_,_,_,_,_)| n == "__init__");
        if let Some((_, init_args, _, _, init_body, _, _, _)) = init_method {
            let skip_self = if init_args.first().map(|(n,_)| n.as_str()) == Some("self") { 1 } else { 0 };
            for (i, (arg_name, _default)) in init_args.iter().enumerate().skip(skip_self) {
                if i > skip_self { self.write(", "); }
                self.write(arg_name);
                self.write(": Value");
            }
        }
        self.writeln(") -> Value {");

        self.indent_level += 1;
        self.writeln("let mut obj = Value::Dict(BTreeMap::new());");

        if let Some((_, _, _, _, init_body, _, _, _)) = init_method {
            for stmt in init_body {
                if let Stmt::Assign(targets, value) = stmt {
                    if targets.len() == 1 {
                        if let Expr::Attribute(obj_expr, attr) = &targets[0] {
                            if let Expr::Name(n) = obj_expr.as_ref() {
                                if n == "self" {
                                    self.write_indent();
                                    self.write(&format!("obj.set_attr(\"{}\", ", attr));
                                    self.transpile_moved(value);
                                    self.writeln(");");
                                } else {
                                    self.transpile_stmt(stmt);
                                }
                            }
                        } else {
                            self.transpile_stmt(stmt);
                        }
                    } else {
                        self.transpile_stmt(stmt);
                    }
                } else {
                    self.transpile_stmt(stmt);
                }
            }
        }

        self.writeln("obj");
        self.indent_level -= 1;
        self.writeln("}");
        self.writeln("");

        
        for (mname, ..) in &methods {
            self.class_methods.insert(mname.clone(), name.to_string());
        }

        
        for (mname, m_args, _, _, m_body, _, _, _) in &methods {
            if mname == "__init__" {
                continue;
            }

            let has_self = m_args.first().map(|(n,_)| n.as_str()) == Some("self");

            self.write(&format!("pub fn {}_{}(", name, mname));
            let mut first_param = true;
            if has_self {
                self.write("mut self_: Value");
                first_param = false;
            }
            for (i, (arg_name, _default)) in m_args.iter().enumerate() {
                if has_self && i == 0 { continue; }
                if !first_param { self.write(", "); }
                first_param = false;
                self.write(arg_name);
                self.write(": Value");
            }
            self.writeln(") -> Value {");

            self.indent_level += 1;
            for stmt in m_body {
                self.transpile_stmt(stmt);
            }
            let has_return = m_body.iter().any(|s| matches!(s, Stmt::Return(_)));
            if !has_return {
                self.writeln("Value::None");
            }
            self.indent_level -= 1;

            self.writeln("}");
            self.writeln("");
        }
    }

    fn transpile_moved(&mut self, expr: &Expr) {
        match expr {
            Expr::Name(s) => self.write(&format!("{}.clone()", s)),
            _ => self.transpile_expr(expr),
        }
    }

    fn transpile_range_arg(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLiteral(n) => self.write(&format!("{}usize", n)),
            _ => {
                self.transpile_expr(expr);
                self.write(".to_int() as usize");
            }
        }
    }

    fn is_deep_qt_attr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Attribute(obj, _) => {
                match obj.as_ref() {
                    Expr::Attribute(_, _) => true,
                    Expr::Name(n) => self.qt_objects.contains(n) || n == "self",
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn is_deep_sdl_attr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Attribute(obj, _) => {
                match obj.as_ref() {
                    Expr::Attribute(_, _) => true,
                    Expr::Name(n) => self.sdl_objects.contains(n) || n == "self",
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn is_deep_gl_attr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Attribute(obj, _) => {
                match obj.as_ref() {
                    Expr::Attribute(_, _) => true,
                    Expr::Name(n) => self.gl_objects.contains(n) || n == "self",
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn transpile_deep_qt_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    fn transpile_deep_sdl_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    fn transpile_deep_gl_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    fn is_deep_gtk_attr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Attribute(obj, _) => {
                match obj.as_ref() {
                    Expr::Attribute(_, _) => true,
                    Expr::Name(n) => self.gtk_objects.contains(n) || n == "self",
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn is_gtk_construct(&self, expr: &Expr) -> bool {
        if let Expr::Attribute(obj, _) = expr {
            if let Expr::Name(n) = obj.as_ref() {
                return self.gtk_objects.contains(n);
            }
        }
        false
    }

    fn transpile_deep_gtk_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    fn transpile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::NoneLiteral => self.write("Value::None"),
            Expr::BoolLiteral(b) => {
                self.write(&format!("Value::Bool({})", b));
            }
            Expr::IntLiteral(n) => {
                self.write(&format!("Value::Int({})", n));
            }
            Expr::FloatLiteral(f) => {
                self.write(&format!("Value::Float({})", f));
            }
            Expr::StrLiteral(s) => {
                self.write(&format!("Value::Str(\"{}\".to_string())", s.replace('"', "\\\"").replace('\n', "\\n")));
            }
            Expr::Name(s) => {
                if s == "self" {
                    self.write("self_");
                } else {
                    self.write(s);
                }
            }
            Expr::List(items) => {
                self.write("Value::List(vec![");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.transpile_moved(item);
                }
                self.write("])");
            }
            Expr::Tuple(items) => {
                if items.is_empty() {
                    self.write("Value::Tuple(vec![])");
                } else {
                    self.write("Value::Tuple(vec![");
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 { self.write(", "); }
                        self.transpile_moved(item);
                    }
                self.write("])");
            }
            }
            Expr::Dict(items) => {
                self.write("Value::Dict(BTreeMap::from([");
                for (i, (key, val)) in items.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.write("(");
                    if let Expr::StrLiteral(s) = key {
                        self.write(&format!("\"{}\".to_string()", s.replace('"', "\\\"").replace('\n', "\\n")));
                    } else {
                        self.write("format!(\"{}\", ");
                        self.transpile_expr(key);
                        self.write(")");
                    }
                    self.write(", ");
                    self.transpile_moved(val);
                    self.write(")");
                }
                self.write("]))");
            }
            Expr::Set(items) => {
                self.write("Value::Set(vec![");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.transpile_moved(item);
                }
                self.write("])");
            }
            Expr::BinOp(left, op, right) => {
                match op {
                    BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Mod => {
                        self.transpile_moved(left);
                        self.write(&format!(" {} ", op));
                        self.transpile_moved(right);
                    }
                    BinOpKind::FloorDiv => {
                        self.transpile_moved(left);
                        self.write(".floordiv(");
                        self.transpile_moved(right);
                        self.write(")");
                    }
                    BinOpKind::Pow => {
                        self.write("(");
                        self.transpile_moved(left);
                        self.write(").pow(&");
                        self.transpile_expr(right);
                        self.write(")");
                    }
                    BinOpKind::LShift => {
                        self.transpile_moved(left);
                        self.write(".lshift(");
                        self.transpile_moved(right);
                        self.write(")");
                    }
                    BinOpKind::RShift => {
                        self.transpile_moved(left);
                        self.write(".rshift(");
                        self.transpile_moved(right);
                        self.write(")");
                    }
                    BinOpKind::BitOr => {
                        self.transpile_moved(left);
                        self.write(".bitor(");
                        self.transpile_moved(right);
                        self.write(")");
                    }
                    BinOpKind::BitXor => {
                        self.transpile_moved(left);
                        self.write(".bitxor(");
                        self.transpile_moved(right);
                        self.write(")");
                    }
                    BinOpKind::BitAnd => {
                        self.transpile_expr(left);
                        self.write(".bitand(");
                        self.transpile_expr(right);
                        self.write(")");
                    }
                    BinOpKind::MatMult => {
                        self.transpile_expr(left);
                        self.write(".matmult(");
                        self.transpile_expr(right);
                        self.write(")");
                    }
                }
            }
            Expr::UnaryOp(op, expr) => {
                match op {
                    UnaryOpKind::Not => {
                        self.write("!(");
                        self.transpile_expr(expr);
                        self.write(".to_bool())");
                    }
                    UnaryOpKind::Neg => {
                        self.write("(-");
                        self.transpile_expr(expr);
                        self.write(")");
                    }
                    UnaryOpKind::Pos => {
                        self.write("(+");
                        self.transpile_expr(expr);
                        self.write(")");
                    }
                    UnaryOpKind::Invert => {
                        self.write("(~");
                        self.transpile_expr(expr);
                        self.write(")");
                    }
                }
            }
            Expr::Compare(left, ops, comparators) => {
                for (i, (op, right)) in ops.iter().zip(comparators.iter()).enumerate() {
                    if i > 0 {
                        self.write(" && ");
                    }
                    match op {
                        CmpOp::Eq | CmpOp::NotEq | CmpOp::Lt | CmpOp::LtE | CmpOp::Gt | CmpOp::GtE => {
                            self.write("(");
                            self.transpile_expr(left);
                            self.write(")");
                        }
                        _ => { self.transpile_expr(left); }
                    }
                    match op {
                        CmpOp::Eq => self.write(".eq_val(&"),
                        CmpOp::NotEq => self.write(".ne_val(&"),
                        CmpOp::Lt => self.write(".lt_val(&"),
                        CmpOp::LtE => self.write(".le_val(&"),
                        CmpOp::Gt => self.write(".gt_val(&"),
                        CmpOp::GtE => self.write(".ge_val(&"),
                        CmpOp::Is => self.write(".is_("),
                        CmpOp::IsNot => self.write(".is_not("),
                        CmpOp::In => self.write(".is_in("),
                        CmpOp::NotIn => self.write(".not_in("),
                    }
                    self.transpile_expr(right);
                    self.write(")");
                }
            }
            Expr::Call(func, args, kwargs) => {
                
                if self.use_pyqt6 {
                    if let Expr::Attribute(obj, method) = func.as_ref() {
                        if method == "connect" {
                            self.write("method(&");
                            self.transpile_expr(obj);
                            self.write(", \"connect\", vec![])");
                            return;
                        }
                        if self.is_deep_qt_attr(func) {
                            self.write("method(&");
                            self.transpile_deep_qt_obj(func);
                            self.write(&format!(", \"{}\", vec![", method));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                    if let Expr::Name(n) = func.as_ref() {
                        if n.starts_with("Q") || self.qt_objects.contains(n) {
                            self.write(&format!("construct(\"{}\", vec![", n));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                }

                if self.use_sdl2 {
                    if let Expr::Attribute(obj, method) = func.as_ref() {
                        if self.is_deep_sdl_attr(func) {
                            self.write("method(&");
                            self.transpile_deep_sdl_obj(func);
                            self.write(&format!(", \"{}\", vec![", method));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                    if let Expr::Name(n) = func.as_ref() {
                        if self.sdl_objects.contains(n) {
                            self.write(&format!("construct(\"{}\", vec![", n));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                }

                if self.use_opengl {
                    if let Expr::Attribute(obj, method) = func.as_ref() {
                        if self.is_deep_gl_attr(func) {
                            self.write("method(&");
                            self.transpile_deep_gl_obj(func);
                            self.write(&format!(", \"{}\", vec![", method));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                    if let Expr::Name(n) = func.as_ref() {
                        if self.gl_objects.contains(n) {
                            self.write(&format!("construct(\"{}\", vec![", n));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                }

                if self.use_gtk4 {
                    if let Expr::Attribute(obj, method) = func.as_ref() {
                        if self.is_gtk_construct(func) {
                            self.write(&format!("construct(\"Gtk{}\", vec![", method));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                        if self.is_deep_gtk_attr(func) {
                            self.write("method(&");
                            self.transpile_deep_gtk_obj(func);
                            self.write(&format!(", \"{}\", vec![", method));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write("])");
                            return;
                        }
                    }
                }

                
                if let Expr::Attribute(obj, method) = func.as_ref() {
                    if let Expr::Name(module_name) = obj.as_ref() {
                        if self.imported_modules.contains(module_name) {
                            self.write(&format!("{}::{}(", module_name, method));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 { self.write(", "); }
                                self.transpile_moved(arg);
                            }
                            self.write(")");
                            return;
                        }
                    }
                    
                    if VALUE_METHODS.iter().any(|m| *m == method.as_str()) {
                        let is_mutable = MUTABLE_VALUE_METHODS.iter().any(|m| *m == method.as_str());
                        let is_vec_args = VEC_ARGS_VALUE_METHODS.iter().any(|m| *m == method.as_str());
                        if is_mutable {
                            self.transpile_expr(obj);
                        } else {
                            self.transpile_moved(obj);
                        }
                        self.write(&format!(".{}(", value_method_name(method)));
                        if is_vec_args {
                            self.write("vec![");
                        }
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_moved(arg);
                        }
                        if is_vec_args {
                            self.write("]");
                        }
                        self.write(")");
                        return;
                    }

                    if self.use_pyqt6 {
                        self.write("method(&");
                        self.transpile_expr(obj);
                        self.write(&format!(", \"{}\", vec![", method));
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_moved(arg);
                        }
                        self.write("])");
                        return;
                    }

                    if self.use_sdl2 {
                        self.write("method(&");
                        self.transpile_expr(obj);
                        self.write(&format!(", \"{}\", vec![", method));
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_moved(arg);
                        }
                        self.write("])");
                        return;
                    }

                    if self.use_opengl {
                        self.write("method(&");
                        self.transpile_expr(obj);
                        self.write(&format!(", \"{}\", vec![", method));
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_moved(arg);
                        }
                        self.write("])");
                        return;
                    }

                    if self.use_gtk4 {
                        self.write("method(&");
                        self.transpile_expr(obj);
                        self.write(&format!(", \"{}\", vec![", method));
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_moved(arg);
                        }
                        self.write("])");
                        return;
                    }

                    if let Some(class_name) = self.class_methods.get(method) {
                        self.write(&format!("{}_{}(", class_name, method));
                        self.transpile_moved(obj);
                        for (i, arg) in args.iter().enumerate() {
                            self.write(", ");
                            self.transpile_moved(arg);
                        }
                        self.write(")");
                        return;
                    }
                }
                if let Expr::Name(n) = func.as_ref() {
                if n == "print" {
                    if args.len() == 1 {
                        self.write("println!(\"{}\", ");
                        self.transpile_moved(&args[0]);
                        self.write(")");
                    } else {
                        let fmt = args.iter().map(|_| "{}").collect::<Vec<_>>().join(" ");
                        self.write(&format!("println!(\"{}\", ", fmt));
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_moved(arg);
                        }
                        self.write(")");
                    }
                    return;
                    } else if n == "len" {
                        if let Some(arg) = args.first() {
                            self.transpile_moved(arg);
                            self.write(".len_()");
                        }
                        return;
                    } else if n == "range" {
                        if args.len() == 3 {
                            let step_expr = &args[2];
                            self.write("std::iter::successors(Some(");
                            self.transpile_range_arg(&args[0]);
                            self.write("), |x| { let n = x + ");
                            self.transpile_range_arg(step_expr);
                            self.write("; if n < ");
                            self.transpile_range_arg(&args[1]);
                            self.write(" { Some(n) } else { None } }).map(|x| Value::Int(x as i64))");
                        } else {
                            self.write("(");
                            if args.len() == 1 {
                                self.write("0usize..");
                                self.transpile_range_arg(&args[0]);
                            } else {
                                self.transpile_range_arg(&args[0]);
                                self.write("..");
                                self.transpile_range_arg(&args[1]);
                            }
                            self.write(").map(|x| Value::Int(x as i64))");
                        }
                        return;
                    } else if n == "int" {
                        if let Some(arg) = args.first() {
                            self.write("Value::Int(");
                            self.transpile_moved(arg);
                            self.write(".to_int())");
                        }
                        return;
                    } else if n == "float" {
                        if let Some(arg) = args.first() {
                            self.write("Value::Float(");
                            self.transpile_moved(arg);
                            self.write(".to_float())");
                        }
                        return;
                    } else if n == "str" {
                        if let Some(arg) = args.first() {
                            self.write("Value::Str(");
                            self.transpile_moved(arg);
                            self.write(".to_string())");
                        }
                        return;
                    } else if n == "list" {
                        if let Some(arg) = args.first() {
                            self.write("Value::List(");
                            self.transpile_moved(arg);
                            self.write(".to_list())");
                        }
                        return;
                    } else if n == "type" {
                        if let Some(arg) = args.first() {
                            self.write("Value::Str(");
                            self.transpile_expr(arg);
                            self.write(".type_repr())");
                        }
                        return;
                    } else if n == "isinstance" {
                        self.write("({ let obj = ");
                        self.transpile_moved(&args[0]);
                        self.write("; Value::Bool(obj.type_name() == \"");
                        if args.len() > 1 {
                            if let Expr::Name(type_name) = &args[1] {
                                self.write(type_name);
                            } else {
                                self.write("int");
                            }
                        }
                        self.write("\") })");
                        return;
                    } else if n == "hasattr" {
                        self.write("Value::Bool(");
                        self.transpile_moved(&args[0]);
                        self.write(".get_attr(&format!(\"{}\", ");
                        if args.len() > 1 { self.transpile_moved(&args[1]); }
                        else { self.write("Value::Str(String::new())"); }
                        self.write(")) != Value::None)");
                        return;
                    } else if n == "getattr" {
                        self.write("({ let __o = ");
                        self.transpile_moved(&args[0]);
                        self.write("; let __n = format!(\"{}\", ");
                        if args.len() > 1 { self.transpile_moved(&args[1]); }
                        else { self.write("Value::Str(String::new())"); }
                        self.write("); let __v = __o.get_attr(&__n); if __v != Value::None { __v } else { ");
                        if args.len() > 2 { self.transpile_moved(&args[2]); }
                        else { self.write("Value::None"); }
                        self.write(" } })");
                        return;
                    } else if n == "abs" {
                        self.write("(match ");
                        self.transpile_moved(&args[0]);
                        self.write(" { Value::Int(i) => Value::Int(i.abs()), Value::Float(f) => Value::Float(f.abs()), v => v })");
                        return;
                    } else if n == "sum" {
                        self.write("(");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list().into_iter().fold(Value::Int(0), |a, b| a + b))");
                        return;
                    } else if n == "min" {
                        if args.len() == 1 {
                            self.write("(");
                            self.transpile_moved(&args[0]);
                            self.write(".to_list().into_iter().reduce(|a, b| if a < b { a } else { b }).unwrap_or(Value::None))");
                        } else if args.len() >= 2 {
                            self.write("({ let mut m = ");
                            self.transpile_moved(&args[0]);
                            for arg in &args[1..] {
                                self.write("; let v = ");
                                self.transpile_moved(arg);
                                self.write("; if v < m { m = v; }");
                            }
                            self.write("; m })");
                        }
                        return;
                    } else if n == "max" {
                        if args.len() == 1 {
                            self.write("(");
                            self.transpile_moved(&args[0]);
                            self.write(".to_list().into_iter().reduce(|a, b| if a > b { a } else { b }).unwrap_or(Value::None))");
                        } else if args.len() >= 2 {
                            self.write("({ let mut m = ");
                            self.transpile_moved(&args[0]);
                            for arg in &args[1..] {
                                self.write("; let v = ");
                                self.transpile_moved(arg);
                                self.write("; if v > m { m = v; }");
                            }
                            self.write("; m })");
                        }
                        return;
                    } else if n == "any" {
                        self.write("Value::Bool(");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list().into_iter().any(|x| x.to_bool()))");
                        return;
                    } else if n == "all" {
                        self.write("Value::Bool(");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list().into_iter().all(|x| x.to_bool()))");
                        return;
                    } else if n == "enumerate" {
                        self.write("(");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list().into_iter().enumerate().map(|(i, v)| Value::Tuple(vec![Value::Int(i as i64), v]))");
                        self.write(".collect::<Vec<Value>>())");
                        return;
                    } else if n == "zip" {
                        self.write("({ let __a = ");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list(); let __b = ");
                        if args.len() > 1 { self.transpile_moved(&args[1]); }
                        else { self.write("vec![]"); }
                        self.write(".to_list(); Value::List(__a.into_iter().zip(__b).map(|(x, y)| Value::Tuple(vec![x, y])).collect()) })");
                        return;
                    } else if n == "reversed" {
                        self.write("({ let __v = ");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list(); Value::Str(format!(\"<list_reverseiterator object at 0x{:x}>\", &__v as *const _ as usize)) })");
                        return;
                    } else if n == "sorted" {
                        self.write("({ let mut __v = ");
                        self.transpile_moved(&args[0]);
                        self.write(".to_list(); __v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less)); Value::List(__v) })");
                        return;
                    }
                }

                
                if let Expr::Name(n) = func.as_ref() {
                    if n.chars().next().map_or(false, |c| c.is_uppercase()) {
                        self.write(&format!("{}_new(", n));
                    } else {
                        self.transpile_expr(func);
                        self.write("(");
                    }
                } else {
                    self.transpile_expr(func);
                    self.write("(");
                }
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.transpile_moved(arg);
                }
                for (i, (k, v)) in kwargs.iter().enumerate() {
                    if !args.is_empty() || i > 0 { self.write(", "); }
                    self.write(&format!("{}: ", k));
                    self.transpile_moved(v);
                }
                self.write(")");
            }
            Expr::Attribute(obj, attr) => {
                if let Expr::Name(n) = obj.as_ref() {
                    if n == "self" {
                        
                        self.write(&format!("self_.get_attr(\"{}\")", attr));
                    } else {
                        self.write(&format!("{}.get_attr(\"{}\")", n, attr));
                    }
                } else {
                    self.transpile_expr(obj);
                    self.write(&format!(".get_attr(\"{}\")", attr));
                }
            }
            Expr::Subscript(obj, idx) => {
                self.transpile_expr(obj);
                self.write(".index(&(");
                self.transpile_expr(idx);
                self.write("))");
            }
            Expr::Slice(start, stop, step) => {
                self.write("Value::Slice(Box::new(");
                if let Some(s) = start {
                    self.transpile_expr(s);
                } else {
                    self.write("Value::None");
                }
                self.write("), Box::new(");
                if let Some(s) = stop {
                    self.transpile_expr(s);
                } else {
                    self.write("Value::None");
                }
                self.write("), Box::new(");
                if let Some(s) = step {
                    self.transpile_expr(s);
                } else {
                    self.write("Value::None");
                }
                self.write("))");
            }
            Expr::Lambda(args, body) => {
                self.write("move |");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.write(arg);
                    self.write(": Value");
                }
                self.write("| ");
                self.transpile_moved(body);
            }
            Expr::IfExpr(test, true_expr, false_expr) => {
                self.write("if ");
                self.transpile_expr(test);
                self.write(".to_bool() { ");
                self.transpile_expr(true_expr);
                self.write(" } else { ");
                self.transpile_expr(false_expr);
                self.write(" }");
            }
            Expr::ListComp(expr, target, ifs) => {
                self.write("// list comprehension: [");
                self.transpile_expr(expr);
                self.write(" for ");
                self.transpile_expr(target);
                self.write(" in ...]");
                if ifs.is_empty() {
                    self.write("Value::List(vec![])");
                } else {
                    
                    self.write("Value::List(vec![])");
                }
            }
            Expr::SetComp(expr, target, ifs) => {
                self.write("// set comprehension");
                self.write("Value::Set(vec![])");
            }
            Expr::DictComp(key, val, target, ifs) => {
                self.write("// dict comprehension");
                self.write("Value::Dict(BTreeMap::new())");
            }
            Expr::Generator(expr, target, ifs) => {
                self.write("// generator expression");
                self.write("Value::None");
            }
            Expr::Starred(expr) => {
                self.write("// *");
                self.transpile_expr(expr);
            }
            Expr::Yield(value) => {
                self.write("// yield");
                if let Some(v) = value {
                    self.transpile_expr(v);
                } else {
                    self.write("Value::None");
                }
            }
            Expr::Await(expr) => {
                self.transpile_expr(expr);
            }
            Expr::JoinedStr(parts) => {
                self.write("Value::Str(format!(");
                self.write("\"");
                for part in parts {
                    if let Expr::StrLiteral(s) = part {
                        let escaped = s.replace('{', "{{").replace('}', "}}");
                        self.write(&escaped);
                    } else {
                        self.write("{}");
                    }
                }
                self.write("\"");
                for part in parts {
                    if !matches!(part, Expr::StrLiteral(_)) {
                        self.write(", ");
                        self.transpile_moved(part);
                    }
                }
                self.write("))");
            }
            Expr::Ellipsis => {
                self.write("// ...");
            }
        }
    }
}
