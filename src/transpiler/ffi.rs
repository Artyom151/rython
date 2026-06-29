use crate::ast::*;

impl super::Transpiler {
    pub(crate) fn is_deep_qt_attr(&self, expr: &Expr) -> bool {
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

    pub(crate) fn is_deep_sdl_attr(&self, expr: &Expr) -> bool {
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

    pub(crate) fn is_deep_gl_attr(&self, expr: &Expr) -> bool {
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

    pub(crate) fn transpile_deep_qt_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    pub(crate) fn transpile_deep_sdl_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    pub(crate) fn transpile_deep_gl_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }

    pub(crate) fn is_deep_gtk_attr(&self, expr: &Expr) -> bool {
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

    pub(crate) fn is_gtk_construct(&self, expr: &Expr) -> bool {
        if let Expr::Attribute(obj, _) = expr {
            if let Expr::Name(n) = obj.as_ref() {
                return self.gtk_objects.contains(n);
            }
        }
        false
    }

    pub(crate) fn transpile_deep_gtk_obj(&mut self, expr: &Expr) {
        match expr {
            Expr::Attribute(obj, _) => {
                self.transpile_expr(obj);
            }
            _ => {
                self.transpile_expr(expr);
            }
        }
    }
}
