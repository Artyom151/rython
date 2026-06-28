use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    NoneLiteral,
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    StrLiteral(String),
    Name(String),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Set(Vec<Expr>),
    BinOp(Box<Expr>, BinOpKind, Box<Expr>),
    UnaryOp(UnaryOpKind, Box<Expr>),
    Compare(Box<Expr>, Vec<CmpOp>, Vec<Expr>),
    Call(Box<Expr>, Vec<Expr>, Vec<(String, Expr)>),
    Attribute(Box<Expr>, String),
    Subscript(Box<Expr>, Box<Expr>),
    Slice(Option<Box<Expr>>, Option<Box<Expr>>, Option<Box<Expr>>),
    Lambda(Vec<String>, Box<Expr>),
    IfExpr(Box<Expr>, Box<Expr>, Box<Expr>),
    ListComp(Box<Expr>, Box<Expr>, Vec<Box<Expr>>),
    SetComp(Box<Expr>, Box<Expr>, Vec<Box<Expr>>),
    DictComp(Box<Expr>, Box<Expr>, Box<Expr>, Vec<Box<Expr>>),
    Generator(Box<Expr>, Box<Expr>, Vec<Box<Expr>>),
    Starred(Box<Expr>),
    Yield(Option<Box<Expr>>),
    Await(Box<Expr>),
    Ellipsis,
    JoinedStr(Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, FloorDiv, Mod, Pow,
    LShift, RShift, BitOr, BitXor, BitAnd,
    MatMult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOpKind {
    Pos, Neg, Not, Invert,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CmpOp {
    Eq, NotEq, Lt, LtE, Gt, GtE, Is, IsNot, In, NotIn,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    FunctionDef {
        name: String,
        args: Vec<(String, Option<Expr>)>,
        vararg: Option<String>,
        kwarg: Option<String>,
        body: Vec<Stmt>,
        decorators: Vec<Expr>,
        returns: Option<Expr>,
        is_async: bool,
    },
    ClassDef {
        name: String,
        bases: Vec<Expr>,
        body: Vec<Stmt>,
        decorators: Vec<Expr>,
    },
    Return(Option<Expr>),
    Delete(Vec<Expr>),
    Assign(Vec<Expr>, Expr),
    AnnAssign(Expr, Expr, Option<Expr>),
    AugAssign(Expr, BinOpKind, Expr),
    For {
        target: Expr,
        iter: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    While {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    If {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },
    With {
        items: Vec<(Expr, Option<Expr>)>,
        body: Vec<Stmt>,
    },
    Try {
        body: Vec<Stmt>,
        handlers: Vec<(Option<Expr>, Option<String>, Vec<Stmt>)>,
        orelse: Vec<Stmt>,
        finalbody: Vec<Stmt>,
    },
    Raise(Option<Expr>, Option<Expr>),
    Assert(Expr, Option<Expr>),
    Import(Vec<(String, Option<String>)>),
    ImportFrom {
        module: Option<String>,
        names: Vec<(String, Option<String>)>,
        level: usize,
    },
    Global(Vec<String>),
    Nonlocal(Vec<String>),
    ExprStmt(Expr),
    Pass,
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOpKind::Add => write!(f, "+"),
            BinOpKind::Sub => write!(f, "-"),
            BinOpKind::Mul => write!(f, "*"),
            BinOpKind::Div => write!(f, "/"),
            BinOpKind::FloorDiv => write!(f, "//"),
            BinOpKind::Mod => write!(f, "%"),
            BinOpKind::Pow => write!(f, "**"),
            BinOpKind::LShift => write!(f, "<<"),
            BinOpKind::RShift => write!(f, ">>"),
            BinOpKind::BitOr => write!(f, "|"),
            BinOpKind::BitXor => write!(f, "^"),
            BinOpKind::BitAnd => write!(f, "&"),
            BinOpKind::MatMult => write!(f, "@"),
        }
    }
}

impl fmt::Display for UnaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOpKind::Pos => write!(f, "+"),
            UnaryOpKind::Neg => write!(f, "-"),
            UnaryOpKind::Not => write!(f, "not"),
            UnaryOpKind::Invert => write!(f, "~"),
        }
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CmpOp::Eq => write!(f, "=="),
            CmpOp::NotEq => write!(f, "!="),
            CmpOp::Lt => write!(f, "<"),
            CmpOp::LtE => write!(f, "<="),
            CmpOp::Gt => write!(f, ">"),
            CmpOp::GtE => write!(f, ">="),
            CmpOp::Is => write!(f, "is"),
            CmpOp::IsNot => write!(f, "is not"),
            CmpOp::In => write!(f, "in"),
            CmpOp::NotIn => write!(f, "not in"),
        }
    }
}
