use inkwell::{context::Context, types::{BasicType, BasicTypeEnum}};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    pub fn get_value(&self) -> String {
        match self {
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Str(s) => s.clone(),
        }
    }

    pub fn to_llvm_type<'ctx>(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            Value::Int(_) => ctx.i64_type().as_basic_type_enum(),
            Value::Float(_) => ctx.f64_type().as_basic_type_enum(),
            Value::Bool(_) => ctx.bool_type().as_basic_type_enum(),
            Value::Str(_) => ctx.i8_type().ptr_type(inkwell::AddressSpace::default()).as_basic_type_enum(),
        }
    }

    // fn to_str(&self) -> &str {
    //     match self {
    //         Value::Int(i) => {
    //             let i = i.to_string();
    //             &i[..]
    //         }
    //         Value::Float(f) => {
    //             let f = f.to_string();
    //             &f[..]
    //         },
    //         Value::Bool(b) => {
    //             let b = b.to_string();
    //             &b[..]
    //         },
    //         Value::Str(s) => &s[..],
    //     }
    // }
}
