use bahasac::codegen::codegen::CodeGen;
use bahasac::parse::parse::parse;
use bahasac::token::token::Token;
// use bahasac::scope::Scope;
// use bahasac::value::Value;
use inkwell::{builder::Builder, context::Context, module::Module};
// use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let context = Context::create();
    // let code = "printf(\"hi\")";
    // let code = "int x = 3;";
    let code = "fn a() {}";

    let tokens = Token::tokenize(code);
    let parsed_stmt_vec = parse(&tokens);
    let codegen = CodeGen::new(&context);

    // pub fn generate_code(&self, parsed_stmt: Vec<Stmt>, context: &'ctx Context, builder: &Builder, module: &mut Module<'ctx>)
    codegen.generate_code(parsed_stmt_vec);
    let result = codegen.show_result(); //result returns String
    
    let mut llvm_ir_code_file = File::create("exec.ll").unwrap();
    llvm_ir_code_file.write_all(result.as_bytes()).unwrap();
}
