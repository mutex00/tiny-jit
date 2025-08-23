use crate::ir::{Ref, IR, IROp, IRIns, IRType};
use crate::bytecode::BC;


use std::collections::HashMap;

struct Recorder {
    ir: IR,
    stack: Vec<Ref>,
    env: HashMap<u16, Ref>
}

impl Recorder{
    pub fn new() -> Self {
        Self { ir: IR::new(), stack: Vec::new(), env: HashMap::new() }
    }

    fn emit_add(&mut self, a: Ref, b: Ref) -> Ref {
        // constant folding, if we're adding two const combine them into one
        if let (Some(x), Some(y)) = (self.ir.const_value(a), self.ir.const_value(b)) {
            return self.ir.emit_kint(x + y);
        }

        // common subexpression elimination (CSE) using skip chain lookback
        let mut prev = self.ir.last_of_op[IROp::Add as usize];
        while prev != u16::MAX {
            let candidate = &self.ir.code[prev as usize];
            let same = (candidate.a == a && candidate.b == b) || (candidate.a == b && candidate.b == a);

            if same { return Ref(prev); }
            prev = candidate.prev_same_op;
        }

        self.ir.push(IRIns {
                op: IROp::Add,
                ty: IRType::Int,
                a,
                b,
                prev_same_op: u16::MAX,
        })
    }

    fn emit_loadvar();

    fn emit_storevar();

    fn emit_print();

    pub fn record(&mut self, bc: Vec<BC>){
        for op in bc {
            match op {
                BC::LoadConst(n) => {
                    let r = self.ir.emit_kint(n);
                    self.stack.push(r);
                }
                BC::LoadVar(name) => {
                    let sym = self.ir.intern_sym(name.as_str());
                    let r = self.emit_loadvar(sym);
                    self.stack.push(r);
                }
                BC::Add => {
                    let b = self.stack.pop().expect("stack underflow");
                    let a = self.stack.pop().expect("stack underflow");
                    let r = self.emit_add(a, b);
                    self.stack.push(r);
                }
                BC::StoreVar(name) => {
                    let v = self.stack.pop();
                    let sym = self.ir.intern_sym(name.as_str());
                    self.emit_storevar(sym, v);
                }
                BC::Print => {
                    let v = self.stack.pop();
                    self.emit_print(v);
                }
                BC::Call(name, n_args) => {
                    /// >>????
                }
                BC::Ret => {
                    //// >>>>>?????
                }
            }
        }
    }
}
