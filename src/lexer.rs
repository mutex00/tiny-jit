/* ================= Lexer ================= */

use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Ident(String),
    Plus, Assign, Semicolon, Comma,
    LParen, RParen, LBrace, RBrace,
    Print, Fn, Return,
    EOF,
}

pub struct Lexer<'a> { it: Peekable<std::str::Chars<'a>>, line_idx: usize }
impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self { Self { it: s.chars().peekable(), line_idx: 0usize } }
    pub fn next_token(&mut self) -> Token {
        use Token::*;
        while let Some(&c) = self.it.peek() {
            match c {
                c if c.is_whitespace() => { self.it.next(); }
                '0'..='9' => return self.lex_num(),
                'a'..='z' | 'A'..='Z' | '_' => return self.lex_ident(),
                '+' => { self.it.next(); return Plus; }
                '=' => { self.it.next(); return Assign; }
                ';' => { self.it.next(); return Semicolon; }
                ',' => { self.it.next(); return Comma; }
                '(' => { self.it.next(); return LParen; }
                ')' => { self.it.next(); return RParen; }
                '{' => { self.it.next(); return LBrace; }
                '}' => { self.it.next(); return RBrace; }
                '\n' => { self.it.next(); self.line_idx = self.line_idx + 1; }
                _ => { self.it.next(); /* skip unknown */ }
            }
        }
        EOF
    }
    pub fn lex_num(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.it.peek() {
            if c.is_ascii_digit() { s.push(c); self.it.next(); } else { break; }
        }
        Token::Number(s.parse().unwrap())
    }
    pub fn lex_ident(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.it.peek() {
            if c.is_alphanumeric() || c == '_' { s.push(c); self.it.next(); } else { break; }
        }
        match s.as_str() {
            "print"  => Token::Print,
            "fn"     => Token::Fn,
            "return" => Token::Return,
            _        => Token::Ident(s),
        }
    }
}