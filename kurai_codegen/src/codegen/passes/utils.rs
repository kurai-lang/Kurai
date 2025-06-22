use colored::Colorize;
use inkwell::values::FunctionValue;
use kurai_core::scope::Scope;
use kurai_parser::{BlockParser, FunctionParser, ImportParser, LoopParser, StmtParser};
use kurai_stmt::stmt::Stmt;

use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn get_or_compile_function(
        &mut self,
        name: &str,
        discovered_modules: &mut Vec<String>,
        stmt_parser: &dyn StmtParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        block_parser: &dyn BlockParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Option<FunctionValue<'ctx>> {
        if let Some((modname, funcname)) = Self::split_module_function_name(name) {
        // found modname and funcname? compile.
            self.get_function_from_module(
                modname,
                funcname,
                discovered_modules,
                stmt_parser,
                fn_parser,
                import_parser,
                block_parser, 
                loop_parser,
                scope
            )
        } else {
            // already compiled? ok reuse it
            self.module.lock().unwrap().get_function(name)
        }
    }

    pub fn split_module_function_name(name: &str) -> Option<(&str, &str)> {
        let mut parts = name.split("::");
        let modname = parts.next().unwrap();
        let funcname = parts.next().unwrap();

        Some((modname, funcname))
    }

    pub fn get_function_from_module(
        &mut self,
        modname: &str,
        funcname: &str,
        discovered_modules: &mut Vec<String>,
        stmt_parser: &dyn StmtParser,
        fn_parser: &dyn FunctionParser,
        import_parser: &dyn ImportParser,
        block_parser: &dyn BlockParser,
        loop_parser: &dyn LoopParser,
        scope: &mut Scope,
    ) -> Option<FunctionValue<'ctx>> {
        if let Some(mod_stmts) = self.loaded_modules.get(modname) {
            let already_compiled: Option<FunctionValue<'ctx>> = self.module.lock().unwrap().get_function(funcname);

            if already_compiled.is_some() {
                already_compiled
            } else {
                for stmt in mod_stmts {
                    if let Stmt::FnDecl { name: fname, .. } = stmt {
                        if fname == funcname {
                            #[cfg(debug_assertions)]
                            {
                                use colored::Colorize;

                                println!("
                                    {} `{}` from `{}` is now being compiled", "Compiling function".green(),
                                    funcname, modname);
                            }

                            self.generate_code(
                                vec![stmt.clone()], 
                                vec![],
                                discovered_modules,
                                stmt_parser,
                                fn_parser,
                                import_parser,
                                block_parser,
                                loop_parser,
                                scope
                            );
                            break;
                        }
                    }
                }
                // try again after compiling
                self.module.lock().unwrap().get_function(funcname)
            }
        } else {
            println!("{}: Module not found: `{}`", "error".red().bold(), modname);
            None
        }
    }
}
