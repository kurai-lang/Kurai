use inkwell::{context::Context, types::{BasicType, BasicTypeEnum, PointerType}};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Unknown,
    Var,
}

impl Type {
    pub fn to_llvm_type<'ctx>(&self, ctx: &'ctx Context) -> Option<BasicTypeEnum<'ctx>> {
        match *self {
            Type::Int => Some(ctx.i64_type().as_basic_type_enum()),
            Type::Float => Some(ctx.f64_type().as_basic_type_enum()),
            Type::Bool => Some(ctx.bool_type().as_basic_type_enum()),
            Type::Str => Some(ctx.i8_type().ptr_type(inkwell::AddressSpace::default()).as_basic_type_enum()),
            _ => None,
        }
    }

    pub fn from_llvm_type(typ: BasicTypeEnum) -> Option<Type> {
        if typ.is_int_type() {
            Some(Type::Int)
        } else if typ.is_float_type() {
            // FIXME: i havent even implemented float datatype yet fr
            Some(Type::Float)
        } else if typ.is_pointer_type() {
            Some(Type::Str)
        } else {
            None
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "bool" => Some(Type::Bool),
            "str" => Some(Type::Str),
            "var" => Some(Type::Var),      // if you have this
            _ => None,
        }
    }
}
