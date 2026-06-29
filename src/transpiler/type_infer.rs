#![allow(dead_code)]

use crate::ast::*;
use super::Type;

impl super::Transpiler {
    fn infer_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::NoneLiteral => Type::None,
            Expr::BoolLiteral(_) => Type::Bool,
            Expr::IntLiteral(_) => Type::Int,
            Expr::FloatLiteral(_) => Type::Float,
            Expr::StrLiteral(_) => Type::Str,
            Expr::Name(s) => {
                if s == "self" {
                    Type::Dict(Box::new(Type::Str), Box::new(Type::Any))
                } else if let Some(t) = self.type_map.get(s) {
                    t.clone()
                } else if s == "True" || s == "False" {
                    Type::Bool
                } else if s == "None" {
                    Type::None
                } else {
                    Type::Any
                }
            }
            Expr::List(items) => {
                if items.is_empty() {
                    Type::List(Box::new(Type::Any))
                } else {
                    Type::List(Box::new(self.infer_type(&items[0])))
                }
            }
            Expr::Tuple(items) => {
                Type::Tuple(items.iter().map(|i| self.infer_type(i)).collect())
            }
            Expr::Dict(items) => {
                if items.is_empty() {
                    Type::Dict(Box::new(Type::Str), Box::new(Type::Any))
                } else {
                    Type::Dict(Box::new(self.infer_type(&items[0].0)), Box::new(self.infer_type(&items[0].1)))
                }
            }
            Expr::Set(items) => {
                if items.is_empty() {
                    Type::Set(Box::new(Type::Any))
                } else {
                    Type::Set(Box::new(self.infer_type(&items[0])))
                }
            }
            Expr::BinOp(left, op, right) => {
                let lt = self.infer_type(left);
                let rt = self.infer_type(right);
                match op {
                    BinOpKind::Add => {
                        if lt == Type::Str || rt == Type::Str { Type::Str }
                        else if lt == Type::Float || rt == Type::Float { Type::Float }
                        else if lt == Type::Int && rt == Type::Int { Type::Int }
                        else { Type::Any }
                    }
                    BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Mod | BinOpKind::FloorDiv | BinOpKind::Pow => {
                        if lt == Type::Float || rt == Type::Float { Type::Float }
                        else if lt == Type::Int && rt == Type::Int { Type::Int }
                        else { Type::Any }
                    }
                    BinOpKind::LShift | BinOpKind::RShift | BinOpKind::BitOr | BinOpKind::BitXor | BinOpKind::BitAnd => {
                        if lt == Type::Int && rt == Type::Int { Type::Int }
                        else { Type::Any }
                    }
                    BinOpKind::MatMult => Type::Any,
                }
            }
            Expr::UnaryOp(op, e) => {
                match op {
                    UnaryOpKind::Not => Type::Bool,
                    UnaryOpKind::Neg | UnaryOpKind::Pos => self.infer_type(e),
                    UnaryOpKind::Invert => Type::Int,
                }
            }
            Expr::Compare(_, _, _) => Type::Bool,
            Expr::Call(func, args, _) => {
                if let Expr::Name(n) = func.as_ref() {
                    match n.as_str() {
                        "int" => Type::Int,
                        "float" => Type::Float,
                        "str" => Type::Str,
                        "bool" => Type::Bool,
                        "len" => Type::Int,
                        "abs" => if args.is_empty() { Type::Any } else { self.infer_type(&args[0]) },
                        "sum" => Type::Int,
                        "type" => Type::Str,
                        "isinstance" => Type::Bool,
                        "hasattr" => Type::Bool,
                        "list" => Type::List(Box::new(Type::Any)),
                        "range" => Type::Any, // range iterator
                        "enumerate" => Type::Any,
                        "zip" => Type::List(Box::new(Type::Tuple(vec![]))),
                        "sorted" => Type::List(Box::new(Type::Any)),
                        "reversed" => Type::Str, // returns a reverse iterator repr
                        _ => Type::Any,
                    }
                } else {
                    Type::Any
                }
            }
            Expr::Attribute(_, _) => Type::Any,
            Expr::Subscript(_, _) => Type::Any,
            Expr::Slice(_, _, _) => Type::Any,
            Expr::Lambda(_, _) => Type::Any,
            Expr::IfExpr(_, t, _) => self.infer_type(t),
            Expr::ListComp(expr, _, _, _) => Type::List(Box::new(self.infer_type(expr))),
            Expr::SetComp(expr, _, _, _) => Type::Set(Box::new(self.infer_type(expr))),
            Expr::DictComp(key, _, _, _, _) => Type::Dict(Box::new(self.infer_type(key)), Box::new(Type::Any)),
            Expr::Generator(_, _, _, _) => Type::List(Box::new(Type::Any)),
            Expr::Starred(e) => self.infer_type(e),
            Expr::Yield(_) => Type::Any,
            Expr::Await(e) => self.infer_type(e),
            Expr::NamedExpr(_, value) => self.infer_type(value),
            Expr::Ellipsis => Type::None,
            Expr::JoinedStr(_) => Type::Str,
        }
    }

    fn wrap_value(&self, ty: &Type, expr: &str) -> String {
        ty.wrap_in_value(expr)
    }

    fn unwrap_value(&self, ty: &Type, expr: &str) -> String {
        ty.unwrap_from_value(expr)
    }

    fn typed_name(&self, name: &str) -> String {
        if let Some(t) = self.type_map.get(name) {
            if *t != Type::Any {
                return format!("{}: {}", name, t.rust_type());
            }
        }
        format!("{}: Value", name)
    }
}
