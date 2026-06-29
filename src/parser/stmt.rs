use crate::ast::*;
use crate::lexer::TokenKind;

impl super::Parser {
    pub(crate) fn parse_stmt(&mut self) -> Stmt {
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

    pub(crate) fn parse_block(&mut self) -> Vec<Stmt> {
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

    pub(crate) fn parse_decorated(&mut self) -> Stmt {
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

    pub(crate) fn parse_func_def(&mut self) -> Stmt {
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

    pub(crate) fn parse_func_args(&mut self) -> (Vec<(String, Option<Expr>)>, Option<String>, Option<String>) {
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

    pub(crate) fn parse_class_def(&mut self) -> Stmt {
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

    pub(crate) fn parse_if(&mut self) -> Stmt {
        let tok = self.peek_kind();
        match tok {
            Some(&TokenKind::If) | Some(&TokenKind::Elif) => { self.advance(); }
            Some(other) => panic!("Expected if or elif, found {:?}", other),
            None => panic!("Expected if or elif, found EOF"),
        }
        let test = self.parse_expr();
        let body = self.parse_block();

        self.skip_newlines();
        let orelse = if self.peek_kind() == Some(&TokenKind::Elif) {
            vec![self.parse_if()]
        } else if self.peek_kind() == Some(&TokenKind::Else) {
            self.advance();
            self.parse_block()
        } else {
            Vec::new()
        };

        Stmt::If { test, body, orelse }
    }

    // Parse for-loop target without consuming In as comparison operator
    pub(crate) fn parse_for_target(&mut self) -> Expr {
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

    pub(crate) fn parse_for(&mut self) -> Stmt {
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

    pub(crate) fn parse_while(&mut self) -> Stmt {
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

    pub(crate) fn parse_try(&mut self) -> Stmt {
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

    pub(crate) fn parse_with(&mut self) -> Stmt {
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

    pub(crate) fn parse_simple_stmt(&mut self) -> Stmt {
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

    pub(crate) fn parse_small_stmt(&mut self) -> Stmt {
        // Skip comment tokens
        while let Some(TokenKind::Comment(_)) = self.peek_kind() {
            self.advance();
        }
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

    pub(crate) fn parse_small_stmt_rhs(&mut self, expr: Expr) -> Stmt {
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

    pub(crate) fn parse_return(&mut self) -> Stmt {
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

    pub(crate) fn parse_raise(&mut self) -> Stmt {
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

    pub(crate) fn parse_assert(&mut self) -> Stmt {
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

    pub(crate) fn parse_import(&mut self) -> Stmt {
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

    pub(crate) fn parse_import_from(&mut self) -> Stmt {
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

    pub(crate) fn parse_global(&mut self) -> Stmt {
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

    pub(crate) fn parse_nonlocal(&mut self) -> Stmt {
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

    pub(crate) fn parse_del(&mut self) -> Stmt {
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

}
