#[derive(Debug, PartialEq, Clone)]
pub enum AttrArg {
  Bool(bool),
  Str(String),
  Int(i64),
  // TODO: Expand whenever possible
}

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    Test,
    Simple(String),
    WithArgs(String, Vec<AttrArg>),
}
