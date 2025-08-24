/* ================= Bytecode ================= */

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


