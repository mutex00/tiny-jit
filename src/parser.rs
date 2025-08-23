
/* ================= Parser ================= */

use crate::lexer::{Lexer,Token};
use crate::ast::{Stmt, Function, Expr};

pub struct Parser<'a> {
    lex: Lexer<'a>,
    cur: Token,
}
impl<'a> Parser<'a> {
    pub fn new(mut lex: Lexer<'a>) -> Self { let cur = lex.next_token(); Self { lex, cur } }
    pub fn bump(&mut self) { self.cur = self.lex.next_token(); }
    pub fn expect(&mut self, want: &Token) {
        if &self.cur != want { 
            panic!("Expected {:?}, got {:?}", want, self.cur);
        }
        self.bump();
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut v = Vec::new();
        loop {
            match &self.cur {
                Token::EOF => break,
                Token::Fn => v.push(Stmt::FunctionDef(self.parse_fn())),
                _ => v.push(self.parse_stmt()),
            }
        }
        v
    }

    pub fn parse_fn(&mut self) -> Function {
        self.expect(&Token::Fn);
        let name = match std::mem::replace(&mut self.cur, Token::EOF) {
            Token::Ident(s) => { self.bump(); s }
            t => panic!("fn name expected, got {:?}", t),
        };
        self.expect(&Token::LParen);
        let mut params = Vec::new();
        if self.cur != Token::RParen {
            loop {
                match std::mem::replace(&mut self.cur, Token::EOF) {
                    Token::Ident(s) => { self.bump(); params.push(s); }
                    t => panic!("param name expected, got {:?}", t),
                }
                if self.cur == Token::Comma { self.bump(); continue; }
                break;
            }
        }
        self.expect(&Token::RParen);
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while self.cur != Token::RBrace {
            body.push(self.parse_stmt());
        }
        self.expect(&Token::RBrace);
        Function { name, params, body }
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        use Token::*;
        match &self.cur {
            Print => { self.bump(); let e = self.parse_expr(); self.expect(&Semicolon); Stmt::Print(e) }
            Return => { self.bump(); let e = self.parse_expr(); self.expect(&Semicolon); Stmt::Return(e) }
            Ident(name) => {
                // either assignment or call-statement (we support call as expr too)
                let name_clone = name.clone();
                self.bump();
                match &self.cur {
                    Assign => {
                        self.bump();
                        let e = self.parse_expr();
                        self.expect(&Semicolon);
                        Stmt::Assign(name_clone, e)
                    }
                    LParen => {
                        // call as statement
                        let call = self.finish_call(name_clone);
                        self.expect(&Semicolon);
                        // desugar: tmp = call; print? For now we just evaluate and discard.
                        Stmt::Print(call) // (or create a Stmt::Expr(call) variant; we’ll just print)
                    }
                    _ => panic!("unexpected token after ident in stmt: {:?}", self.cur),
                }
            }
            _ => panic!("bad stmt start: {:?}", self.cur),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut lhs = self.parse_atom();
        while self.cur == Token::Plus {
            self.bump();
            let rhs = self.parse_atom();
            lhs = Expr::Add(Box::new(lhs), Box::new(rhs));
        }
        lhs
    }

    fn parse_atom(&mut self) -> Expr {
        use Token::*;
        match std::mem::replace(&mut self.cur, Token::EOF) {
            Number(n) => { self.bump(); Expr::Number(n) }
            Ident(s) => {
                // could be var or call
                self.bump(); // consume '('
                if self.cur == LParen {
                    self.bump();
                    return self.finish_call(s);
                } else {
                    Expr::Var(s)
                }
            }
            t => panic!("atom expected, got {:?}", t),
        }
    }

    fn finish_call(&mut self, name: String) -> Expr {
        let mut args = Vec::new();
        if self.cur != Token::RParen {
            loop {
                let e = self.parse_expr();
                args.push(e);
                if self.cur == Token::Comma { self.bump(); continue; }
                break;
            }
        }
        self.expect(&Token::RParen);
        Expr::Call(name, args)
    }
}
