use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IROp {
    KInt=0,
    Add=1,
    LoadVar=2,
    StoreVar=3,
    Print=4
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRType {
    Int=0,
    Any=1
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ref(u16);
impl Ref {const NONE: Ref = Ref(u16::MAX); }


#[derive(Clone)]
pub struct IRIns {
    pub op: IROp,
    pub ty: IRType,
    pub a: Ref,
    pub b: Ref,
    pub prev_same_op: u16, // used for optimisations
}

pub struct IR {
    pub code: Vec<IRIns>,
    pub last_of_op: [u16; 256],
    const_pool: Vec<i64>,
    const_map: HashMap<i64, Ref>,
    sym_pool: Vec<String>,
    sym_map: HashMap<String,u16>
}

impl IR {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            last_of_op: [u16::MAX; 256],
            const_pool: Vec::new(),
            const_map: HashMap::new(),
            sym_pool: Vec::new(),
            sym_map: HashMap::new(),
        }
    }

    pub fn intern_sym(&mut self, s: &str) -> u16 {
        if let Some(&id) = self.sym_map.get(s) { return id; }

        let id = self.sym_pool.len() as u16;
        self.sym_pool.push(s.to_string());
        self.sym_map.insert(s.to_string(), id);
        id
    }

    pub fn push(&mut self, mut ins: IRIns) -> Ref {
        // Want to record the last of this op for the skip list
        let idx = self.code.len() as u16;
        let opi = ins.op as usize;
        ins.prev_same_op = self.last_of_op[opi];
        self.last_of_op[opi]= idx;
    
        // Now push the IR op
        self.code.push(ins);
        Ref(idx)
    }

    pub fn emit_kint(&mut self, n: i64) -> Ref {
        // Check if we already have this value in our IR
        if let Some(&r) = self.const_map.get(&n) { return r };

        let kid = self.const_pool.len() as u16;
        self.const_pool.push(n);

        let r = self.push(IRIns{ 
            op: IROp::KInt,
            ty: IRType::Int,
            a: Ref(kid),
            b: Ref::NONE,
            prev_same_op: u16::MAX});

        self.const_map.insert(n, r);
        r
    }

    pub fn const_value(&self, r: Ref) -> Option<i64> {
        let i = r.0 as usize;
        if i >= self.code.len() { return None; }
        let ins = &self.code[i];
        if ins.op == IROp::KInt {
            Some(self.const_pool[ins.a.0 as usize])
        } else {
            None
        }
    }
}