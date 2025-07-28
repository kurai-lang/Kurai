use colored::Colorize;
use inkwell::context::Context;
use kurai_codegen::codegen::CodeGen;
use kurai_core::scope::Scope;
use kurai_parser::parse::Parser;
use kurai_token::token::token::Token;

fn main() {
    // let code = r#"
    //     // INLINING TIME!
    //     #[inline]
    //     fn meow() void {
    //         let x = 0;
    //         if x == 0 {
    //             printf(1);
    //         } else {
    //             printf("hi");
    //         }
    //
    //         // return 0;
    //     }
    //
    //     fn main() i64 {
    //         meow();
    //         return 0;
    //     }
    // "#.to_string();

    let code = r#"
        // extern fn free_list_allocator_allocate(size: i64) *u8;

        // fn test() void {
        //     printf("hello");
        //     let x = 0;
        //     let y = 0;
        //     if x == 0 {
        //         y = 2;
        //     } else {
        //         y = 1;
        //     }
        // }

        fn main() void {
            let x  = 7;
        }
        "#.to_string();

    let context = Context::create();
    let mut parser = Parser::new().with_tokens(code.as_str());

    let mut scope = Scope::new();

    let mut discovered_modules: Vec<String> = Vec::new();
    let parsed_stmt_vec = parser.parse_out_vec_stmt();
    // println!("{:?}", parsed_stmt_vec);
    let parsed_expr_vec = parser.parse_out_vec_expr();
    let mut codegen = CodeGen::new(&context, &code, parser);

    // let mut discovered_modules = Vec::new();

    codegen.generate_code(
        parsed_stmt_vec,
        parsed_expr_vec.unwrap(), 
    );

    println!("{}", codegen.module.lock().unwrap().print_to_string().to_string().red());
    println!("{}", codegen.show_result());
}
