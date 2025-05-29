use std::{fs::File, process::Command};

use inkwell::context::Context;
use Kurai::{codegen::codegen::CodeGen, parse::parse::{parse_out_vec_expr, parse_out_vec_stmt}, token::token::Token};

#[test]
fn print_i_love_sara() {
    let code = r#"
        fn main() {
            printf("Do you like sara?");
            check();
        }
    "#;

    let tokens = Token::tokenize(code);
    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parse_out_vec_stmt(&tokens, &mut discovered_modules);
    let parsed_expr_vec = parse_out_vec_expr(&tokens);

    let context = Context::create();
    let mut codegen = CodeGen::new(&context);

    println!("STATEMENTS:\n{:?}", parsed_stmt_vec);
    println!("EXPRESSIONS:\n{:?}", parsed_expr_vec);
    codegen.generate_code(parsed_stmt_vec, parsed_expr_vec.expect("purr!"), &mut discovered_modules);
    let result = codegen.show_result(); //result returns String

    println!("VARIABLES:\n{:?}", codegen.variables);

    let mut llvm_ir_code_file = File::create("exec.ll").unwrap();

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
