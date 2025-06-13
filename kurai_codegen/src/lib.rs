use inkwell::values::BasicValueEnum;
use kurai_typedArg::typedArg::TypedArg;

use crate::codegen::CodeGen;

pub mod codegen;
pub mod registry;

pub trait CodeGenPrint<'ctx> {
    fn printf_format(&self, args: &Vec<TypedArg>, id: usize) -> Vec<BasicValueEnum<'ctx>>;
    fn compile_int(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>>;
    fn compile_str(&self, arg: &TypedArg, id: usize, index: usize) -> Option<BasicValueEnum<'ctx>>;
    fn compile_id(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>>;
    fn printf(&mut self, args: &Vec<TypedArg>) -> Result<(), String>;
    fn import_printf(&mut self) -> Result<(), String>;
}

// pub struct CodeGenPrintStruct<'ctx> {
//     pub codegen: CodeGen<'ctx>,
// }
//
// impl<'ctx> CodeGenPrint<'ctx> for CodeGenPrintStruct<'ctx> {
//     fn printf(&mut self, args: &Vec<TypedArg>) -> Result<(), String> {
//         self.codegen.printf(args)
//     }
//
//     fn import_printf(&mut self) -> Result<(), String> {
//         self.codegen.import_printf()
//     }
//
//     fn printf_format(&self.codegen, args: &Vec<TypedArg>, id: usize) -> Vec<BasicValueEnum<'ctx>> {
//         self.codegen.printf_format(args, id)
//     }
//
//     fn compile_int(&self.codegen, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
//         self.codegen.compile_int(arg)
//     }
//
//     fn compile_str(&self.codegen, arg: &TypedArg, id: usize, index: usize) -> Option<BasicValueEnum<'ctx>> {
//         self.codegen.compile_str(arg, id, index)
//     }
//
//     fn compile_id(&self.codegen, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>> {
//         self.codegen.compile_id(arg)
//     }
// }
