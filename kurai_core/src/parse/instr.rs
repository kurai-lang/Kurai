use crate::value::Value;

#[derive(Debug)]
pub struct Instr {
    pub func_name: String,
    pub args: Vec<Value>,
}

impl Instr {
    pub fn printf(func_name: String, args: Vec<Value>) -> Self {
        Self {
            func_name,
            args,
        }
    }

    pub fn to_llvm_ir(&self) -> String {
        format!("call i32 (ptr, ...) @{}({:?})",
            self.func_name,
            self.args,
        )
    }  
}
