use inkwell::{context::Context, types::{BasicType, BasicTypeEnum}, values::BasicValueEnum};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Type {
    #[default]
    U8,
    U16,
    U32,
    U64,
    U128,

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
    Ptr(Box<Type>),
}

impl Type {
    pub fn to_llvm_type<'ctx>(&self, ctx: &'ctx Context) -> Option<BasicTypeEnum<'ctx>> {
        match *self {
            Type::I8 | Type::U8 => Some(ctx.i8_type().as_basic_type_enum()),
            Type::I16 | Type::U16 => Some(ctx.i16_type().as_basic_type_enum()),
            Type::I32 | Type::U32 => Some(ctx.i32_type().as_basic_type_enum()),
            Type::I64 | Type::U64 => Some(ctx.i64_type().as_basic_type_enum()),
            Type::I128 | Type::U128 => Some(ctx.i128_type().as_basic_type_enum()),

            Type::F16 => Some(ctx.f16_type().as_basic_type_enum()),
            Type::F32 => Some(ctx.f32_type().as_basic_type_enum()),
            Type::F64 => Some(ctx.f64_type().as_basic_type_enum()),
            Type::F128 => Some(ctx.f128_type().as_basic_type_enum()),

            Type::Bool => Some(ctx.bool_type().as_basic_type_enum()),
            Type::Str => Some(ctx.ptr_type(inkwell::AddressSpace::default()).as_basic_type_enum()),
            Type::Ptr(_) => {
                // let inner_ty = inner.to_llvm_type(ctx)?;
                Some(ctx.ptr_type(inkwell::AddressSpace::default()).as_basic_type_enum())
            }

            _ => None,
        }
    }

    pub fn to_llvm_value_zero<'ctx>(&self, ctx: &'ctx Context) -> BasicValueEnum<'ctx> {
        match *self {
            Type::I8 => ctx.i8_type().const_zero().into(),
            Type::I16 => ctx.i16_type().const_zero().into(),
            Type::I32 => ctx.i32_type().const_zero().into(),
            Type::I64 => ctx.i64_type().const_zero().into(),
            Type::I128 => ctx.i128_type().const_zero().into(),

            Type::F16 => ctx.f16_type().const_zero().into(),
            Type::F32 => ctx.f32_type().const_float(0.0).into(),
            Type::F64 => ctx.f64_type().const_float(0.0).into(),
            Type::F128 => ctx.f128_type().const_float(0.0).into(),

            Type::Bool => ctx.bool_type().const_zero().into(),

            Type::Ptr(_) | Type::Str => ctx
                .ptr_type(inkwell::AddressSpace::default())
                .const_null()
                .into(),

            _ => ctx.i64_type().const_zero().into(), // Fallback for unknowns
        }
    }

    pub fn from_llvm_type(typ: BasicTypeEnum) -> Option<Type> {
        if typ.is_int_type() {
            Some(Type::I32)
        } else if typ.is_float_type() {
            Some(Type::F32)
        } else if typ.is_pointer_type() {
            Some(Type::Str)
        } else {
            None
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();

        // for pointers
        if let Some(stripped) = s.strip_prefix('*') {
            return Some(Type::Ptr(Box::new(Type::from_str(stripped)?)));
        }

        match s {
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "u128" => Some(Type::U128),

            "i8" => Some(Type::I8),
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
