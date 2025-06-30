use clap::Parser;
use colored::Colorize;
// use bahasac::scope::Scope;
// use bahasac::value::Value;
use inkwell::context::Context;
use kurai_codegen::codegen::CodeGen;
use kurai_parser::parse::parse::{parse_out_vec_expr, parse_out_vec_stmt};
use kurai_parser::parse::parse_stmt::StmtParserStruct;
use kurai_parser::GroupedParsers;
use kurai_parser_function::FunctionParserStruct;
use kurai_parser_import_decl::ImportParserStruct;
use kurai_parser_loop::LoopParserStruct;
use kurai_parser_loop::BlockParserStruct;
use kurai_token::token::token::Token;
use kurai_core::scope::Scope;
use std::fs::{self, remove_file, File};
use std::io::prelude::*;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

static PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Parser)]
struct Cli {
    pub input: String,

    #[arg(long, default_value="a", short='o')]
    pub output_name: String,

    #[arg(long, default_value="2", short='O', value_parser=clap::value_parser!(u8))]
    pub opt_level: u8,

    #[arg(long, action)]
    pub show_output_files: bool,
}

fn main() {
    let cli = Cli::parse();
    // let output_name = format!("target/{}", cli.output_name);
    let output_name = cli.output_name;
    let output_name_clone = output_name.clone();

    std::panic::set_hook(Box::new(move |panic_info| {
        PANIC_COUNT.fetch_add(1, Ordering::SeqCst);

        if let Some(location) = panic_info.location() {
            eprintln!("{}:{}", location.file(), location.line());
        }

        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("{}: {}", "error".red().bold(), s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("{}: {}", "error".red().bold(), s);
        } else {
            eprintln!("{}: could not output a reason why", "warning".yellow().bold());
        }

        eprintln!("{}: could not compile `{}` due to {} previous error",
            "error".red().bold(),
            output_name_clone,
            PANIC_COUNT.load(Ordering::SeqCst));
    }));

    let context = Context::create();
    let parsers = GroupedParsers::new(
        &StmtParserStruct,
        &FunctionParserStruct,
        &ImportParserStruct,
        &BlockParserStruct,
        &LoopParserStruct
    );

    let mut code = String::new();

    #[cfg(debug_assertions)]
    {
        let code = "
            fn main() {
                for i in 0..4 {
                    printf(\"yes?\");
                }
            }
        ";
    }

    let file_path = cli.input;

    if !(cfg!(debug_assertions)) { 
        code = fs::read_to_string(file_path).unwrap(); 
    }

    let mut scope = Scope::new();

    let tokens = Token::tokenize(code.as_str());
    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parse_out_vec_stmt(
        &tokens,
        &mut discovered_modules,
        &parsers,
        &mut scope,
    );
    let parsed_expr_vec = parse_out_vec_expr(&tokens, &mut discovered_modules, &parsers, &mut scope);
    let mut codegen = CodeGen::new(&context);
    // codegen.printf("hi");

    // pub fn generate_code(&self, parsed_stmt: Vec<Stmt>, context: &'ctx Context, builder: &Builder, module: &mut Module<'ctx>)
    #[cfg(debug_assertions)]
    {
        println!("STATEMENTS:\n{:?}", parsed_stmt_vec);
        println!("EXPRESSIONS:\n{:?}", parsed_expr_vec);
        println!("VARIABLES:\n{:?}", codegen.variables);
    }

    codegen.generate_code(
        parsed_stmt_vec,
        parsed_expr_vec.unwrap(), 
        &mut discovered_modules,
        &parsers,
        &mut scope,
    );
    let result = codegen.show_result(); //result returns String

    let output_path_ll = format!("{}.ll", output_name);
    let output_path_opt_ll = format!("{}_opt.ll", output_name);
    let output_path_bc = format!("{}.bc", output_name);
    let output_path_opt_bc = format!("{}_opt.bc", output_name);
    let output_path_s = format!("{}.s", output_name);
    let output_path_o = format!("{}.o", output_name);

    let mut llvm_ir_code_file = File::create(&output_path_ll).unwrap();
    llvm_ir_code_file.write_all(result.as_bytes()).unwrap();

    let start_time = Instant::now();
    Command::new("llvm-as")
        .arg(&output_path_ll)
        .arg("-o")
        .arg(&output_path_bc)
        .status()
        .unwrap();
    Command::new("opt")
        .arg(format!("-O{}", cli.opt_level))
        .arg("-S")
        .arg(&output_path_bc)
        .arg("-o")
        .arg(&output_path_opt_ll)
        .status()
        .unwrap();
    Command::new("llc")
        .arg(&output_path_opt_ll)
        .arg("-o")
        .arg(&output_path_s)
        .status()
        .unwrap();

    let status = Command::new("clang")
        .arg(&output_path_opt_ll)
        .arg("-o")
        .arg(&output_name)
        // .arg("-g")
        .status()
        .unwrap();

    let end_time = start_time.elapsed().as_secs_f64();

    if status.success() {
        println!("{:>5}{} the program in {:.2}s", " ", "Finished".green().bold(), end_time);
        println!("{:>5}{} `{}`", " ", "Running".green().bold(), &output_name);

        if !cli.show_output_files {
            remove_file(&output_path_ll).unwrap();
            remove_file(&output_path_bc).unwrap();
            remove_file(&output_path_opt_ll).unwrap();
            remove_file(&output_path_s).unwrap();
        }

        Command::new(format!("./{}", output_name)).status().unwrap();
    } else {
        println!("{}: Compilation unsuccessful", "error".red());
    }
}
