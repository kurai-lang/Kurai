#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
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
