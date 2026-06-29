#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
    pub(crate) stmts_buffer: Vec<Stmt>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, stmts_buffer: Vec::new() }
    }

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub(crate) fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    pub(crate) fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }

    pub(crate) fn expect(&mut self, kind: &TokenKind) {
        if self.peek_kind() == Some(kind) {
            self.advance();
        } else {
            let found = self.peek().map(|t| t.kind.to_string()).unwrap_or("EOF".to_string());
            panic!("Expected {}, found {} at line {}", kind, found, self.peek().map(|t| t.line).unwrap_or(0));
        }
    }

    pub(crate) fn skip_newlines(&mut self) {
        loop {
            match self.peek_kind() {
                Some(&TokenKind::Newline) | Some(&TokenKind::Comment(_)) => { self.advance(); }
                _ => break,
            }
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

    pub(crate) fn peek_next_kind(&self) -> Option<TokenKind> {
        self.tokens.get(self.pos + 1).map(|t| t.kind.clone())
    }
}

mod stmt;
mod expr;
