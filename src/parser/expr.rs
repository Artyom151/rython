use crate::ast::*;
use crate::lexer::TokenKind;

impl super::Parser {
    pub(crate) fn parse_expr(&mut self) -> Expr {
        self.parse_named_expr()
    }

    fn parse_named_expr(&mut self) -> Expr {
        // name := expr  (walrus operator)
        // backtrack if not followed by :=
        if self.peek_kind().map_or(false, |k| matches!(k, TokenKind::Name(_))) {
            let n = match self.peek_kind().unwrap() {
                TokenKind::Name(s) => s.clone(),
                _ => unreachable!(),
            };
            if n == "_" { return self.parse_if_expr(); } // _ is not assignable via walrus
            let saved = self.pos;
            self.advance();
            if self.peek_kind() == Some(&TokenKind::Walrus) {
                self.advance();
                let value = self.parse_expr();
                return Expr::NamedExpr(Box::new(Expr::Name(n)), Box::new(value));
            }
            self.pos = saved;
        }
        self.parse_if_expr()
    }

    pub(crate) fn parse_expr_list(&mut self) -> Vec<Expr> {
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

    pub(crate) fn parse_if_expr(&mut self) -> Expr {
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

    pub(crate) fn parse_or_expr(&mut self) -> Expr {
        let mut left = self.parse_and_expr();
        while self.peek_kind() == Some(&TokenKind::Or) {
            self.advance();
            let right = self.parse_and_expr();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitOr, Box::new(right));
        }
        left
    }

    pub(crate) fn parse_and_expr(&mut self) -> Expr {
        let mut left = self.parse_not_expr();
        while self.peek_kind() == Some(&TokenKind::And) {
            self.advance();
            let right = self.parse_not_expr();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitAnd, Box::new(right));
        }
        left
    }

    pub(crate) fn parse_not_expr(&mut self) -> Expr {
        if self.peek_kind() == Some(&TokenKind::Not) {
            self.advance();
            let expr = self.parse_not_expr();
            return Expr::UnaryOp(UnaryOpKind::Not, Box::new(expr));
        }
        self.parse_comparison()
    }

    pub(crate) fn parse_comparison(&mut self) -> Expr {
        let left = self.parse_bitwise_or();
        let mut ops = Vec::new();
        let mut comparators = Vec::new();
        loop {
            let op = match self.peek_kind() {
                Some(TokenKind::EqEq) => { self.advance(); CmpOp::Eq }
                Some(TokenKind::NotEq) => { self.advance(); CmpOp::NotEq }
                Some(TokenKind::Less) => { self.advance(); CmpOp::Lt }
                Some(TokenKind::LessEq) => { self.advance(); CmpOp::LtE }
                Some(TokenKind::Greater) => { self.advance(); CmpOp::Gt }
                Some(TokenKind::GreaterEq) => { self.advance(); CmpOp::GtE }
                Some(TokenKind::Is) => {
                    self.advance();
                    if self.peek_kind() == Some(&TokenKind::Not) {
                        self.advance();
                        CmpOp::IsNot
                    } else {
                        CmpOp::Is
                    }
                }
                Some(TokenKind::In) => { self.advance(); CmpOp::In }
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
            ops.push(op);
            comparators.push(self.parse_bitwise_or());
        }
        if ops.is_empty() {
            left
        } else {
            Expr::Compare(Box::new(left), ops, comparators)
        }
    }

    pub(crate) fn parse_bitwise_or(&mut self) -> Expr {
        let mut left = self.parse_bitwise_xor();
        while self.peek_kind() == Some(&TokenKind::Pipe) {
            self.advance();
            let right = self.parse_bitwise_xor();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitOr, Box::new(right));
        }
        left
    }

    pub(crate) fn parse_bitwise_xor(&mut self) -> Expr {
        let mut left = self.parse_bitwise_and();
        while self.peek_kind() == Some(&TokenKind::Caret) {
            self.advance();
            let right = self.parse_bitwise_and();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitXor, Box::new(right));
        }
        left
    }

    pub(crate) fn parse_bitwise_and(&mut self) -> Expr {
        let mut left = self.parse_shift();
        while self.peek_kind() == Some(&TokenKind::Amp) {
            self.advance();
            let right = self.parse_shift();
            left = Expr::BinOp(Box::new(left), BinOpKind::BitAnd, Box::new(right));
        }
        left
    }

    pub(crate) fn parse_shift(&mut self) -> Expr {
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

    pub(crate) fn parse_term(&mut self) -> Expr {
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

    pub(crate) fn parse_factor(&mut self) -> Expr {
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

    pub(crate) fn parse_power(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        if self.peek_kind() == Some(&TokenKind::DoubleStar) {
            self.advance();
            let right = self.parse_unary();
            expr = Expr::BinOp(Box::new(expr), BinOpKind::Pow, Box::new(right));
        }
        expr
    }

    pub(crate) fn parse_unary(&mut self) -> Expr {
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

    pub(crate) fn parse_trailer(&mut self) -> Expr {
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

    pub(crate) fn parse_call_args(&mut self) -> (Vec<Expr>, Vec<(String, Expr)>) {
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

    pub(crate) fn parse_subscript(&mut self) -> Expr {
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

    pub(crate) fn parse_atom(&mut self) -> Expr {
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

    pub(crate) fn parse_lambda_args(&mut self) -> Vec<String> {
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

    pub(crate) fn parse_f_string(&mut self, content: &str) -> Expr {
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

    pub(crate) fn parse_f_string_expr(&mut self, expr_str: &str) -> Expr {
        let tokens = crate::lexer::tokenize(expr_str);
        let mut p = super::Parser::new(tokens);
        p.parse_expr()
    }
}
