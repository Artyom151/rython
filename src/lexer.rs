use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Name(String),
    Int(String),
    Float(String),
    String(String),
    FStringStart,
    FStringEnd,
    FStringMid(String),

    Def, Class, If, Elif, Else, For, While, Return,
    Import, From, As, Pass, Break, Continue,
    And, Or, Not, In, Is, Try, Except, Finally,
    Raise, With, Yield, Lambda, Assert,
    Global, Nonlocal, Del, Async, Await,
    True, False, None,

    Plus, Minus, Star, Slash, Percent, DoubleStar, DoubleSlash, At,
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq,
    DoubleStarEq, DoubleSlashEq, AtEq,
    Eq, EqEq, NotEq, Less, Greater, LessEq, GreaterEq,
    Amp, Pipe, Caret, Tilde, LShift, RShift,
    AmpEq, PipeEq, CaretEq, LShiftEq, RShiftEq,

    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Colon, Semi, Dot, Comma, Arrow, Walrus, Ellipsis,
    Newline, Indent, Dedent, EOF,
    Comment(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Name(s) => write!(f, "{}", s),
            TokenKind::Int(s) => write!(f, "{}", s),
            TokenKind::Float(s) => write!(f, "{}", s),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::FStringStart => write!(f, "f\""),
            TokenKind::FStringEnd => write!(f, "\""),
            TokenKind::FStringMid(s) => write!(f, "{}", s),
            TokenKind::Def => write!(f, "def"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Elif => write!(f, "elif"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::For => write!(f, "for"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::From => write!(f, "from"),
            TokenKind::As => write!(f, "as"),
            TokenKind::Pass => write!(f, "pass"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Is => write!(f, "is"),
            TokenKind::Try => write!(f, "try"),
            TokenKind::Except => write!(f, "except"),
            TokenKind::Finally => write!(f, "finally"),
            TokenKind::Raise => write!(f, "raise"),
            TokenKind::With => write!(f, "with"),
            TokenKind::Yield => write!(f, "yield"),
            TokenKind::Lambda => write!(f, "lambda"),
            TokenKind::Assert => write!(f, "assert"),
            TokenKind::Global => write!(f, "global"),
            TokenKind::Nonlocal => write!(f, "nonlocal"),
            TokenKind::Del => write!(f, "del"),
            TokenKind::Async => write!(f, "async"),
            TokenKind::Await => write!(f, "await"),
            TokenKind::True => write!(f, "True"),
            TokenKind::False => write!(f, "False"),
            TokenKind::None => write!(f, "None"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::DoubleStar => write!(f, "**"),
            TokenKind::DoubleSlash => write!(f, "//"),
            TokenKind::At => write!(f, "@"),
            TokenKind::PlusEq => write!(f, "+="),
            TokenKind::MinusEq => write!(f, "-="),
            TokenKind::StarEq => write!(f, "*="),
            TokenKind::SlashEq => write!(f, "/="),
            TokenKind::PercentEq => write!(f, "%="),
            TokenKind::DoubleStarEq => write!(f, "**="),
            TokenKind::DoubleSlashEq => write!(f, "//="),
            TokenKind::AtEq => write!(f, "@="),
            TokenKind::Eq => write!(f, "="),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::LessEq => write!(f, "<="),
            TokenKind::GreaterEq => write!(f, ">="),
            TokenKind::Amp => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::LShift => write!(f, "<<"),
            TokenKind::RShift => write!(f, ">>"),
            TokenKind::AmpEq => write!(f, "&="),
            TokenKind::PipeEq => write!(f, "|="),
            TokenKind::CaretEq => write!(f, "^="),
            TokenKind::LShiftEq => write!(f, "<<="),
            TokenKind::RShiftEq => write!(f, ">>="),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Walrus => write!(f, ":="),
            TokenKind::Ellipsis => write!(f, "..."),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Indent => write!(f, "INDENT"),
            TokenKind::Dedent => write!(f, "DEDENT"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Comment(text) => write!(f, "#{}", text),
        }
    }
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    indent_stack: Vec<usize>,
    tokens: Vec<Token>,
    pending: Vec<Token>,
    at_beginning_of_line: bool,
    paren_depth: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Lexer {
            chars,
            pos: 0,
            line: 1,
            col: 1,
            indent_stack: vec![0],
            tokens: Vec::new(),
            pending: Vec::new(),
            at_beginning_of_line: true,
            paren_depth: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn emit(&mut self, kind: TokenKind) {
        let token = Token {
            kind,
            line: self.line,
            col: self.col,
        };
        self.pending.push(token);
    }

    fn emit_str(&mut self, s: &str, kind: fn(String) -> TokenKind) {
        self.emit(kind(s.to_string()));
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\r'
    }

    fn is_ident_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_ident_continue(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn read_indent(&mut self) -> usize {
        let mut indent = 0usize;
        while let Some(c) = self.peek() {
            if c == ' ' {
                indent += 1;
                self.advance();
            } else if c == '\t' {
                indent += 4;
                self.advance();
            } else if c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
        indent
    }

    fn handle_indent(&mut self) {
        if self.paren_depth > 0 {
            return;
        }

        if self.peek() == Some('\n') || self.peek().is_none() {
            return;
        }

        let indent = self.read_indent();
        let current = *self.indent_stack.last().unwrap();

        if indent > current {
            self.indent_stack.push(indent);
            self.emit(TokenKind::Indent);
        } else {
            while *self.indent_stack.last().unwrap() > indent {
                self.indent_stack.pop();
                self.emit(TokenKind::Dedent);
            }
        }
    }

    fn read_string(&mut self, quote: char, triple: bool) -> String {
        let mut s = String::new();
        loop {
            match self.advance() {
                None => break,
                Some('\\') => {
                    match self.advance() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('r') => s.push('\r'),
                        Some('\\') => s.push('\\'),
                        Some('\'') => s.push('\''),
                        Some('"') => s.push('"'),
                        Some(c) => { s.push('\\'); s.push(c); }
                        None => s.push('\\'),
                    }
                }
                Some(c) if c == quote => {
                    if triple {
                        if self.peek() == Some(quote) && self.peek_next() == Some(quote) {
                            self.advance(); self.advance();
                            break;
                        }
                        s.push(c);
                    } else {
                        break;
                    }
                }
                Some(c) => s.push(c),
            }
        }
        s
    }

    fn tokenize(&mut self) -> Vec<Token> {
        loop {
            if self.at_beginning_of_line {
                self.at_beginning_of_line = false;
                let saved_pos = self.pos;
                let indent = self.read_indent();

                if self.peek() == Some('\n') || self.peek().is_none() || self.peek() == Some('#') {
                    self.pos = saved_pos;
                    self.col -= indent;
                } else if self.paren_depth == 0 {
                    let current = *self.indent_stack.last().unwrap();
                    if indent > current {
                        self.indent_stack.push(indent);
                        self.emit(TokenKind::Indent);
                    } else {
                        while *self.indent_stack.last().unwrap() > indent {
                            self.indent_stack.pop();
                            self.emit(TokenKind::Dedent);
                        }
                    }
                }
            }

            match self.advance() {
                None => {
                    while self.indent_stack.len() > 1 {
                        self.indent_stack.pop();
                        self.emit(TokenKind::Dedent);
                    }
                    self.emit(TokenKind::EOF);
                    self.tokens.append(&mut self.pending);
                    break;
                }
                Some('\n') => {
                    if self.paren_depth == 0 {
                        self.emit(TokenKind::Newline);
                    }
                    self.at_beginning_of_line = true;
                }
                Some(c) if Self::is_whitespace(c) => {}
                Some('#') => {
                    let start = self.pos;
                    while self.peek() != Some('\n') && self.peek().is_some() {
                        self.advance();
                    }
                    let comment_text: String = self.chars[start..self.pos].iter().collect();
                    let text = if comment_text.starts_with("# ") { comment_text[2..].to_string() }
                               else if comment_text.starts_with('#') { comment_text[1..].to_string() }
                               else { comment_text.clone() };
                    self.emit(TokenKind::Comment(text));
                }
                Some(c) if Self::is_ident_start(c) => {
                    let mut name = String::new();
                    name.push(c);
                    while let Some(next) = self.peek() {
                        if Self::is_ident_continue(next) {
                            name.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                    let kind = match name.as_str() {
                        "def" => TokenKind::Def,
                        "class" => TokenKind::Class,
                        "if" => TokenKind::If,
                        "elif" => TokenKind::Elif,
                        "else" => TokenKind::Else,
                        "for" => TokenKind::For,
                        "while" => TokenKind::While,
                        "return" => TokenKind::Return,
                        "import" => TokenKind::Import,
                        "from" => TokenKind::From,
                        "as" => TokenKind::As,
                        "pass" => TokenKind::Pass,
                        "break" => TokenKind::Break,
                        "continue" => TokenKind::Continue,
                        "and" => TokenKind::And,
                        "or" => TokenKind::Or,
                        "not" => TokenKind::Not,
                        "in" => TokenKind::In,
                        "is" => TokenKind::Is,
                        "try" => TokenKind::Try,
                        "except" => TokenKind::Except,
                        "finally" => TokenKind::Finally,
                        "raise" => TokenKind::Raise,
                        "with" => TokenKind::With,
                        "yield" => TokenKind::Yield,
                        "lambda" => TokenKind::Lambda,
                        "assert" => TokenKind::Assert,
                        "global" => TokenKind::Global,
                        "nonlocal" => TokenKind::Nonlocal,
                        "del" => TokenKind::Del,
                        "async" => TokenKind::Async,
                        "await" => TokenKind::Await,
                        "True" => TokenKind::True,
                        "False" => TokenKind::False,
                        "None" => TokenKind::None,
                        _ => TokenKind::Name(name),
                    };
                    self.emit(kind);
                }
                Some(c @ '"') | Some(c @ '\'') => {
                    let quote = c;
                    let triple = self.peek() == Some(quote) && self.peek_next() == Some(quote);
                    if triple {
                        self.advance(); self.advance();
                    }
                    let s = self.read_string(quote, triple);
                    self.emit(TokenKind::String(s));
                }
                Some(c @ '0'..='9') => {
                    let mut num = String::new();
                    num.push(c);
                    let mut is_float = false;
                    while let Some(next) = self.peek() {
                        if next.is_ascii_digit() {
                            num.push(self.advance().unwrap());
                        } else if next == '.' && !is_float {
                            let after = self.peek_next();
                            if after.map_or(false, |a| a.is_ascii_digit()) {
                                is_float = true;
                                num.push(self.advance().unwrap());
                            } else {
                                break;
                            }
                        } else if next == 'e' || next == 'E' {
                            is_float = true;
                            num.push(self.advance().unwrap());
                            if self.peek() == Some('+') || self.peek() == Some('-') {
                                num.push(self.advance().unwrap());
                            }
                        } else if next == 'x' || next == 'X' || next == 'o' || next == 'O' || next == 'b' || next == 'B' {
                            if num == "0" {
                                num.push(self.advance().unwrap());
                            } else {
                                break;
                            }
                        } else if next == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if is_float {
                        self.emit(TokenKind::Float(num));
                    } else {
                        self.emit(TokenKind::Int(num));
                    }
                }
                Some('.') => {
                    if self.peek() == Some('.') && self.peek_next() == Some('.') {
                        self.advance(); self.advance();
                        self.emit(TokenKind::Ellipsis);
                    } else if self.peek().map_or(false, |c| c.is_ascii_digit()) {
                        let mut num = String::from(".");
                        while let Some(next) = self.peek() {
                            if next.is_ascii_digit() {
                                num.push(self.advance().unwrap());
                            } else if next == '_' {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.emit(TokenKind::Float(num));
                    } else {
                        self.emit(TokenKind::Dot);
                    }
                }
                Some('+') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::PlusEq); }
                    else { self.emit(TokenKind::Plus); }
                }
                Some('-') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::MinusEq); }
                    else if self.peek() == Some('>') { self.advance(); self.emit(TokenKind::Arrow); }
                    else { self.emit(TokenKind::Minus); }
                }
                Some('*') => {
                    if self.peek() == Some('*') {
                        self.advance();
                        if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::DoubleStarEq); }
                        else { self.emit(TokenKind::DoubleStar); }
                    } else if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::StarEq); }
                    else { self.emit(TokenKind::Star); }
                }
                Some('/') => {
                    if self.peek() == Some('/') {
                        self.advance();
                        if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::DoubleSlashEq); }
                        else { self.emit(TokenKind::DoubleSlash); }
                    } else if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::SlashEq); }
                    else { self.emit(TokenKind::Slash); }
                }
                Some('%') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::PercentEq); }
                    else { self.emit(TokenKind::Percent); }
                }
                Some('@') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::AtEq); }
                    else { self.emit(TokenKind::At); }
                }
                Some('=') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::EqEq); }
                    else { self.emit(TokenKind::Eq); }
                }
                Some('!') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::NotEq); }
                    else { panic!("Unexpected '!' at line {}", self.line); }
                }
                Some('<') => {
                    if self.peek() == Some('<') {
                        self.advance();
                        if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::LShiftEq); }
                        else { self.emit(TokenKind::LShift); }
                    } else if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::LessEq); }
                    else { self.emit(TokenKind::Less); }
                }
                Some('>') => {
                    if self.peek() == Some('>') {
                        self.advance();
                        if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::RShiftEq); }
                        else { self.emit(TokenKind::RShift); }
                    } else if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::GreaterEq); }
                    else { self.emit(TokenKind::Greater); }
                }
                Some('&') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::AmpEq); }
                    else { self.emit(TokenKind::Amp); }
                }
                Some('|') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::PipeEq); }
                    else { self.emit(TokenKind::Pipe); }
                }
                Some('^') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::CaretEq); }
                    else { self.emit(TokenKind::Caret); }
                }
                Some('~') => { self.emit(TokenKind::Tilde); }
                Some('(') => { self.paren_depth += 1; self.emit(TokenKind::LParen); }
                Some(')') => { self.paren_depth -= 1; self.emit(TokenKind::RParen); }
                Some('[') => { self.paren_depth += 1; self.emit(TokenKind::LBracket); }
                Some(']') => { self.paren_depth -= 1; self.emit(TokenKind::RBracket); }
                Some('{') => { self.paren_depth += 1; self.emit(TokenKind::LBrace); }
                Some('}') => { self.paren_depth -= 1; self.emit(TokenKind::RBrace); }
                Some(':') => {
                    if self.peek() == Some('=') { self.advance(); self.emit(TokenKind::Walrus); }
                    else { self.emit(TokenKind::Colon); }
                }
                Some(';') => { self.emit(TokenKind::Semi); }
                Some(',') => { self.emit(TokenKind::Comma); }
                Some('\\') => {
                    if self.peek() == Some('\n') {
                        self.advance();
                    } else if self.peek() == Some('\r') {
                        self.advance();
                        if self.peek() == Some('\n') {
                            self.advance();
                        }
                    }
                }
                Some(c) => {
                    panic!("Unexpected character '{}' at line {} col {}", c, self.line, self.col);
                }
            }

            self.tokens.append(&mut self.pending);
        }

        std::mem::take(&mut self.tokens)
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
