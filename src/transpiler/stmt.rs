use super::*;

impl super::Transpiler {
    pub(crate) fn transpile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunctionDef { name, args, vararg, kwarg, body, decorators, returns, is_async } => {
                let adjusted_name = if name == "main" { "__main__" } else { name };
                self.transpile_func_def(adjusted_name, args, vararg, kwarg, body, decorators, returns, *is_async);
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
                            self.write(".set_item(&");
                            self.transpile_expr(idx);
                            self.write(", ");
                            self.transpile_expr(value);
                            self.writeln(");");
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
                // if __name__ == "__main__" -> execute body unconditionally
                let is_name_main = matches!(test, Expr::Compare(_, ops, _) if ops.len() == 1 && ops[0] == CmpOp::Eq)
                    && match test {
                        Expr::Compare(left, _, comparators) => {
                            (matches!(left.as_ref(), Expr::Name(n) if n == "__name__")
                                && matches!(comparators.first(), Some(Expr::StrLiteral(s)) if s == "__main__"))
                            || (matches!(left.as_ref(), Expr::StrLiteral(s) if s == "__main__")
                                && matches!(comparators.first(), Some(Expr::Name(n)) if n == "__name__"))
                        }
                        _ => false,
                    };
                if is_name_main {
                    self.transpile_block(body);
                    return;
                }
                
                // Hoist variable declarations from tuple destructuring across all branches
                fn collect_assign_names(stmts: &[Stmt]) -> Vec<String> {
                    let mut names = Vec::new();
                    for stmt in stmts {
                        match stmt {
                            Stmt::Assign(targets, _) => {
                                for target in targets {
                                    if let Expr::Tuple(items) = target {
                                        for item in items {
                                            if let Expr::Name(n) = item {
                                                names.push(n.clone());
                                            }
                                        }
                                    } else if let Expr::Name(n) = target {
                                        names.push(n.clone());
                                    }
                                }
                            }
                            Stmt::AnnAssign(Expr::Name(n), _, _) => {
                                names.push(n.clone());
                            }
                            _ => {}
                        }
                    }
                    names
                }
                for name in &collect_assign_names(body) {
                    if !self.used_vars.contains(name.as_str()) {
                        self.used_vars.insert(name.clone());
                        self.write_indent();
                        self.writeln(&format!("let mut {};", name));
                    }
                }
                for name in &collect_assign_names(orelse) {
                    if !self.used_vars.contains(name.as_str()) {
                        self.used_vars.insert(name.clone());
                        self.write_indent();
                        self.writeln(&format!("let mut {};", name));
                    }
                }
                
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
                        self.writeln("};");
                    }
                } else {
                    self.write_indent();
                    self.writeln("};");
                }
            }
            Stmt::With { items, body } => {
                // Generate context manager protocol: call __enter__ on entry, __exit__ on exit
                self.write_indent();
                self.writeln("// with statement (context manager protocol)");

                // Step 1: evaluate each context manager expression (cloned) and store in __cm{i}
                for (i, (expr, _)) in items.iter().enumerate() {
                    self.write_indent();
                    self.write(&format!("let __cm{} = ", i));
                    self.transpile_expr(expr);
                    self.write(".clone();");
                    self.writeln("");
                }

                // Step 2: call __enter__() on each context manager
                // If an 'as' var exists, bind the result to it; otherwise ignore it
                for (i, (_, as_var)) in items.iter().enumerate() {
                    match as_var {
                        Some(Expr::Name(var)) => {
                            if !self.used_vars.contains(var.as_str()) {
                                self.used_vars.insert(var.clone());
                                self.write_indent();
                                self.write(&format!("let mut {} = __cm{}.clone().__enter__();", var, i));
                                self.writeln("");
                            } else {
                                self.write_indent();
                                self.write(&format!("{} = __cm{}.clone().__enter__();", var, i));
                                self.writeln("");
                            }
                        }
                        Some(_) => {
                            // non-trivial as target — skip (shouldn't happen in real Python)
                        }
                        None => {
                            // No binding, just call __enter__ for side effects
                            self.write_indent();
                            self.write(&format!("__cm{}.clone().__enter__();", i));
                            self.writeln("");
                        }
                    }
                }

                // Step 3: wrap body in catch_unwind for __exit__ cleanup
                self.write_indent();
                self.writeln("let __with_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {");
                self.indent_level += 1;
                self.transpile_block(body);
                self.indent_level -= 1;
                self.write_indent();
                self.writeln("}));");

                // Step 4: call __exit__(None, None, None) on all context managers
                self.write_indent();
                self.writeln("match __with_result {");
                self.indent_level += 1;
                self.write_indent();
                self.writeln("Ok(_) => {");
                self.indent_level += 1;
                for (i, _) in items.iter().enumerate() {
                    self.write_indent();
                    self.write(&format!("__cm{}.clone().__exit__(Value::None, Value::None, Value::None);", i));
                    self.writeln("");
                }
                self.indent_level -= 1;
                self.write_indent();
                self.writeln("},");
                self.write_indent();
                self.writeln("Err(__e) => {");
                self.indent_level += 1;
                for (i, _) in items.iter().enumerate() {
                    self.write_indent();
                    self.write(&format!("__cm{}.clone().__exit__(Value::None, Value::None, Value::None);", i));
                    self.writeln("");
                }
                self.write_indent();
                self.writeln("std::panic::resume_unwind(__e);");
                self.indent_level -= 1;
                self.write_indent();
                self.writeln("}");
                self.indent_level -= 1;
                self.write_indent();
                self.writeln("}");
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
                self.writeln("Value::None");
                self.write_indent();
                self.writeln("}));");
                self.write_indent();
                self.writeln("std::panic::set_hook(__prev_hook);");
                self.write_indent();
                self.writeln("match __try_result {");
                self.indent_level += 1;
                self.write_indent();
                if !orelse.is_empty() {
                    self.write("Ok(_) => {");
                    self.writeln("");
                    self.transpile_block(orelse);
                    self.write_indent();
                    self.writeln("},");
                } else {
                    self.writeln("Ok(_) => {},");
                }
                if !handlers.is_empty() {
                    self.write_indent();
                    self.writeln("Err(__e) => {");
                    self.indent_level += 1;
                    self.write_indent();
                    self.writeln("let __exc_str = if let Some(s) = __e.downcast_ref::<&str>() { s.to_string() } else if let Some(s) = __e.downcast_ref::<String>() { s.clone() } else { String::new() };");
                    self.write_indent();
                    self.writeln("let __exc_type = __exc_str.split(':').next().unwrap_or(\"\").trim();");
                    for (i, (exc_type, exc_var, handler_body)) in handlers.iter().enumerate() {
                        self.write_indent();
                        if i == 0 { self.write("if "); } else { self.write("} else if "); }
                        if let Some(etype) = exc_type {
                            if let Expr::Name(n) = etype {
                                self.write(&format!("__exc_type == \"{}\"", n));
                            } else {
                                self.write("true");
                            }
                        } else {
                            self.write("true");
                        }
                        self.write(" {");
                        self.writeln("");
                        self.indent_level += 1;
                        if let Some(var) = exc_var {
                            self.write_indent();
                            self.write(&format!("let mut {} = Value::Str(__exc_str.clone());", var));
                            self.writeln("");
                        }
                        self.transpile_block(handler_body);
                        self.indent_level -= 1;
                        // Don't close here — the `} else if` / `} else` chain does it
                    }
                    self.write_indent();
                    self.writeln("} else { std::panic::resume_unwind(__e); }");
                    self.indent_level -= 1;
                    self.write_indent();
                    self.writeln("},");
                } else {
                    self.write_indent();
                    self.writeln("Err(_) => {},");
                }
                self.indent_level -= 1;
                self.write_indent();
                self.writeln("}");
                if !finalbody.is_empty() {
                    self.transpile_block(finalbody);
                }
            }
            Stmt::Raise(exc, _cause) => {
                self.write_indent();
                if let Some(e) = exc {
                    if let Expr::Call(func, args, _) = e {
                        if let Expr::Name(n) = func.as_ref() {
                            let is_exc = n == "Exception"
                                || n.ends_with("Error")
                                || n.ends_with("Warning")
                                || n == "BaseException"
                                || n.ends_with("Exception");
                            if is_exc && args.len() == 1 {
                                if let Expr::StrLiteral(s) = &args[0] {
                                    let escaped = s.replace('"', "\\\"");
                                    self.write(&format!("std::panic::panic_any(format!(\"{}: {}\"));", n, escaped));
                                } else {
                                    self.write(&format!("std::panic::panic_any(format!(\"{}: {{}}\", ", n));
                                    self.transpile_expr(&args[0]);
                                    self.write("));");
                                }
                                self.writeln("");
                                return;
                            }
                        }
                    }
                    self.write("std::panic::panic_any({ let __v = ");
                    self.transpile_expr(e);
                    self.write("; format!(\"{}: {}\", __v.repr(), __v) })");
                } else {
                    self.write("std::panic::panic_any(\"Exception: raised exception\".to_string())");
                }
                self.writeln(";");
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
                        let pyqt = "PyQt6".to_string();
                        if !self.imported_modules.contains(&pyqt) {
                            self.imported_modules.push(pyqt);
                        }
                        if !self.use_pyqt6 {
                            self.writeln("use wrappers::qt6::construct;");
                            self.writeln("use wrappers::qt6::method;");
                        }
                        self.use_pyqt6 = true;
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.qt_objects.insert(import_name.to_string());
                            if import_name == "Qt" {
                                self.writeln("// Qt namespace object");
                                self.writeln("fn __qt_value() -> Value { Value::Dict(BTreeMap::new()) }");
                            }
                        }
                    } else if mod_name == "sdl2" {
                        let sdl2 = "sdl2".to_string();
                        if !self.imported_modules.contains(&sdl2) {
                            self.imported_modules.push(sdl2);
                        }
                        if !self.use_sdl2 {
                            self.writeln("use wrappers::sdl2::construct;");
                            self.writeln("use wrappers::sdl2::method;");
                        }
                        self.use_sdl2 = true;
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.sdl_objects.insert(import_name.to_string());
                        }
                    } else if mod_name == "opengl" {
                        let opengl = "opengl".to_string();
                        if !self.imported_modules.contains(&opengl) {
                            self.imported_modules.push(opengl);
                        }
                        if !self.use_opengl {
                            self.writeln("use wrappers::opengl::construct;");
                            self.writeln("use wrappers::opengl::method;");
                        }
                        self.use_opengl = true;
                        for (name, alias) in names {
                            let import_name = alias.as_deref().unwrap_or(name);
                            self.gl_objects.insert(import_name.to_string());
                        }
                    } else if mod_name == "gi.repository" || mod_name == "gi" {
                        let gi = "gi".to_string();
                        if !self.imported_modules.contains(&gi) {
                            self.imported_modules.push(gi);
                        }
                        if !self.use_gtk4 {
                            self.writeln("use wrappers::gtk4::construct;");
                            self.writeln("use wrappers::gtk4::method;");
                        }
                        self.use_gtk4 = true;
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
                    } else if mod_name == "dataclasses" {
                        // handled by transpiler
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
                if let Expr::Call(func, args, _) = expr {
                    if let Expr::Name(n) = func.as_ref() {
                        if n == "exec" {
                            if let Some(arg) = args.first() {
                                if let Expr::StrLiteral(s) = arg {
                                    let exec_code = self.exec_string_literal(s);
                                    self.writeln(&exec_code.trim());
                                    return;
                                }
                            }
                        }
                    }
                }
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

    pub(crate) fn transpile_func_def(&mut self, name: &str, args: &[(String, Option<Expr>)], vararg: &Option<String>, kwarg: &Option<String>, body: &[Stmt], _decorators: &[Expr], _returns: &Option<Expr>, _is_async: bool) {
        self.function_names.insert(name.to_string());

        let saved_used_vars = self.used_vars.clone();
        let saved_type_map = self.type_map.clone();
        self.used_vars.clear();
        self.type_map.clear();

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

        let is_gen = self.has_yield_block(body);
        self.is_generator = is_gen;

        if is_gen {
            self.write_indent();
            self.writeln("let mut __yielded: Vec<Value> = Vec::new();");
        }

        
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
        
        if is_gen {
            self.write_indent();
            self.writeln("Value::List(__yielded)");
        } else {
            let has_return = body.iter().any(|s| matches!(s, Stmt::Return(_)));
            if !has_return {
                self.writeln("Value::None");
            }
        }
        self.indent_level -= 1;

        self.write_indent();
        self.writeln("}");
        self.writeln("");

        self.is_generator = false;
        self.used_vars = saved_used_vars;
        self.type_map = saved_type_map;
    }

    pub(crate) fn transpile_class_def(&mut self, name: &str, _bases: &[Expr], body: &[Stmt], decorators: &[Expr]) {
        
        let mut methods = Vec::new();
        let mut properties: Vec<String> = Vec::new();
        let mut is_dataclass = false;
        for d in decorators {
            if let Expr::Name(n) = d {
                if n == "dataclass" {
                    is_dataclass = true;
                }
            }
        }
        for stmt in body {
            if let Stmt::FunctionDef { name: mname, args, vararg, kwarg, body, decorators, returns, is_async } = stmt {
                methods.push((mname.clone(), args.clone(), vararg.clone(), kwarg.clone(), body.clone(), decorators.clone(), returns.clone(), is_async.clone()));
                for d in decorators {
                    if let Expr::Name(n) = d {
                        if n == "property" {
                            properties.push(mname.clone());
                        }
                    }
                }
            }
        }

        
        let mut dataclass_fields: Vec<(String, Option<Expr>)> = Vec::new();
        if is_dataclass {
            for stmt in body {
                if let Stmt::AnnAssign(Expr::Name(fname), _, default) = stmt {
                    dataclass_fields.push((fname.clone(), default.clone()));
                }
            }
        }

        
        self.writeln(&format!("// Class: {}", name));
            self.write(&format!("pub fn {}_new(", name));
        let init_method = methods.iter().find(|(n,_,_,_,_,_,_,_)| n == "__init__");
        if is_dataclass && init_method.is_none() {
            
            for (i, (fname, _default)) in dataclass_fields.iter().enumerate() {
                if i > 0 { self.write(", "); }
                self.write(fname);
                self.write(": Value");
            }
        }
        if let Some((_, init_args, _, _, _, _, _, _)) = init_method {
            let skip_self = if init_args.first().map(|(n,_)| n.as_str()) == Some("self") { 1 } else { 0 };
            for (i, (arg_name, _default)) in init_args.iter().enumerate().skip(skip_self) {
                if i > skip_self { self.write(", "); }
                self.write(arg_name);
                self.write(": Value");
            }
        }
        self.writeln(") -> Value {");

        self.indent_level += 1;
        self.writeln("let mut self_ = Value::Dict(BTreeMap::new());");

        if is_dataclass && init_method.is_none() {
            
            for (fname, default) in &dataclass_fields {
                self.write_indent();
                if let Some(def) = default {
                    self.write(&format!("self_.set_attr(\"{}\", if {}.clone() != Value::None {{ {}.clone() }} else {{ ", fname, fname, fname));
                    self.transpile_moved(def);
                    self.writeln(&format!(" }});"));
                } else {
                    self.write(&format!("self_.set_attr(\"{}\", {}.clone());", fname, fname));
                    self.writeln("");
                }
            }
        }
        if let Some((_, _, _, _, init_body, _, _, _)) = init_method {
            for stmt in init_body {
                if let Stmt::Assign(targets, value) = stmt {
                    if targets.len() == 1 {
                        if let Expr::Attribute(obj_expr, attr) = &targets[0] {
                            if let Expr::Name(n) = obj_expr.as_ref() {
                                if n == "self" {
                                    self.write_indent();
                                    self.write(&format!("self_.set_attr(\"{}\", ", attr));
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

        
        for prop in &properties {
            self.write_indent();
            self.write(&format!("self_.set_attr(\"{}\", {}_{}(self_.clone()));", prop, name, prop));
            self.writeln("");
        }

        self.writeln("self_");
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
            let saved_used_vars = self.used_vars.clone();
            self.used_vars.clear();
            let saved_type_map = self.type_map.clone();
            self.type_map.clear();

            let is_prop = properties.contains(mname);

            let has_self = m_args.first().map(|(n,_)| n.as_str()) == Some("self");

            // Register parameters in used_vars
            if has_self {
                self.used_vars.insert("self".to_string());
            }
            for (i, (arg_name, _default)) in m_args.iter().enumerate() {
                if has_self && i == 0 { continue; }
                self.used_vars.insert(arg_name.clone());
            }

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

            self.used_vars = saved_used_vars;
            self.type_map = saved_type_map;
        }

        
        if is_dataclass && !methods.iter().any(|(n,_,_,_,_,_,_,_)| n == "__repr__") && !dataclass_fields.is_empty() {
            let fnames: Vec<&str> = dataclass_fields.iter().map(|(n, _)| n.as_str()).collect();
            self.write(&format!("pub fn {}___repr__(self_: Value) -> Value {{", name));
            self.writeln("");
            self.indent_level += 1;
            self.write_indent();
            self.writeln("let mut __parts = Vec::new();");
            for fname in &fnames {
                self.write_indent();
                self.write(&format!("__parts.push(format!(\"{}={{:?}}\", self_.get_attr(\"{}\")));", fname, fname));
                self.writeln("");
            }
            self.write_indent();
            self.write(&format!("Value::Str(format!(\"{}({{}})\", __parts.join(\", \")))", name));
            self.writeln("");
            self.indent_level -= 1;
            self.writeln("}");
            self.writeln("");
            
            self.class_methods.insert("__repr__".to_string(), name.to_string());
        }

        if is_dataclass && !methods.iter().any(|(n,_,_,_,_,_,_,_)| n == "__eq__") && !dataclass_fields.is_empty() {
            self.write(&format!("pub fn {}___eq__(self_: Value, other: Value) -> Value {{", name));
            self.writeln("");
            self.indent_level += 1;
            for (i, (fname, _)) in dataclass_fields.iter().enumerate() {
                self.write_indent();
                if i == 0 {
                    self.write("if ");
                } else {
                    self.write("else if ");
                }
                self.write(&format!("self_.get_attr(\"{}\") != other.get_attr(\"{}\") {{ return Value::Bool(false); }}", fname, fname));
                self.writeln("");
            }
            self.write_indent();
            self.writeln("Value::Bool(true)");
            self.indent_level -= 1;
            self.writeln("}");
            self.writeln("");
            
            self.class_methods.insert("__eq__".to_string(), name.to_string());
        }
    }
}
