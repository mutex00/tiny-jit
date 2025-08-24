/* ================= VM with calls ================= */

use std::collections::HashMap;

use crate::bytecode::BC;
use crate::codegen::Module;



#[derive(Clone, Debug)]
enum Value { Int(i64) }

pub struct VM<'m> {
    module: &'m Module,
    // Call stack of environments (lexical locals). Simple: HashMap per frame.
    env_stack: Vec<HashMap<String, Value>>,
}

impl<'m> VM<'m> {
    pub fn new(module: &'m Module) -> Self {
        Self { module, env_stack: vec![HashMap::new()] }
    }

    fn with_frame<F: FnOnce(&mut VM) -> Value>(&mut self, f: F) -> Value {
        self.env_stack.push(HashMap::new());
        let ret = f(self);
        self.env_stack.pop();
        ret
    }

    fn set(&mut self, k: &str, v: Value) {
        self.env_stack.last_mut().unwrap().insert(k.to_string(), v);
    }
    fn get(&self, k: &str) -> Value {
        for frame in self.env_stack.iter().rev() {
            if let Some(v) = frame.get(k) { return v.clone(); }
        }
        Value::Int(0)
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Value {
        let proto = self.module.funs.get(name)
            .unwrap_or_else(|| panic!("undefined function: {}", name));
        if proto.params.len() != args.len() { panic!("arity mismatch for {}", name); }

        self.with_frame(|vm| {
            for (p, v) in proto.params.iter().zip(args.into_iter()) {
                vm.set(p, v);
            }
            vm.run_code(&proto.code)
        })
    }

    fn run_code(&mut self, code: &[BC]) -> Value {
        use BC::*;
        let mut stack: Vec<Value> = Vec::new();
        let mut ip = 0usize;
        loop {
            let op = &code[ip];
            match op {
                LoadConst(n) => stack.push(Value::Int(*n)),
                LoadVar(name) => stack.push(self.get(name)),
                StoreVar(name) => {
                    let v = stack.pop().expect("stack underflow");
                    self.set(name, v);
                }
                Add => {
                    let b = match stack.pop().expect("stack underflow") { Value::Int(n) => n };
                    let a = match stack.pop().expect("stack underflow") { Value::Int(n) => n };
                    stack.push(Value::Int(a + b));
                }
                Call(fname, argc) => {
                    let mut args = Vec::with_capacity(*argc);
                    for _ in 0..*argc {
                        args.push(stack.pop().expect("stack underflow"));
                    }
                    args.reverse();
                    let ret = self.call_function(fname, args);
                    stack.push(ret);
                }
                Print => {
                    match stack.pop().expect("stack underflow") {
                        Value::Int(n) => println!("{n}"),
                    }
                }
                Ret => {
                    return stack.pop().unwrap_or(Value::Int(0));
                }
            }
            ip += 1;
        }
    }

    pub fn run_main(&mut self) {
        let _ = self.run_code(&self.module.main.code);
    }
}