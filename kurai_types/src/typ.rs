use inkwell::{context::Context, types::{AnyType, BasicType, BasicTypeEnum, PointerType, VoidType}};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    F16,
    F32,
    F64,
    F128,
    Bool,
    Str,
    Unknown,
    Var,
    Void,
}

impl Type {
    pub fn to_llvm_type<'ctx>(&self, ctx: &'ctx Context) -> Option<BasicTypeEnum<'ctx>> {
        match *self {
            Type::I8 => Some(ctx.i8_type().as_basic_type_enum()),
            Type::I16 => Some(ctx.i16_type().as_basic_type_enum()),
            Type::I32 => Some(ctx.i32_type().as_basic_type_enum()),
            Type::I64 => Some(ctx.i64_type().as_basic_type_enum()),
            Type::I128 => Some(ctx.i128_type().as_basic_type_enum()),

            Type::F16 => Some(ctx.f16_type().as_basic_type_enum()),
            Type::F32 => Some(ctx.f32_type().as_basic_type_enum()),
            Type::F64 => Some(ctx.f64_type().as_basic_type_enum()),
            Type::F128 => Some(ctx.f128_type().as_basic_type_enum()),

            Type::Bool => Some(ctx.bool_type().as_basic_type_enum()),
            Type::Str => Some(ctx.ptr_type(inkwell::AddressSpace::default()).as_basic_type_enum()),

            _ => None,
        }
    }

    pub fn from_llvm_type(typ: BasicTypeEnum) -> Option<Type> {
        if typ.is_int_type() {
            Some(Type::I32)
        } else if typ.is_float_type() {
            // FIXME: i havent even implemented float datatype yet fr
            Some(Type::F32)
        } else if typ.is_pointer_type() {
            Some(Type::Str)
        } else {
            None
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "i128" => Some(Type::I128),

            "f16" => Some(Type::F16),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            "f128" => Some(Type::F128),

            "bool" => Some(Type::Bool),
            "str" => Some(Type::Str),
            "var" => Some(Type::Var), // Not sure why "var" is here but ok..
            "void" => Some(Type::Void),
            _ => None,
        }
    }
}
