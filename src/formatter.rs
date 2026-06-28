use crate::lexer::{tokenize, TokenKind};

pub fn format_python(source: &str) -> String {
    let tokens = tokenize(source);
    let mut out = String::new();
    let mut i = 0;
    let mut indent: usize = 0;
    let mut at_newline = true;
    let mut prev: Option<&TokenKind> = None;
    let mut suppress_indent = false;

    while i < tokens.len() {
        let kind = &tokens[i].kind;

        match kind {
            TokenKind::Newline => {
                out.push('\n');
                at_newline = true;
                prev = None;
                suppress_indent = false;
            }
            TokenKind::Indent => {
                indent += 1;
            }
            TokenKind::Dedent => {
                indent = indent.saturating_sub(1);
            }
            TokenKind::Comment(text) => {
                if !at_newline { out.push_str("  "); }
                out.push('#');
                out.push_str(text);
                out.push('\n');
                at_newline = true;
                prev = None;
                suppress_indent = true;
            }
            TokenKind::EOF => break,
            _ => {
                if at_newline && !suppress_indent {
                    for _ in 0..indent { out.push_str("    "); }
                }
                at_newline = false;
                suppress_indent = false;

                let text = token_text(kind, &tokens, &mut i);
                if needs_space(prev, kind, &text) {
                    out.push(' ');
                }
                out.push_str(&text);
                prev = Some(kind);
            }
        }
        i += 1;
    }

    // Trim trailing whitespace from each line
    let mut result = String::new();
    for line in out.lines() {
        result.push_str(line.trim_end());
        result.push('\n');
    }
    // Remove trailing newlines
    while result.ends_with('\n') {
        result.pop();
    }
    result.push('\n');
    result
}

fn needs_space(prev: Option<&TokenKind>, curr: &TokenKind, _curr_text: &str) -> bool {
    use TokenKind::*;
    use std::option::Option::None as OptNone;
    match (prev, curr) {
        (OptNone, _) => false,
        (Some(Newline), _) | (Some(Indent), _) | (Some(Dedent), _) => false,
        (Some(Comment(_)), _) => true,

        // Before open brackets/parens - no space for function calls, space for keywords
        (Some(Name(_)), LParen) | (Some(Name(_)), LBracket) | (Some(Name(_)), LBrace) => false,
        (Some(_), LParen) | (Some(_), LBracket) | (Some(_), LBrace) => true,

        // After open brackets
        (Some(LParen), _) | (Some(LBracket), _) | (Some(LBrace), _) => false,

        // Before close brackets
        (Some(_), RParen) | (Some(_), RBracket) | (Some(_), RBrace) => false,

        // Dots
        (Some(Dot), _) | (Some(_), Dot) => false,

        // Commas
        (Some(Comma), _) => true,
        (Some(_), Comma) => false,

        // Colons
        (Some(Colon), _) => true,
        (Some(_), Colon) => false,

        // Semicolons
        (Some(Semi), _) => true,
        (Some(_), Semi) => false,

        // Operators - space around binary operators
        (Some(Plus), _) | (Some(_), Plus) => true,
        (Some(Minus), _) | (Some(_), Minus) => true,
        (Some(Star), _) | (Some(_), Star) => true,
        (Some(Slash), _) | (Some(_), Slash) => true,
        (Some(Percent), _) | (Some(_), Percent) => true,
        (Some(DoubleStar), _) | (Some(_), DoubleStar) => true,
        (Some(DoubleSlash), _) | (Some(_), DoubleSlash) => true,
        (Some(Eq), _) | (Some(_), Eq) => true,
        (Some(EqEq), _) | (Some(_), EqEq) => true,
        (Some(NotEq), _) | (Some(_), NotEq) => true,
        (Some(Less), _) | (Some(_), Less) => true,
        (Some(Greater), _) | (Some(_), Greater) => true,
        (Some(LessEq), _) | (Some(_), LessEq) => true,
        (Some(GreaterEq), _) | (Some(_), GreaterEq) => true,
        (Some(Amp), _) | (Some(_), Amp) => true,
        (Some(Pipe), _) | (Some(_), Pipe) => true,
        (Some(Caret), _) | (Some(_), Caret) => true,
        (Some(LShift), _) | (Some(_), LShift) => true,
        (Some(RShift), _) | (Some(_), RShift) => true,
        (Some(PlusEq), _) | (Some(_), PlusEq) => true,
        (Some(MinusEq), _) | (Some(_), MinusEq) => true,
        (Some(StarEq), _) | (Some(_), StarEq) => true,
        (Some(SlashEq), _) | (Some(_), SlashEq) => true,
        (Some(PercentEq), _) | (Some(_), PercentEq) => true,
        (Some(DoubleStarEq), _) | (Some(_), DoubleStarEq) => true,
        (Some(DoubleSlashEq), _) | (Some(_), DoubleSlashEq) => true,
        (Some(Tilde), _) => false, // unary operator
        (Some(_), Tilde) => true,

        // Keywords
        (Some(And), _) | (Some(Or), _) | (Some(Not), _) | (Some(In), _) | (Some(Is), _) => true,
        (Some(_), And) | (Some(_), Or) | (Some(_), Not) | (Some(_), In) | (Some(_), Is) => true,

        (Some(Def), _) | (Some(Class), _) | (Some(Return), _) | (Some(From), _) | (Some(Import), _)
        | (Some(If), _) | (Some(Elif), _) | (Some(Else), _) | (Some(For), _) | (Some(While), _)
        | (Some(Try), _) | (Some(Except), _) | (Some(Finally), _) | (Some(Yield), _)
        | (Some(Lambda), _) | (Some(With), _) | (Some(Raise), _) | (Some(Assert), _)
        | (Some(Global), _) | (Some(Nonlocal), _) | (Some(Del), _) | (Some(Async), _) => true,

        (Some(_), Name(_)) | (Some(Name(_)), _) => true,
        (Some(_), Int(_)) | (Some(_), Float(_)) | (Some(_), String(_)) => true,
        (Some(Int(_)), _) | (Some(Float(_)), _) | (Some(String(_)), _) => true,

        (Some(_), True) | (Some(_), False) | (Some(_), None) => true,
        (Some(True), _) | (Some(False), _) | (Some(None), _) => true,

        (Some(Arrow), _) => true,
        (Some(_), Arrow) => true,
        (Some(At), _) => true,
        (Some(_), At) => false,

        _ => true,
    }
}

fn token_text(kind: &TokenKind, _tokens: &[crate::lexer::Token], _i: &mut usize) -> String {
    use TokenKind::*;
    match kind {
        Name(n) => n.clone(),
        Int(n) => n.clone(),
        Float(n) => n.clone(),
        String(s) => {
            // Check if it needs quoting
            if s.contains('\n') || s.contains('"') || s.contains('\'') {
                format!("'''{}'''", s)
            } else {
                format!("\"{}\"", s)
            }
        }
        FStringStart => "f\"".to_string(),
        FStringEnd => "\"".to_string(),
        FStringMid(s) => s.clone(),
        Comment(_) => "".to_string(),
        Def => "def".to_string(),
        Class => "class".to_string(),
        If => "if".to_string(),
        Elif => "elif".to_string(),
        Else => "else".to_string(),
        For => "for".to_string(),
        While => "while".to_string(),
        Return => "return".to_string(),
        Import => "import".to_string(),
        From => "from".to_string(),
        As => "as".to_string(),
        Pass => "pass".to_string(),
        Break => "break".to_string(),
        Continue => "continue".to_string(),
        And => "and".to_string(),
        Or => "or".to_string(),
        Not => "not".to_string(),
        In => "in".to_string(),
        Is => "is".to_string(),
        Try => "try".to_string(),
        Except => "except".to_string(),
        Finally => "finally".to_string(),
        Raise => "raise".to_string(),
        With => "with".to_string(),
        Yield => "yield".to_string(),
        Lambda => "lambda".to_string(),
        Assert => "assert".to_string(),
        Global => "global".to_string(),
        Nonlocal => "nonlocal".to_string(),
        Del => "del".to_string(),
        Async => "async".to_string(),
        Await => "await".to_string(),
        True => "True".to_string(),
        False => "False".to_string(),
        None => "None".to_string(),
        Plus => "+".to_string(),
        Minus => "-".to_string(),
        Star => "*".to_string(),
        Slash => "/".to_string(),
        Percent => "%".to_string(),
        DoubleStar => "**".to_string(),
        DoubleSlash => "//".to_string(),
        At => "@".to_string(),
        PlusEq => "+=".to_string(),
        MinusEq => "-=".to_string(),
        StarEq => "*=".to_string(),
        SlashEq => "/=".to_string(),
        PercentEq => "%=".to_string(),
        DoubleStarEq => "**=".to_string(),
        DoubleSlashEq => "//=".to_string(),
        AtEq => "@=".to_string(),
        Eq => "=".to_string(),
        EqEq => "==".to_string(),
        NotEq => "!=".to_string(),
        Less => "<".to_string(),
        Greater => ">".to_string(),
        LessEq => "<=".to_string(),
        GreaterEq => ">=".to_string(),
        Amp => "&".to_string(),
        Pipe => "|".to_string(),
        Caret => "^".to_string(),
        Tilde => "~".to_string(),
        LShift => "<<".to_string(),
        RShift => ">>".to_string(),
        AmpEq => "&=".to_string(),
        PipeEq => "|=".to_string(),
        CaretEq => "^=".to_string(),
        LShiftEq => "<<=".to_string(),
        RShiftEq => ">>=".to_string(),
        LParen => "(".to_string(),
        RParen => ")".to_string(),
        LBracket => "[".to_string(),
        RBracket => "]".to_string(),
        LBrace => "{".to_string(),
        RBrace => "}".to_string(),
        Colon => ":".to_string(),
        Semi => ";".to_string(),
        Dot => ".".to_string(),
        Comma => ",".to_string(),
        Arrow => "->".to_string(),
        Walrus => ":=".to_string(),
        Ellipsis => "...".to_string(),
        Newline => "\n".to_string(),
        Indent => "".to_string(),
        Dedent => "".to_string(),
        EOF => "".to_string(),
    }
}
