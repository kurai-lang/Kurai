use inkwell::{
    builder::BuilderError,
    values::BasicValueEnum,
    values::BasicValue,
};
use crate::codegen::codegen::CodeGen;

use kurai_core::value::Value;

impl<'ctx> CodeGen<'ctx> {
    pub fn const_from_value(&self, value: &Value) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        match value {
            Value::Int(i) => Ok(BasicValueEnum::IntValue(self.context.i64_type().const_int(*i as u64, true))),
            Value::Float(f) => Ok(BasicValueEnum::FloatValue(self.context.f64_type().const_float(*f))),
            Value::Bool(b) => Ok(self.context.bool_type().const_int(*b as u64, false).into()),
            Value::Str(s) => { 
                let gv = self.builder.build_global_string_ptr(s, "str")?;
                Ok(gv.as_pointer_value().as_basic_value_enum())
            }
        }
    }
}
