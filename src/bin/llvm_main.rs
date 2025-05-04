// use bahasac::scope::Scope;
// use bahasac::value::Value;
use inkwell::{builder::Builder, context::Context, module::Module};
use Kurai::codegen::codegen::CodeGen;
use Kurai::parse::parse::parse;
use Kurai::token::token::Token;
// use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let context = Context::create();
    // let code = "printf(\"hi\")";
    let code = "
        let x = 3;
        x = 1;
    ";
    // let code = "fn a() {
    //     let x = 3;
    // }";

    let tokens = Token::tokenize(code);
    let parsed_stmt_vec = parse(&tokens);
    let mut codegen = CodeGen::new(&context);

    // pub fn generate_code(&self, parsed_stmt: Vec<Stmt>, context: &'ctx Context, builder: &Builder, module: &mut Module<'ctx>)
    println!("{:?}", parsed_stmt_vec);
    codegen.generate_code(parsed_stmt_vec);
    let result = codegen.show_result(); //result returns String
    
    let mut llvm_ir_code_file = File::create("exec.ll").unwrap();
    llvm_ir_code_file.write_all(result.as_bytes()).unwrap();
}
