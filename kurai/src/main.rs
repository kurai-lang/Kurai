use colored::Colorize;
// use bahasac::scope::Scope;
// use bahasac::value::Value;
use inkwell::context::Context;
use kurai_codegen::codegen::CodeGen;
use kurai_parser::parse::parse::{parse_out_vec_expr, parse_out_vec_stmt};
use kurai_parser::parse::parse_stmt::StmtParserStruct;
use kurai_parser_function::FunctionParserStruct;
use kurai_parser_import_decl::ImportParserStruct;
use kurai_token::token::token::Token;
use core::time;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
// use std::sync::{Arc, Mutex};
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;
use std::time::Instant;
use kurai_codegen::CodeGenPrint;

fn main() {
    let context = Context::create();
    let args = env::args().skip(1).collect::<Vec<String>>();

    // let mut code = String::new();
    let code = "
            fn main() {
                let x = 0;
            }
        ";
    let mut output_path: Cow<'static, str> = Cow::Owned("target/a".to_owned());

    if !args.is_empty() {
        let file_path = &args[args.len() - 1];
        // code = fs::read_to_string(file_path).unwrap();

        if let Some(output_name_index) = args.iter().position(|x| x == "-o") {
            if let Some(name) = args.get(output_name_index + 1) {
                output_path = Cow::Owned(format!("target/{}", name));
            }
        }
    }

    let tokens = Token::tokenize(code);
    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parse_out_vec_stmt(&tokens, &mut discovered_modules, &FunctionParserStruct, &ImportParserStruct);
    let parsed_expr_vec = parse_out_vec_expr(&tokens);
    let mut codegen = CodeGen::new(&context);
    codegen.printf("hi");

    // pub fn generate_code(&self, parsed_stmt: Vec<Stmt>, context: &'ctx Context, builder: &Builder, module: &mut Module<'ctx>)
    #[cfg(debug_assertions)]
    {
        println!("STATEMENTS:\n{:?}", parsed_stmt_vec);
        println!("EXPRESSIONS:\n{:?}", parsed_expr_vec);
        println!("VARIABLES:\n{:?}", codegen.variables);
    }

    codegen.generate_code(parsed_stmt_vec, parsed_expr_vec.expect("purr!"), &mut discovered_modules, &StmtParserStruct, &FunctionParserStruct, &ImportParserStruct);
    let result = codegen.show_result(); //result returns String

    let output_path_ll = format!("{}.ll", output_path);
    let mut llvm_ir_code_file = File::create(&output_path_ll).unwrap();
    llvm_ir_code_file.write_all(result.as_bytes()).unwrap();

    let start_time = Instant::now();
    let status = Command::new("clang")
        .arg("-Wno-override-module")
        .arg(&output_path_ll)
        .arg("-o")
        .arg(output_path.as_ref())
        .status()
        .unwrap();
    let end_time = start_time.elapsed().as_secs_f64();

    if status.success() {
        println!("{} the program in {:.2}", "Finished".green().bold(), end_time);
        println!("{} `{}`", "Running".green().bold(), output_path);
    } else {
        println!("{}: Compilation unsuccessful", "error".red());
    }
}
