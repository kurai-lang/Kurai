use clap::Parser;
use colored::Colorize;
use inkwell::context::Context;
use vyn_codegen::codegen::CodeGen;

use vyn_parser::parse::Parser as VyncParser;
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

    #[arg(short = 'L', long = "lib-dir", num_args = 1.., value_name = "DIR")]
    pub lib_dirs: Vec<String>,

    #[arg(short = 'l', long = "link-lib", num_args = 1.., value_name = "LIB")]
    pub link_libs: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
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

    let file_path = cli.input;
    let code = fs::read_to_string(file_path).unwrap(); 

    // TODO: Scopin xD
    // let mut scope = Scope::new();

    // let tokens = Token::tokenize(code.as_str());
    let mut parser = VyncParser::new().with_src_code(code.as_str());

    let parsed_stmt_vec = parser.parse_out_vec_stmt();
    let parsed_expr_vec = parser.parse_out_vec_expr();
    let mut codegen = CodeGen::new(&context, &code, parser).init();
    // codegen.printf("hi");

    // pub fn generate_code(&self, parsed_stmt: Vec<Stmt>, context: &'ctx Context, builder: &Builder, module: &mut Module<'ctx>)
    #[cfg(debug_assertions)]
    {
        println!("STATEMENTS:\n{parsed_stmt_vec:?}");
        println!("EXPRESSIONS:\n{parsed_expr_vec:?}");
        println!("VARIABLES:\n{:?}", codegen.variables);
    }

    codegen.generate_code(
        parsed_stmt_vec,
         
    );

    // == UNUSED == 
    // let output_path_opt_bc = format!("{output_name}_opt.bc");
    // let output_path_o = format!("{output_name}.o");

    let output_path_ll = format!("{output_name}.ll");
    let output_path_opt_ll = format!("{output_name}_opt.ll");
    let output_path_bc = format!("{output_name}.bc");
    let output_path_s = format!("{output_name}.s");

    let result = codegen.show_result();

    let mut llvm_ir_code_file = File::create(&output_path_ll).unwrap();
    llvm_ir_code_file.write_all(result.as_bytes()).unwrap();

    // WHATS THIS NAME LMFAO
    let install_llvm_bro = "install llvm bro";
    let start_time = Instant::now();
    Command::new("llvm-as-18")
        .arg(&output_path_ll)
        .arg("-o")
        .arg(&output_path_bc)
        .status()
        .expect(install_llvm_bro);
    Command::new("opt")
        .arg(format!("-O{}", cli.opt_level))
        .arg("-S")
        .arg(&output_path_bc)
        .arg("-o")
        .arg(&output_path_opt_ll)
        .status()
        .expect(install_llvm_bro);
    Command::new("llc")
        .arg(&output_path_opt_ll)
        .arg("-o")
        .arg(&output_path_s)
        .status()
        .expect(install_llvm_bro);

    let mut binding = Command::new("clang");
    let cmd = binding
        .arg(&output_path_ll)
        .arg("-o")
        .arg(&output_name);
        // .arg("-g")

        //lib_dirs, link_libs
    for dir in &cli.lib_dirs {
        cmd.arg(format!("-L{dir}"));
        println!("{}{}{}: searching for libraries in: {}", "[".green(), "external library paths".green().bold(), "]".green(), dir);
    }

    for lib in &cli.link_libs {
        cmd.arg(format!("-l{lib}"));
        println!("{}{}{}: linking external libraries: {}", "[".green(), "external libraries".green().bold(), "]".green(), lib);
    }

    let status = cmd.status().unwrap();

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

        Command::new(format!("./{output_name}")).status().unwrap();
    } else {
        println!("{}: Compilation unsuccessful", "error".red());
    }
}
