use crate::ir::{Ref, IR, IROp, IRIns, IRType};
use crate::bytecode::BC;


use std::collections::HashMap;
use std::u16;

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

    fn emit_loadvar(&mut self, sym: u16) -> Ref {
        if let Some(&r) = self.env.get(&sym){ return r; };

        let r = self.ir.push(IRIns {
            op : IROp::LoadVar,
            ty : IRType::Int,
            a: Ref(sym),
            b: Ref::NONE,
            prev_same_op: u16::MAX
        });

        self.env.insert(sym, r);
        r
    }

    fn emit_storevar(&mut self, sym: u16, v: Ref) {
        if self.env.get(&sym).copied() == Some(v) { return; }
        self.ir.push(IRIns { 
            op: IROp::StoreVar,
            ty: IRType::Any,
            a: Ref(sym),
            b: v, 
            prev_same_op: u16::MAX });
        self.env.insert(sym, v);
    }

    fn emit_print(&mut self, v: Ref) {
        self.ir.push(IRIns {
            op: IROp::Print,
            ty: IRType::Any,
            a: v,
            b: Ref::NONE,
            prev_same_op: u16::MAX
        });
    }

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
                    let v = self.stack.pop().expect("stack underflow");
                    let sym = self.ir.intern_sym(name.as_str());
                    self.emit_storevar(sym, v);
                }
                BC::Print => {
                    let v = self.stack.pop().expect("stack underflow");
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
