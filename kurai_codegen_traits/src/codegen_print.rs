use kurai_typedArg::typedArg::TypedArg;
use inkwell::values::BasicValueEnum;

pub trait CodeGenPrint<'ctx> {
    fn printf_format(&self, args: &Vec<TypedArg>, id: usize) -> Vec<BasicValueEnum<'ctx>>;
    fn compile_int(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>>;
    fn compile_str(&self, arg: &TypedArg, id: usize, index: usize) -> Option<BasicValueEnum<'ctx>>;
    fn compile_id(&self, arg: &TypedArg) -> Option<BasicValueEnum<'ctx>>;
    fn printf(&mut self, args: &Vec<TypedArg>) -> Result<(), String>;
    fn import_printf(&mut self) -> Result<(), String>;
}
