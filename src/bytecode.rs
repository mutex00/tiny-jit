
/* ================= Bytecode & Module ================= */
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum BC {
    // values
    LoadConst(i64),
    LoadVar(String),
    StoreVar(String),
    Add,

    // function/misc
    Call(String, usize), // func name, argc
    Ret,                 // return top of stack
    Print,               // builtin
}

#[derive(Debug, Clone)]
pub struct FunctionProto {
    pub name: String,
    pub params: Vec<String>,
    pub code: Vec<BC>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub funs: HashMap<String, FunctionProto>,
    pub main: FunctionProto, // top-level statements as an implicit main()
}

