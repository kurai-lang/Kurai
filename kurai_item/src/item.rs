use kurai_attr::attribute::Attribute;
use kurai_expr::expr::Expr;
use kurai_stmt::stmt::IfBranch;
use kurai_typedArg::typedArg::TypedArg;
use kurai_types::typ::Type;

pub enum Item {
    FnDecl { name: String, args: Vec<TypedArg>, body: Vec<Expr>, ret_type: Type, attributes: Vec<Attribute> },
    Import  { path: Vec<String>, nickname: Option<String>, is_glob: bool },
    Loop    { body: Vec<Expr> },
    If      { branches: Vec<IfBranch>, else_: Option<Vec<Expr>> },
    Return  { expr: Option<Box<Expr>> },
    Expr    (Expr),
    Block   (Vec<Item>),
    Break,
}
