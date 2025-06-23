use kurai_types::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    Test,
    Simple(String),
    WithArgs {
        name: String,
        args: Vec<AttrArg>
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttrArg {
    Positional(String),
    Named(String, Value),
}
