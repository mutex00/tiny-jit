mod lexer;
mod bytecode;
mod parser;
mod ast;
mod codegen;
mod vm;
mod ir;
mod jit;



use codegen::Module;
use parser::Parser;
use lexer::Lexer;
use vm::VM;


/* ================= Demo ================= */

fn dump_module(m: &Module) {
    println!("== Functions ==");
    for (name, f) in &m.funs {
        println!("fn {}({})", name, f.params.join(", "));
        for (i, bc) in f.code.iter().enumerate() {
            println!("  {:04}: {:?}", i, bc);
        }
    }
    println!("\n== main ==");
    for (i, bc) in m.main.code.iter().enumerate() {
        println!("  {:04}: {:?}", i, bc);
    }
}

fn main() {
    let program = r#"
        fn add(a, b) {
            return a + b;
        }

        fn twice(x) {
            return add(x, x);
        }

        x = add(2, 3);
        print x;

        y = twice(x);
        print y;

        print add(x + y, 7);
    "#;

    // Frontend
    let mut parser = Parser::new(Lexer::new(program));
    let ast = parser.parse_program();

    // Compile
    let module = codegen::compile_module(ast);
    dump_module(&module);

    // Run
    println!("\n== Program output ==");
    let mut vm = VM::new(&module);
    vm.run_main();
}
