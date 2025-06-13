// use bahasac::scope::Scope;
// use bahasac::value::Value;
use inkwell::context::Context;
use kurai_codegen::codegen::CodeGen;
use kurai_parser::parse::parse::{parse_out_vec_expr, parse_out_vec_stmt};
use kurai_token::token::token::Token;
use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let context = Context::create();
    // let code = "printf(\"hi\")";
    // let code = "
    //     fn main() {
    //         let x = 3;
    //         x = 1;
    //     }
    // ";
    // let code = r#"
    // use stdlib::print;
    //
    //     fn main() {
    //         printf("prr");
    //         printf("prr #2");
    //     }
    // "#;
    let code = r#"
        use stdlib::print;
        fn check() {
            let do_i_like_sara = 10;
            if (do_i_like_sara <= 10) {
                printf("YES I DO!");
            } else {
                printf("idk");
            }
        }

        fn main() {
            printf("Do you like sara?");
            check();
        }
    "#;

    // let args: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    // let code: &str = &args;
    let tokens = Token::tokenize(code);
    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parse_out_vec_stmt(&tokens, &mut discovered_modules);
    let parsed_expr_vec = parse_out_vec_expr(&tokens);
    let mut codegen = CodeGen::new(&context);

    // pub fn generate_code(&self, parsed_stmt: Vec<Stmt>, context: &'ctx Context, builder: &Builder, module: &mut Module<'ctx>)
    println!("STATEMENTS:\n{:?}", parsed_stmt_vec);
    println!("EXPRESSIONS:\n{:?}", parsed_expr_vec);
    codegen.generate_code(parsed_stmt_vec, parsed_expr_vec.expect("purr!"), &mut discovered_modules);
    let result = codegen.show_result(); //result returns String

    println!("VARIABLES:\n{:?}", codegen.variables);

    let mut llvm_ir_code_file = File::create("exec.ll").unwrap();
    llvm_ir_code_file.write_all(result.as_bytes()).unwrap();

    let status = Command::new("clang")
        .arg("exec.ll")
        .arg("-o")
        .arg("exec")
        .status()
        .unwrap();

    match status.success() {
        true => println!("Compilation successful"),
        false => println!("meh"),
    }
}
