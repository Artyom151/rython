use super::*;

impl super::Transpiler {
    pub(crate) fn transpile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::NoneLiteral => self.write("Value::None"),
            Expr::BoolLiteral(b) => {
                self.write(&format!("Value::Bool({})", b));
            }
            Expr::IntLiteral(n) => {
                self.write(&format!("Value::Int({})", n));
            }
            Expr::FloatLiteral(f) => {
                let s = format!("{}", f);
                if !s.contains('.') && !s.contains('e') && !s.contains('E') {
                    self.write(&format!("Value::Float({}.0)", s));
                } else {
                    self.write(&format!("Value::Float({})", s));
                }
            }
            Expr::StrLiteral(s) => {
                self.write(&format!("Value::Str(\"{}\".to_string())", s.replace('"', "\\\"").replace('\n', "\\n")));
            }
            Expr::Name(s) => {
                if s == "self" {
                    self.write("self_");
                } else if s == "__name__" {
                    self.write("Value::Str(\"__main__\".to_string())");
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
                        self.write("(");
                        self.transpile_moved(left);
                        self.write(&format!(" {} ", op));
                        self.transpile_moved(right);
                        self.write(")");
                    }
                    BinOpKind::FloorDiv => {
                        self.transpile_expr(left);
                        self.write(".floordiv(&");
                        self.transpile_expr(right);
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
                        self.transpile_expr(left);
                        self.write(".bitor(&");
                        self.transpile_expr(right);
                        self.write(")");
                    }
                    BinOpKind::BitXor => {
                        self.transpile_expr(left);
                        self.write(".bitxor(&");
                        self.transpile_expr(right);
                        self.write(")");
                    }
                    BinOpKind::BitAnd => {
                        self.transpile_expr(left);
                        self.write(".bitand(&");
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
                        self.write("Value::Bool(!");
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
                        CmpOp::Is => self.write(".is_(&"),
                        CmpOp::IsNot => self.write(".is_not(&"),
                        CmpOp::In => self.write(".is_in(&"),
                        CmpOp::NotIn => self.write(".not_in(&"),
                    }
                    self.transpile_expr(right);
                    self.write(")");
                }
            }
            Expr::Call(func, args, kwargs) => {
                
                // super() -> self_.clone() (single inheritance shortcut)
                if let Expr::Name(n) = func.as_ref() {
                    if n == "super" {
                        self.write("self_.clone()");
                        return;
                    }
                }
                
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
                if n == "main" {
                    self.write("__main__(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.write(", "); }
                        self.transpile_moved(arg);
                    }
                    self.write(")");
                    return;
                }
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
                    } else if n == "round" {
                        self.write("({ let __rv = "); self.transpile_moved(&args[0]);
                        if args.len() == 1 {
                            self.write("; let __rr = __rv.to_float().round()");
                        } else {
                            self.write("; let __rp = "); self.transpile_moved(&args[1]); self.write(".to_int()");
                            self.write("; let __rr = (__rv.to_float() * 10_f64.powi(__rp as i32)).round() / 10_f64.powi(__rp as i32)");
                        }
                        self.write("; Value::Float(__rr) })");
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
                    } else if n == "exec" {
                        if let Some(arg) = args.first() {
                            if let Expr::StrLiteral(s) = arg {
                                let exec_code = self.exec_string_literal(s);
                                self.write(&exec_code);
                            } else {
                                self.write("exec_runtime(");
                                self.transpile_moved(arg);
                                self.write(")");
                            }
                        }
                        return;
                    } else if n == "eval" {
                        if let Some(arg) = args.first() {
                            if let Expr::StrLiteral(s) = arg {
                                let eval_code = self.eval_string_literal(s);
                                self.write(&eval_code);
                            } else {
                                self.write("eval_runtime(");
                                self.transpile_moved(arg);
                                self.write(")");
                            }
                        } else {
                            self.write("Value::None");
                        }
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
                    } else if n == "Qt" {
                        self.write(&format!("__qt_value().get_attr(\"{}\")", attr));
                    } else if self.imported_modules.contains(n) {
                        if n == "sys" && attr == "argv" {
                            self.write("Value::List(vec![])");
                        } else if n == "math" && attr == "pi" {
                            self.write("Value::Float(std::f64::consts::PI)");
                        } else {
                            self.write(&format!("{}.get_attr(\"{}\")", n, attr));
                        }
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
            Expr::ListComp(expr, target, iter, ifs) => {
                self.write("Value::List({ let mut __comp = Vec::new(); for ");
                self.transpile_expr(target);
                self.write(" in ");
                self.transpile_expr(iter);
                self.write(" { ");
                if !ifs.is_empty() {
                    self.write("if ");
                    for (i, cond) in ifs.iter().enumerate() {
                        if i > 0 { self.write(" && "); }
                        self.transpile_expr(cond);
                        self.write(".to_bool()");
                    }
                    self.write(" { ");
                }
                self.write("__comp.push(");
                self.transpile_expr(expr);
                self.write("); ");
                if !ifs.is_empty() {
                    for _ in ifs {
                        self.write("} ");
                    }
                }
                self.write("} __comp })");
            }
            Expr::SetComp(expr, target, iter, ifs) => {
                self.write("Value::Set({ let mut __comp = Vec::new(); for ");
                self.transpile_expr(target);
                self.write(" in ");
                self.transpile_expr(iter);
                self.write(" { ");
                if !ifs.is_empty() {
                    self.write("if ");
                    for (i, cond) in ifs.iter().enumerate() {
                        if i > 0 { self.write(" && "); }
                        self.transpile_expr(cond);
                        self.write(".to_bool()");
                    }
                    self.write(" { ");
                }
                self.write("__comp.push(");
                self.transpile_expr(expr);
                self.write("); ");
                if !ifs.is_empty() {
                    for _ in ifs {
                        self.write("} ");
                    }
                }
                self.write("} __comp })");
            }
            Expr::DictComp(key, val, target, iter, ifs) => {
                self.write("Value::Dict({ let mut __comp = ::std::collections::BTreeMap::new(); for ");
                self.transpile_expr(target);
                self.write(" in ");
                self.transpile_expr(iter);
                self.write(" { ");
                if !ifs.is_empty() {
                    self.write("if ");
                    for (i, cond) in ifs.iter().enumerate() {
                        if i > 0 { self.write(" && "); }
                        self.transpile_expr(cond);
                        self.write(".to_bool()");
                    }
                    self.write(" { ");
                }
                self.write("__comp.insert(");
                self.transpile_expr(key);
                self.write(".to_string(), ");
                self.transpile_expr(val);
                self.write("); ");
                if !ifs.is_empty() {
                    for _ in ifs {
                        self.write("} ");
                    }
                }
                self.write("} __comp })");
            }
            Expr::Generator(expr, target, iter, ifs) => {
                self.write("Value::List({ let mut __comp = Vec::new(); for ");
                self.transpile_expr(target);
                self.write(" in ");
                self.transpile_expr(iter);
                self.write(" { ");
                if !ifs.is_empty() {
                    self.write("if ");
                    for (i, cond) in ifs.iter().enumerate() {
                        if i > 0 { self.write(" && "); }
                        self.transpile_expr(cond);
                        self.write(".to_bool()");
                    }
                    self.write(" { ");
                }
                self.write("__comp.push(");
                self.transpile_expr(expr);
                self.write("); ");
                if !ifs.is_empty() {
                    for _ in ifs {
                        self.write("} ");
                    }
                }
                self.write("} __comp })");
            }
            Expr::Starred(expr) => {
                self.write("// *");
                self.transpile_expr(expr);
            }
            Expr::Yield(value) => {
                if let Some(v) = value {
                    self.write("__yielded.push(");
                    self.transpile_moved(v);
                } else {
                    self.write("__yielded.push(Value::None");
                }
                self.write("); Value::None");
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
            Expr::NamedExpr(target, value) => {
                // target := value  ->  ({ let __val = value; target = __val.clone(); __val })
                // Requires target to be already declared as let mut.
                self.write("({ let __val = ");
                self.transpile_moved(value);
                self.write("; ");
                if let Expr::Name(n) = target.as_ref() {
                    self.write(&format!("{} = __val.clone()", n));
                }
                self.write("; __val })");
            }
            Expr::Ellipsis => {
                self.write("// ...");
            }
        }
    }
}
