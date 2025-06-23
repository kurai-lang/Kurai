use colored::Colorize;
use inkwell::context::Context;
use kurai_codegen::codegen::CodeGen;
use kurai_core::scope::Scope;
use kurai_parser::{parse::{parse::{parse_out_vec_expr, parse_out_vec_stmt}, parse_block::BlockParserStruct, parse_stmt::StmtParserStruct}, GroupedParsers};
use kurai_parser_function::FunctionParserStruct;
use kurai_parser_import_decl::ImportParserStruct;
use kurai_parser_loop::LoopParserStruct;
use kurai_token::token::token::Token;

fn main() {
    // let code = r#"
    //     fn main() {
    //         let x = 5;
    //
    //         loop {
    //             printf("yes");
    //
    //             if (x >= 10) {
    //                 break;
    //             } else {
    //                 x = x + 1;
    //             }
    //         }
    //     }
    //     "#.to_string();

    let code = r#"
        #[test]
        fn meow() {
            let x = 0;
            if x == 0 {
                printf("meow");
            } else {
                printf("hi");
            }
        }

        fn main() {
            meow();
        }
    "#.to_string();

    let context = Context::create();
    let parsers = GroupedParsers::new(
        &StmtParserStruct,
        &FunctionParserStruct,
        &ImportParserStruct,
        &BlockParserStruct,
        &LoopParserStruct
    );

    let mut scope = Scope::new();
    let tokens = Token::tokenize(code.as_str());

    println!("{:?}", tokens);

    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parse_out_vec_stmt(
        &tokens,
        &mut discovered_modules,
        &parsers,
        &mut scope,
    );
    println!("{:?}", parsed_stmt_vec);
    let parsed_expr_vec = parse_out_vec_expr(&tokens);
    let mut codegen = CodeGen::new(&context);

    let mut discovered_modules = Vec::new();

    codegen.generate_code(
        parsed_stmt_vec,
        parsed_expr_vec.unwrap(), 
        &mut discovered_modules,
        &parsers,
        &mut scope,
    );

    println!("{}", codegen.module.lock().unwrap().print_to_string().to_string().red());
    println!("{}", codegen.show_result());
}
