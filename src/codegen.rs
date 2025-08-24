/* ================= Codegen ================= */
use std::collections::HashMap;

use crate::ast::*;
use crate::bytecode::*;


#[derive(Debug, Clone)]
pub struct FunctionProto {
    pub name: String,
    pub params: Vec<String>,
    pub code: Vec<BC>,
}


#[derive(Debug, Clone)]
pub struct Module {
    pub funs: HashMap<String, FunctionProto>,
    pub main: FunctionProto,
}


pub fn gen_expr(code: &mut Vec<BC>, e: &Expr) {
    match e {
        Expr::Number(n) => code.push(BC::LoadConst(*n)),
        Expr::Var(v) => code.push(BC::LoadVar(v.clone())),
        Expr::Add(a, b) => {
            gen_expr(code,a); 
            gen_expr(code,b); 
            code.push(BC::Add); 
        }
        Expr::Call(name, args) => {
            for a in args {
                gen_expr(code, a);
            }
            code.push(BC::Call(name.clone(), args.len()));
        }
    }
}

pub fn gen_stmt(code: &mut Vec<BC>, s: &Stmt) {
    match s {
        Stmt::Assign(name, e) => { gen_expr(code, e); code.push(BC::StoreVar(name.clone())); }
        Stmt::Print(e)        => { gen_expr(code, e); code.push(BC::Print); }
        Stmt::Return(e)       => { gen_expr(code, e); code.push(BC::Ret); }
        Stmt::FunctionDef(_)  => { /* handled at module level */ }
    }
}

pub fn compile_module(stmts: Vec<Stmt>) -> Module {
    let mut funs = HashMap::new();
    let mut main_code = Vec::new();

    // First, extract function defs into prototypes.
    for s in &stmts {
        if let Stmt::FunctionDef(f) = s {
            let mut code = Vec::new();
            for st in &f.body { gen_stmt(&mut code, st); }
            // ensure implicit return (like Lua) if none present
            if !matches!(code.last(), Some(BC::Ret)) { code.push(BC::LoadConst(0)); code.push(BC::Ret); }
            funs.insert(f.name.clone(), FunctionProto { name: f.name.clone(), params: f.params.clone(), code });
        }
    }

    // Now compile top-level into main()
    for s in &stmts {
        if !matches!(s, Stmt::FunctionDef(_)) {
            gen_stmt(&mut main_code, s);
        }
    }
    // main returns 0
    if !matches!(main_code.last(), Some(BC::Ret)) { main_code.push(BC::LoadConst(0)); main_code.push(BC::Ret); }

    let main = FunctionProto { name: "main".into(), params: vec![], code: main_code };
    Module { funs, main }
}