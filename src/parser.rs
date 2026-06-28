#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    stmts_buffer: Vec<Stmt>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, stmts_buffer: Vec::new() }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }

    fn expect(&mut self, kind: &TokenKind) {
        if self.peek_kind() == Some(kind) {
            self.advance();
        } else {
            let found = self.peek().map(|t| t.kind.to_string()).unwrap_or("EOF".to_string());
            panic!("Expected {}, found {} at line {}", kind, found, self.peek().map(|t| t.line).unwrap_or(0));
        }
    }

    fn skip_newlines(&mut self) {
        while self.peek_kind() == Some(&TokenKind::Newline) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while self.peek_kind() != Some(&TokenKind::EOF) || !self.stmts_buffer.is_empty() {
            if !self.stmts_buffer.is_empty() {
                stmts.push(self.stmts_buffer.remove(0));
            } else {
                stmts.push(self.parse_stmt());
                self.skip_newlines();
            }
        }
        Program { stmts }
    }

    fn parse_stmt(&mut self) -> Stmt {
        if self.peek_kind().is_none() {
            panic!("Unexpected end of input");
        }

        match self.peek_kind().cloned() {
            Some(TokenKind::Def) => self.parse_func_def(),
            Some(TokenKind::Async) => self.parse_func_def(),
            Some(TokenKind::Class) => self.parse_class_def(),
            Some(TokenKind::If) => self.parse_if(),
            Some(TokenKind::For) => self.parse_for(),
            Some(TokenKind::While) => self.parse_while(),
            Some(TokenKind::Try) => self.parse_try(),
            Some(TokenKind::With) => self.parse_with(),
            Some(TokenKind::Return) => self.parse_return(),
            Some(TokenKind::Raise) => self.parse_raise(),
            Some(TokenKind::Assert) => self.parse_assert(),
            Some(TokenKind::Import) => self.parse_import(),
            Some(TokenKind::From) => self.parse_import_from(),
            Some(TokenKind::Global) => self.parse_global(),
            Some(TokenKind::Nonlocal) => self.parse_nonlocal(),
            Some(TokenKind::Del) => self.parse_del(),
            Some(TokenKind::Pass) => { self.advance(); Stmt::Pass },
            Some(TokenKind::Break) => { self.advance(); Stmt::Break },
            Some(TokenKind::Continue) => { self.advance(); Stmt::Continue },
            _ => {
                if self.peek_kind() == Some(&TokenKind::At) {
                    return self.parse_decorated();
                }
                self.parse_simple_stmt()
            }
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(&TokenKind::Colon);
        self.skip_newlines();

        let mut stmts = Vec::new();
        if self.peek_kind() == Some(&TokenKind::Indent) {
            self.advance();
            self.skip_newlines();
            while self.peek_kind() != Some(&TokenKind::Dedent) && self.peek_kind() != Some(&TokenKind::EOF) {
                stmts.push(self.parse_stmt());
                self.skip_newlines();
            }
            if self.peek_kind() == Some(&TokenKind::Dedent) {
                self.advance();
            }
        } else {
            stmts.push(self.parse_simple_stmt());
        }
        stmts
    }

    fn parse_decorated(&mut self) -> Stmt {
        let mut decorators = Vec::new();
        while self.peek_kind() == Some(&TokenKind::At) {
            self.advance();
            decorators.push(self.parse_expr());
            self.skip_newlines();
        }
        let mut stmt = self.parse_stmt();
        match &mut stmt {
            Stmt::FunctionDef { decorators: d, .. } | Stmt::ClassDef { decorators: d, .. } => {
                *d = decorators;
            }
            _ => panic!("Decorators can only be applied to functions and classes"),
        }
        stmt
    }

    fn parse_func_def(&mut self) -> Stmt {
        let is_async = self.peek_kind() == Some(&TokenKind::Async);
        if is_async { self.advance(); }
        self.expect(&TokenKind::Def);
        let name = match self.advance().kind.clone() {
            TokenKind::Name(n) => n,
            _ => panic!("Expected function name"),
        };
        self.expect(&TokenKind::LParen);
        let (args, vararg, kwarg) = self.parse_func_args();
        self.expect(&TokenKind::RParen);

        let returns = if self.peek_kind() == Some(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_expr())
        } else {
            None
        };

        let body = self.parse_block();
        Stmt::FunctionDef {
            name,
            args,
            vararg,
            kwarg,
            body,
            decorators: Vec::new(),
            returns,
            is_async,
        }
    }

    fn parse_func_args(&mut self) -> (Vec<(String, Option<Expr>)>, Option<String>, Option<String>) {
        let mut args = Vec::new();
        let mut vararg = None;
        let mut kwarg = None;

        loop {
            if self.peek_kind() == Some(&TokenKind::RParen) {
                break;
            }
            if matches!(self.peek_kind(), Some(TokenKind::Star)) {
                if matches!(self.peek_next_kind(), Some(TokenKind::Comma)) {
                    self.advance();
                    self.expect(&TokenKind::Comma);
                    continue;
                }
                self.advance();
                match self.advance().kind.clone() {
                    TokenKind::Name(n) => vararg = Some(n),
                    _ => panic!("Expected parameter name after *"),
                }
                if matches!(self.peek_kind(), Some(TokenKind::Comma)) {
                    self.advance();
                }
                continue;
            }
            if matches!(self.peek_kind(), Some(TokenKind::DoubleStar)) {
                self.advance();
                match self.advance().kind.clone() {
                    TokenKind::Name(n) => kwarg = Some(n),
                    _ => panic!("Expected parameter name after **"),
                }
                if matches!(self.peek_kind(), Some(TokenKind::Comma)) {
                    self.advance();
                }
                continue;
            }

            match self.advance().kind.clone() {
                TokenKind::Name(n) => {
                    let default = if matches!(self.peek_kind(), Some(TokenKind::Eq)) {
                        self.advance();
                        Some(self.parse_expr())
                    } else {
                        None
                    };
                    args.push((n, default));
                }
                _ => panic!("Expected parameter name"),
            }

            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        (args, vararg, kwarg)
    }

    fn peek_next_kind(&self) -> Option<TokenKind> {
        self.tokens.get(self.pos + 1).map(|t| t.kind.clone())
    }

    fn parse_class_def(&mut self) -> Stmt {
        self.expect(&TokenKind::Class);
        let name = match self.advance().kind.clone() {
            TokenKind::Name(n) => n,
            _ => panic!("Expected class name"),
        };

        let bases = if self.peek_kind() == Some(&TokenKind::LParen) {
            self.advance();
            let mut bases = Vec::new();
            loop {
                if self.peek_kind() == Some(&TokenKind::RParen) {
                    break;
                }
                bases.push(self.parse_expr());
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.expect(&TokenKind::RParen);
            bases
        } else {
            Vec::new()
        };

        let body = self.parse_block();
        Stmt::ClassDef {
            name,
            bases,
            body,
            decorators: Vec::new(),
        }
    }

    fn parse_if(&mut self) -> Stmt {
        self.expect(&TokenKind::If);
        let test = self.parse_expr();
        let body = self.parse_block();

        self.skip_newlines();
        let orelse = if self.peek_kind() == Some(&TokenKind::Elif) {
            vec![self.parse_if()]
        } else if self.peek_kind() == Some(&TokenKind::Else) {
            self.advance();
            vec![Stmt::ExprStmt(Expr::NoneLiteral)]; 
            let else_body = self.parse_block();
            else_body
        } else {
            Vec::new()
        };

        Stmt::If { test, body, orelse }
    }

    // Parse for-loop target without consuming In as comparison operator
    fn parse_for_target(&mut self) -> Expr {
        // Use parse_unary which stops before comparison operators (including In)
        if self.peek_kind() == Some(&TokenKind::LParen) {
            // Parenthesized target: (a, b) or (a)
            self.advance();
            let mut items = Vec::new();
            loop {
                if self.peek_kind() == Some(&TokenKind::RParen) {
                    self.advance();
                    break;
                }
                items.push(self.parse_unary());
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            if self.peek_kind() == Some(&TokenKind::RParen) {
                self.advance();
            }
            if items.len() == 1 {
                items.into_iter().next().unwrap()
            } else {
                Expr::Tuple(items)
            }
        } else if self.peek_kind() == Some(&TokenKind::LBracket) {
            // List target: [a, b]
            self.advance();
            let mut items = Vec::new();
            loop {
                if self.peek_kind() == Some(&TokenKind::RBracket) {
                    self.advance();
                    break;
                }
                items.push(self.parse_unary());
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            if self.peek_kind() == Some(&TokenKind::RBracket) {
                self.advance();
            }
            Expr::Tuple(items)
        } else {
            let expr = self.parse_unary();
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                let mut items = vec![expr];
                loop {
                    if self.peek_kind() == Some(&TokenKind::In) {
                        break;
                    }
                    items.push(self.parse_unary());
                    if self.peek_kind() == Some(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                Expr::Tuple(items)
            } else {
                expr
            }
        }
    }

    fn parse_for(&mut self) -> Stmt {
        self.expect(&TokenKind::For);
        let target = self.parse_for_target();
        self.expect(&TokenKind::In);
        let iter = self.parse_expr();
        let body = self.parse_block();

        let orelse = if self.peek_kind() == Some(&TokenKind::Else) {
            self.advance();
            self.parse_block()
        } else {
            Vec::new()
        };

        Stmt::For { target, iter, body, orelse }
    }

    fn parse_while(&mut self) -> Stmt {
        self.expect(&TokenKind::While);
        let test = self.parse_expr();
        let body = self.parse_block();

        let orelse = if self.peek_kind() == Some(&TokenKind::Else) {
            self.advance();
            self.parse_block()
        } else {
            Vec::new()
        };

        Stmt::While { test, body, orelse }
    }

    fn parse_try(&mut self) -> Stmt {
        self.expect(&TokenKind::Try);
        let body = self.parse_block();
        let mut handlers = Vec::new();
        let mut orelse = Vec::new();
        let mut finalbody = Vec::new();

        loop {
            self.skip_newlines();
            match self.peek_kind() {
                Some(TokenKind::Except) => {
                    self.advance();
                    let exc_type = if self.peek_kind() != Some(&TokenKind::Colon) {
                        Some(self.parse_expr())
                    } else {
                        None
                    };
                    let exc_var = if self.peek_kind() == Some(&TokenKind::As) {
                        self.advance();
                        match self.advance().kind.clone() {
                            TokenKind::Name(n) => Some(n),
                            _ => panic!("Expected variable name"),
                        }
                    } else {
                        None
                    };
                    let handler_body = self.parse_block();
                    handlers.push((exc_type, exc_var, handler_body));
                }
                Some(TokenKind::Else) => {
                    self.advance();
                    orelse = self.parse_block();
                }
                Some(TokenKind::Finally) => {
                    self.advance();
                    finalbody = self.parse_block();
                    break;
                }
                _ => break,
            }
        }

        Stmt::Try { body, handlers, orelse, finalbody }
    }

    fn parse_with(&mut self) -> Stmt {
        self.expect(&TokenKind::With);
        let mut items = Vec::new();
        loop {
            let expr = self.parse_expr();
            let alias = if self.peek_kind() == Some(&TokenKind::As) {
                self.advance();
                Some(self.parse_expr())
            } else {
                None
            };
            items.push((expr, alias));
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        let body = self.parse_block();
        Stmt::With { items, body }
    }

    fn parse_simple_stmt(&mut self) -> Stmt {
        let mut stmts = Vec::new();
        loop {
            stmts.push(self.parse_small_stmt());
            if self.peek_kind() == Some(&TokenKind::Semi) {
                self.advance();
            } else {
                break;
            }
        }
        let first = stmts.remove(0);
        for s in stmts {
            self.stmts_buffer.push(s);
        }
        first
    }

    fn parse_small_stmt(&mut self) -> Stmt {
        match self.peek_kind().cloned() {
            Some(TokenKind::Return) => self.parse_return(),
            Some(TokenKind::Raise) => self.parse_raise(),
            Some(TokenKind::Assert) => self.parse_assert(),
            Some(TokenKind::Import) => self.parse_import(),
            Some(TokenKind::From) => self.parse_import_from(),
            Some(TokenKind::Global) => self.parse_global(),
            Some(TokenKind::Nonlocal) => self.parse_nonlocal(),
            Some(TokenKind::Del) => self.parse_del(),
            Some(TokenKind::Pass) => { self.advance(); Stmt::Pass },
            Some(TokenKind::Break) => { self.advance(); Stmt::Break },
            Some(TokenKind::Continue) => { self.advance(); Stmt::Continue },
            _ => {
                let expr = self.parse_expr();
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    
                    let mut items = vec![expr];
                    self.advance(); 
                    items.extend(self.parse_expr_list());
                    if self.peek_kind() == Some(&TokenKind::Eq) {
                        self.advance();
                        let value = Expr::Tuple(self.parse_expr_list());
                        Stmt::Assign(items, value)
                    } else {
                        Stmt::ExprStmt(Expr::Tuple(items))
                    }
                } else {
                    self.parse_small_stmt_rhs(expr)
                }
            }
        }
    }

    fn parse_small_stmt_rhs(&mut self, expr: Expr) -> Stmt {
        if self.peek_kind() == Some(&TokenKind::Eq) {
            let mut targets = vec![expr];
            self.advance();
            let value = self.parse_expr();
            Stmt::Assign(targets, value)
        } else if self.peek_kind() == Some(&TokenKind::PlusEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::Add, value)
        } else if self.peek_kind() == Some(&TokenKind::MinusEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::Sub, value)
        } else if self.peek_kind() == Some(&TokenKind::StarEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::Mul, value)
        } else if self.peek_kind() == Some(&TokenKind::SlashEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::Div, value)
        } else if self.peek_kind() == Some(&TokenKind::DoubleStarEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::Pow, value)
        } else if self.peek_kind() == Some(&TokenKind::DoubleSlashEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::FloorDiv, value)
        } else if self.peek_kind() == Some(&TokenKind::PercentEq) {
            self.advance();
            let value = self.parse_expr();
            Stmt::AugAssign(expr, BinOpKind::Mod, value)
        } else if self.peek_kind() == Some(&TokenKind::Colon) {
            
            self.advance();
            let ann = self.parse_expr();
            let value = if self.peek_kind() == Some(&TokenKind::Eq) {
                self.advance();
                Some(self.parse_expr())
            } else {
                None
            };
            Stmt::AnnAssign(expr, ann, value)
        } else if self.peek_kind() == Some(&TokenKind::Walrus) {
            self.advance();
            let value = self.parse_expr();
            Stmt::Assign(vec![expr], value)
        } else {
            Stmt::ExprStmt(expr)
        }
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance();
        let value = if self.peek_kind() == Some(&TokenKind::Newline)
            || self.peek_kind() == Some(&TokenKind::EOF)
            || self.peek_kind() == Some(&TokenKind::Semi)
        {
            None
        } else {
            Some(self.parse_expr())
        };
        Stmt::Return(value)
    }

    fn parse_raise(&mut self) -> Stmt {
        self.advance();
        let exc = if self.peek_kind() == Some(&TokenKind::Newline)
            || self.peek_kind() == Some(&TokenKind::EOF)
        {
            None
        } else {
            Some(self.parse_expr())
        };
        let cause = if self.peek_kind() == Some(&TokenKind::From) {
            self.advance();
            Some(self.parse_expr())
        } else {
            None
        };
        Stmt::Raise(exc, cause)
    }

    fn parse_assert(&mut self) -> Stmt {
        self.advance();
        let test = self.parse_expr();
        let msg = if self.peek_kind() == Some(&TokenKind::Comma) {
            self.advance();
            Some(self.parse_expr())
        } else {
            None
        };
        Stmt::Assert(test, msg)
    }

    fn parse_import(&mut self) -> Stmt {
        self.advance();
        let mut names = Vec::new();
        loop {
            match self.advance().kind.clone() {
                TokenKind::Name(n) => {
                    let alias = if self.peek_kind() == Some(&TokenKind::As) {
                        self.advance();
                        match self.advance().kind.clone() {
                            TokenKind::Name(a) => Some(a),
                            _ => panic!("Expected name after as"),
                        }
                    } else {
                        None
                    };
                    names.push((n, alias));
                }
                TokenKind::Dot => {
                    
                    if let Some(last) = names.last_mut() {
                        last.0.push('.');
                    }
                }
                _ => break,
            }
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Stmt::Import(names)
    }

    fn parse_import_from(&mut self) -> Stmt {
        self.advance();
        let mut level = 0;
        let module = loop {
            match self.peek_kind() {
                Some(TokenKind::Dot) => { level += 1; self.advance(); }
                _ => break {
                    if matches!(self.peek_kind(), Some(TokenKind::Name(_))) {
                        match self.advance().kind.clone() {
                            TokenKind::Name(n) => Some(n),
                            _ => None,
                        }
                    } else {
                        None
                    }
                },
            }
        };
        while self.peek_kind() == Some(&TokenKind::Dot) {
            self.advance();
            if matches!(self.peek_kind(), Some(TokenKind::Name(_))) {
                self.advance();
            }
        }
        self.expect(&TokenKind::Import);
        let mut names = Vec::new();
        if self.peek_kind() == Some(&TokenKind::LParen) {
            self.advance();
            loop {
                if self.peek_kind() == Some(&TokenKind::RParen) { break; }
                match self.advance().kind.clone() {
                    TokenKind::Name(n) => {
                        let alias = if self.peek_kind() == Some(&TokenKind::As) {
                            self.advance();
                            match self.advance().kind.clone() {
                                TokenKind::Name(a) => Some(a),
                                _ => panic!("Expected name"),
                            }
                        } else {
                            None
                        };
                        names.push((n, alias));
                    }
                    _ => break,
                }
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.advance();
                }
            }
            self.expect(&TokenKind::RParen);
            self.skip_newlines();
        } else {
            if self.peek_kind() == Some(&TokenKind::Star) {
                self.advance();
                names.push(("*".to_string(), None));
            } else {
                loop {
                    match self.advance().kind.clone() {
                        TokenKind::Name(n) => {
                            let alias = if self.peek_kind() == Some(&TokenKind::As) {
                                self.advance();
                                match self.advance().kind.clone() {
                                    TokenKind::Name(a) => Some(a),
                                    _ => panic!("Expected name"),
                                }
                            } else {
                                None
                            };
                            names.push((n, alias));
                        }
                        _ => break,
                    }
                    if self.peek_kind() == Some(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }
        Stmt::ImportFrom { module, names, level }
    }

    fn parse_global(&mut self) -> Stmt {
        self.advance();
        let mut names = Vec::new();
        loop {
            match self.advance().kind.clone() {
                TokenKind::Name(n) => names.push(n),
                _ => panic!("Expected name"),
            }
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Stmt::Global(names)
    }

    fn parse_nonlocal(&mut self) -> Stmt {
        self.advance();
        let mut names = Vec::new();
        loop {
            match self.advance().kind.clone() {
                TokenKind::Name(n) => names.push(n),
                _ => panic!("Expected name"),
            }
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Stmt::Nonlocal(names)
    }

    fn parse_del(&mut self) -> Stmt {
        self.advance();
        let mut targets = Vec::new();
        loop {
            targets.push(self.parse_expr());
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Stmt::Delete(targets)
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_if_expr()
    }

    fn parse_expr_list(&mut self) -> Vec<Expr> {
        let mut items = Vec::new();
        items.push(self.parse_if_expr());
        while self.peek_kind() == Some(&TokenKind::Comma) {
            self.advance();
            if self.peek_kind() == Some(&TokenKind::Newline)
                || self.peek_kind() == Some(&TokenKind::EOF)
                || self.peek_kind() == Some(&TokenKind::RParen)
                || self.peek_kind() == Some(&TokenKind::RBracket)
                || self.peek_kind() == Some(&TokenKind::RBrace)
                || self.peek_kind() == Some(&TokenKind::Dedent)
                || self.peek_kind() == Some(&TokenKind::Semi)
                || self.peek_kind() == Some(&TokenKind::Eq)
            {
                break;
            }
            items.push(self.parse_if_expr());
        }
        items
    }

    fn parse_if_expr(&mut self) -> Expr {
        let mut expr = self.parse_or_expr();
        if self.peek_kind() == Some(&TokenKind::If) {
            self.advance();
            let test = self.parse_or_expr();
            self.expect(&TokenKind::Else);
            let orelse = self.parse_expr();
            expr = Expr::IfExpr(Box::new(test), Box::new(expr), Box::new(orelse));
        }
        expr
    }

    fn parse_or_expr(&mut self) -> Expr {
        let mut left = self.parse_and_expr();
        while self.peek_kind() == Some(&TokenKind::Or) {
            self.advance();
            let right = self.parse_and_expr();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitOr, Box::new(right));
        }
        left
    }

    fn parse_and_expr(&mut self) -> Expr {
        let mut left = self.parse_not_expr();
        while self.peek_kind() == Some(&TokenKind::And) {
            self.advance();
            let right = self.parse_not_expr();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitAnd, Box::new(right));
        }
        left
    }

    fn parse_not_expr(&mut self) -> Expr {
        if self.peek_kind() == Some(&TokenKind::Not) {
            self.advance();
            let expr = self.parse_not_expr();
            return Expr::UnaryOp(UnaryOpKind::Not, Box::new(expr));
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let left = self.parse_bitwise_or();
        let mut ops = Vec::new();
        let mut comparators = Vec::new();
        loop {
            let op = match self.peek_kind() {
                Some(TokenKind::EqEq) => CmpOp::Eq,
                Some(TokenKind::NotEq) => CmpOp::NotEq,
                Some(TokenKind::Less) => CmpOp::Lt,
                Some(TokenKind::LessEq) => CmpOp::LtE,
                Some(TokenKind::Greater) => CmpOp::Gt,
                Some(TokenKind::GreaterEq) => CmpOp::GtE,
                Some(TokenKind::Is) => {
                    self.advance();
                    if self.peek_kind() == Some(&TokenKind::Not) {
                        self.advance();
                        CmpOp::IsNot
                    } else {
                        CmpOp::Is
                    }
                }
                Some(TokenKind::In) => CmpOp::In,
                Some(TokenKind::Not) => {
                    if self.peek_next_kind() == Some(TokenKind::In) {
                        self.advance(); self.advance();
                        CmpOp::NotIn
                    } else {
                        break;
                    }
                }
                _ => break,
            };
            self.advance();
            ops.push(op);
            comparators.push(self.parse_bitwise_or());
        }
        if ops.is_empty() {
            left
        } else {
            Expr::Compare(Box::new(left), ops, comparators)
        }
    }

    fn parse_bitwise_or(&mut self) -> Expr {
        let mut left = self.parse_bitwise_xor();
        while self.peek_kind() == Some(&TokenKind::Pipe) {
            self.advance();
            let right = self.parse_bitwise_xor();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitOr, Box::new(right));
        }
        left
    }

    fn parse_bitwise_xor(&mut self) -> Expr {
        let mut left = self.parse_bitwise_and();
        while self.peek_kind() == Some(&TokenKind::Caret) {
            self.advance();
            let right = self.parse_bitwise_and();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitXor, Box::new(right));
        }
        left
    }

    fn parse_bitwise_and(&mut self) -> Expr {
        let mut left = self.parse_shift();
        while self.peek_kind() == Some(&TokenKind::Amp) {
            self.advance();
            let right = self.parse_shift();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitAnd, Box::new(right));
        }
        left
    }

    fn parse_shift(&mut self) -> Expr {
        let mut left = self.parse_term();
        loop {
            match self.peek_kind() {
                Some(TokenKind::LShift) => {
                    self.advance();
                    let right = self.parse_term();
                    left = Expr::BinOp(Box::new(left), BinOpKind::LShift, Box::new(right));
                }
                Some(TokenKind::RShift) => {
                    self.advance();
                    let right = self.parse_term();
                    left = Expr::BinOp(Box::new(left), BinOpKind::RShift, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();
        loop {
            match self.peek_kind() {
                Some(TokenKind::Plus) => {
                    self.advance();
                    let right = self.parse_factor();
                    left = Expr::BinOp(Box::new(left), BinOpKind::Add, Box::new(right));
                }
                Some(TokenKind::Minus) => {
                    self.advance();
                    let right = self.parse_factor();
                    left = Expr::BinOp(Box::new(left), BinOpKind::Sub, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_power();
        loop {
            match self.peek_kind() {
                Some(TokenKind::Star) => {
                    self.advance();
                    let right = self.parse_power();
                    left = Expr::BinOp(Box::new(left), BinOpKind::Mul, Box::new(right));
                }
                Some(TokenKind::Slash) => {
                    self.advance();
                    let right = self.parse_power();
                    left = Expr::BinOp(Box::new(left), BinOpKind::Div, Box::new(right));
                }
                Some(TokenKind::DoubleSlash) => {
                    self.advance();
                    let right = self.parse_power();
                    left = Expr::BinOp(Box::new(left), BinOpKind::FloorDiv, Box::new(right));
                }
                Some(TokenKind::Percent) => {
                    self.advance();
                    let right = self.parse_power();
                    left = Expr::BinOp(Box::new(left), BinOpKind::Mod, Box::new(right));
                }
                Some(TokenKind::At) => {
                    self.advance();
                    let right = self.parse_power();
                    left = Expr::BinOp(Box::new(left), BinOpKind::MatMult, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_power(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        if self.peek_kind() == Some(&TokenKind::DoubleStar) {
            self.advance();
            let right = self.parse_unary();
            expr = Expr::BinOp(Box::new(expr), BinOpKind::Pow, Box::new(right));
        }
        expr
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek_kind() {
            Some(TokenKind::Plus) => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp(UnaryOpKind::Pos, Box::new(expr))
            }
            Some(TokenKind::Minus) => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp(UnaryOpKind::Neg, Box::new(expr))
            }
            Some(TokenKind::Tilde) => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp(UnaryOpKind::Invert, Box::new(expr))
            }
            _ => self.parse_trailer(),
        }
    }

    fn parse_trailer(&mut self) -> Expr {
        let mut expr = self.parse_atom();

        loop {
            match self.peek_kind() {
                Some(TokenKind::LParen) => {
                    self.advance();
                    let (args, kwargs) = self.parse_call_args();
                    self.expect(&TokenKind::RParen);
                    expr = Expr::Call(Box::new(expr), args, kwargs);
                }
                Some(TokenKind::LBracket) => {
                    self.advance();
                    let subscript = self.parse_subscript();
                    self.expect(&TokenKind::RBracket);
                    expr = Expr::Subscript(Box::new(expr), Box::new(subscript));
                }
                Some(TokenKind::Dot) => {
                    self.advance();
                    match self.advance().kind.clone() {
                        TokenKind::Name(n) => expr = Expr::Attribute(Box::new(expr), n),
                        _ => panic!("Expected attribute name"),
                    }
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_call_args(&mut self) -> (Vec<Expr>, Vec<(String, Expr)>) {
        let mut args = Vec::new();
        let mut kwargs = Vec::new();

        loop {
            if self.peek_kind() == Some(&TokenKind::RParen) {
                break;
            }

            if self.peek_kind() == Some(&TokenKind::Star) && self.peek_next_kind() != Some(TokenKind::Star) {
                self.advance();
                args.push(Expr::Starred(Box::new(self.parse_expr())));
            } else if self.peek_kind() == Some(&TokenKind::DoubleStar) {
                self.advance();
                let expr = self.parse_expr();
                
                if let Expr::Name(k) = &expr {
                    
                }
            } else if self.peek_next_kind() == Some(TokenKind::Eq) {
                
                match self.advance().kind.clone() {
                    TokenKind::Name(n) => {
                        self.advance(); 
                        kwargs.push((n, self.parse_expr()));
                    }
                    _ => panic!("Expected keyword argument name"),
                }
            } else {
                args.push(self.parse_expr());
            }

            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        (args, kwargs)
    }

    fn parse_subscript(&mut self) -> Expr {
        if self.peek_kind() == Some(&TokenKind::Colon) {
            
            self.advance();
            let stop = if self.peek_kind() != Some(&TokenKind::RBracket)
                && self.peek_kind() != Some(&TokenKind::Colon)
            {
                Some(Box::new(self.parse_expr()))
            } else {
                None
            };
            let step = if self.peek_kind() == Some(&TokenKind::Colon) {
                self.advance();
                if self.peek_kind() != Some(&TokenKind::RBracket) {
                    Some(Box::new(self.parse_expr()))
                } else {
                    None
                }
            } else {
                None
            };
            Expr::Slice(None, stop, step)
        } else {
            let expr = self.parse_expr();
            if self.peek_kind() == Some(&TokenKind::Colon) {
                self.advance();
                let stop = if self.peek_kind() != Some(&TokenKind::RBracket)
                    && self.peek_kind() != Some(&TokenKind::Colon)
                {
                    Some(Box::new(self.parse_expr()))
                } else {
                    None
                };
                let step = if self.peek_kind() == Some(&TokenKind::Colon) {
                    self.advance();
                    if self.peek_kind() != Some(&TokenKind::RBracket) {
                        Some(Box::new(self.parse_expr()))
                    } else {
                        None
                    }
                } else {
                    None
                };
                Expr::Slice(Some(Box::new(expr)), stop, step)
            } else {
                expr
            }
        }
    }

    fn parse_atom(&mut self) -> Expr {
        match self.peek_kind().cloned() {
            Some(TokenKind::None) => { self.advance(); Expr::NoneLiteral }
            Some(TokenKind::True) => { self.advance(); Expr::BoolLiteral(true) }
            Some(TokenKind::False) => { self.advance(); Expr::BoolLiteral(false) }
            Some(TokenKind::Int(s)) => {
                self.advance();
                let val = if s.starts_with("0x") || s.starts_with("0X") {
                    i64::from_str_radix(&s[2..], 16).unwrap_or(0)
                } else if s.starts_with("0o") || s.starts_with("0O") {
                    i64::from_str_radix(&s[2..], 8).unwrap_or(0)
                } else if s.starts_with("0b") || s.starts_with("0B") {
                    i64::from_str_radix(&s[2..], 2).unwrap_or(0)
                } else {
                    s.parse().unwrap_or(0)
                };
                Expr::IntLiteral(val)
            }
            Some(TokenKind::Float(s)) => {
                self.advance();
                let val: f64 = s.parse().unwrap_or(0.0);
                Expr::FloatLiteral(val)
            }
            Some(TokenKind::String(s)) => {
                self.advance();
                Expr::StrLiteral(s)
            }
            Some(TokenKind::Name(s)) => {
                self.advance();
                if s == "f" && matches!(self.peek_kind(), Some(TokenKind::String(_))) {
                    let string_token = self.advance().kind.clone();
                    if let TokenKind::String(content) = string_token {
                        return self.parse_f_string(&content);
                    }
                }
                Expr::Name(s)
            }
            Some(TokenKind::LParen) => {
                self.advance();
                if self.peek_kind() == Some(&TokenKind::RParen) {
                    self.advance();
                    return Expr::Tuple(Vec::new());
                }
                let expr = self.parse_expr();
                if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.advance();
                    let mut items = vec![expr];
                    loop {
                        if self.peek_kind() == Some(&TokenKind::RParen) {
                            break;
                        }
                        items.push(self.parse_expr());
                        if self.peek_kind() == Some(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RParen);
                    Expr::Tuple(items)
                } else if self.peek_kind() == Some(&TokenKind::For) {
                    
                    self.advance();
                    let target = self.parse_for_target();
                    self.expect(&TokenKind::In);
                    let iter = self.parse_or_expr();
                    let mut ifs = Vec::new();
                    while self.peek_kind() == Some(&TokenKind::If) {
                        self.advance();
                        ifs.push(Box::new(self.parse_or_expr()));
                    }
                    self.expect(&TokenKind::RParen);
                    Expr::Generator(Box::new(expr), Box::new(target), Box::new(iter), ifs)
                } else {
                    self.expect(&TokenKind::RParen);
                    expr
                }
            }
            Some(TokenKind::LBracket) => {
                self.advance();
                if self.peek_kind() == Some(&TokenKind::RBracket) {
                    self.advance();
                    return Expr::List(Vec::new());
                }
                let first = self.parse_expr();
                if self.peek_kind() == Some(&TokenKind::For) {
                    
                    self.advance();
                    let target = self.parse_for_target();
                    self.expect(&TokenKind::In);
                    let iter = self.parse_or_expr();
                    let mut ifs = Vec::new();
                    while self.peek_kind() == Some(&TokenKind::If) {
                        self.advance();
                        ifs.push(Box::new(self.parse_or_expr()));
                    }
                    self.expect(&TokenKind::RBracket);
                    Expr::ListComp(Box::new(first), Box::new(target), Box::new(iter), ifs)
                } else if self.peek_kind() == Some(&TokenKind::Comma) {
                    self.advance();
                    let mut items = vec![first];
                    loop {
                        if self.peek_kind() == Some(&TokenKind::RBracket) {
                            break;
                        }
                        items.push(self.parse_expr());
                        if self.peek_kind() == Some(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RBracket);
                    Expr::List(items)
                } else if self.peek_kind() == Some(&TokenKind::Colon) {
                    
                    self.advance();
                    let stop = if self.peek_kind() != Some(&TokenKind::RBracket)
                        && self.peek_kind() != Some(&TokenKind::Colon)
                    {
                        Some(Box::new(self.parse_expr()))
                    } else {
                        None
                    };
                    let step = if self.peek_kind() == Some(&TokenKind::Colon) {
                        self.advance();
                        if self.peek_kind() != Some(&TokenKind::RBracket) {
                            Some(Box::new(self.parse_expr()))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    self.expect(&TokenKind::RBracket);
                    Expr::Slice(Some(Box::new(first)), stop, step)
                } else {
                    self.expect(&TokenKind::RBracket);
                    Expr::List(vec![first])
                }
            }
            Some(TokenKind::LBrace) => {
                self.advance();
                if self.peek_kind() == Some(&TokenKind::RBrace) {
                    self.advance();
                    return Expr::Dict(Vec::new());
                }
                let first = self.parse_expr();
                if self.peek_kind() == Some(&TokenKind::Colon) {
                    
                    self.advance();
                    let first_val = self.parse_expr();
                    let mut items = vec![(first, first_val)];
                    loop {
                        if self.peek_kind() == Some(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                        if self.peek_kind() == Some(&TokenKind::RBrace) {
                            break;
                        }
                        let key = self.parse_expr();
                        self.expect(&TokenKind::Colon);
                        let val = self.parse_expr();
                        items.push((key, val));
                    }
                    self.expect(&TokenKind::RBrace);
                    Expr::Dict(items)
                } else if self.peek_kind() == Some(&TokenKind::For) {
                    
                    self.advance();
                    let target = self.parse_for_target();
                    self.expect(&TokenKind::In);
                    let iter = self.parse_or_expr();
                    let mut ifs = Vec::new();
                    while self.peek_kind() == Some(&TokenKind::If) {
                        self.advance();
                        ifs.push(Box::new(self.parse_or_expr()));
                    }
                    self.expect(&TokenKind::RBrace);
                    Expr::SetComp(Box::new(first), Box::new(target), Box::new(iter), ifs)
                } else {
                    
                    let mut items = vec![first];
                    loop {
                        if self.peek_kind() == Some(&TokenKind::RBrace) {
                            break;
                        }
                        if self.peek_kind() == Some(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                        if self.peek_kind() == Some(&TokenKind::RBrace) {
                            break;
                        }
                        items.push(self.parse_expr());
                    }
                    self.expect(&TokenKind::RBrace);
                    Expr::Set(items)
                }
            }
            Some(TokenKind::Lambda) => {
                self.advance();
                let args = self.parse_lambda_args();
                self.expect(&TokenKind::Colon);
                let body = self.parse_expr();
                Expr::Lambda(args, Box::new(body))
            }
            Some(TokenKind::Yield) => {
                self.advance();
                let value = if self.peek_kind() == Some(&TokenKind::From) {
                    self.advance();
                    Some(Box::new(self.parse_expr()))
                } else if self.peek_kind() == Some(&TokenKind::Newline)
                    || self.peek_kind() == Some(&TokenKind::EOF)
                    || self.peek_kind() == Some(&TokenKind::RParen)
                    || self.peek_kind() == Some(&TokenKind::RBracket)
                    || self.peek_kind() == Some(&TokenKind::RBrace)
                {
                    None
                } else {
                    Some(Box::new(self.parse_expr()))
                };
                Expr::Yield(value)
            }
            Some(TokenKind::Ellipsis) => {
                self.advance();
                Expr::Ellipsis
            }
            Some(TokenKind::Await) => {
                self.advance();
                let expr = self.parse_expr();
                Expr::Await(Box::new(expr))
            }
            Some(_) => panic!("Unexpected token: {} at line {}", self.peek().unwrap().kind, self.peek().unwrap().line),
            None => panic!("Unexpected end of expression"),
        }
    }

    fn parse_lambda_args(&mut self) -> Vec<String> {
        let mut args = Vec::new();
        if self.peek_kind() == Some(&TokenKind::Colon) {
            return args;
        }
        loop {
            match self.advance().kind.clone() {
                TokenKind::Name(n) => args.push(n),
                _ => panic!("Expected parameter name in lambda"),
            }
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        args
    }

    fn parse_f_string(&mut self, content: &str) -> Expr {
        let mut parts = Vec::new();
        let mut literal = String::new();
        let chars: Vec<char> = content.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '{' {
                if i + 1 < chars.len() && chars[i + 1] == '{' {
                    literal.push('{');
                    i += 2;
                } else {
                    if !literal.is_empty() {
                        parts.push(Expr::StrLiteral(std::mem::take(&mut literal)));
                    }
                    i += 1;
                    let mut depth = 1u32;
                    let mut expr_str = String::new();
                    while i < chars.len() && depth > 0 {
                        match chars[i] {
                            '{' => { depth += 1; if depth > 1 { expr_str.push('{'); } }
                            '}' => {
                                depth -= 1;
                                if depth > 0 { expr_str.push('}'); }
                            }
                            c => expr_str.push(c),
                        }
                        i += 1;
                    }
                    parts.push(self.parse_f_string_expr(&expr_str));
                }
            } else if chars[i] == '}' {
                if i + 1 < chars.len() && chars[i + 1] == '}' {
                    literal.push('}');
                    i += 2;
                } else {
                    literal.push('}');
                    i += 1;
                }
            } else {
                literal.push(chars[i]);
                i += 1;
            }
        }

        if !literal.is_empty() {
            parts.push(Expr::StrLiteral(literal));
        }

        if parts.is_empty() {
            Expr::StrLiteral(String::new())
        } else if parts.len() == 1 {
            parts.into_iter().next().unwrap()
        } else {
            Expr::JoinedStr(parts)
        }
    }

    fn parse_f_string_expr(&mut self, expr_str: &str) -> Expr {
        let tokens = crate::lexer::tokenize(expr_str);
        let mut p = Parser::new(tokens);
        p.parse_expr()
    }
}
