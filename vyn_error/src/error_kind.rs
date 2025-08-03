use colored::Colorize;
use vyn_token::token::token::Token;
use vyn_types::value::Value;

use crate::span::Span;

#[derive(Debug)]
pub enum TypeErrorKind {
    UnsupportedReturnType(String),
    TypeMismatch { expected: String, found: String },
    UndefinedVariable(String),
}

impl std::fmt::Display for TypeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeErrorKind::UnsupportedReturnType(t) => write!(f, "unsupported return type `{}`", t),
            TypeErrorKind::TypeMismatch { expected, found } => write!(f, "type mismatch: expected `{}`, found `{}`", expected, found),
            TypeErrorKind::UndefinedVariable(name) => write!(f, "undefined variable `{}`", name),
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    ExpectedToken(String),
    FnCallFailed(String),
    FnDeclFailed(String),
    VarDeclFailed(String),
    VarAssignFailed { name: String, value: Value },
    LoopFailed { loop_type: String, reason: String },
}

impl std::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::ExpectedToken(err) => write!(f, "{}", err),
            ParseErrorKind::FnCallFailed(name) => write!(f, "{}: function named `{}` failed to be called", 
                                                                "function call error".bold(), name),
            ParseErrorKind::FnDeclFailed(name) => write!(f, "{}: function named `{}` failed to be declared or defined",
                                                                "function declaration error".bold(), name),
            ParseErrorKind::VarDeclFailed(name) => write!(f, "{}: variable named `{}` failed to be declared",
                                                                "variable declaration error".bold(), name),
            ParseErrorKind::VarAssignFailed { name, value } => write!(f, "{}: variable named `{}` failed to be assigned `{:?}`", 
                                                                "variable assignation error".bold(), name, value),
            ParseErrorKind::LoopFailed { loop_type, reason } => write!(f, "{}: a `{}` failed with reason: {}", 
                                                                "loop error".bold(), loop_type, reason),
        }
    }
}

#[derive(Debug)]
pub enum CodegenErrorKind {
    LowerExprFailed(String),
    LowerStmtFailed(String),
}

impl std::fmt::Display for CodegenErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenErrorKind::LowerExprFailed(err) => write!(f, "{}", err),
            CodegenErrorKind::LowerStmtFailed(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Type {
        kind: TypeErrorKind,
        span: Span,
    },
    Parse {
        kind: ParseErrorKind,
        span: Span,
    },
    Codegen {
        kind: CodegenErrorKind,
        span: Span,
    },
}

impl ErrorKind {
    pub fn span(&self) -> &Span {
        match self {
            ErrorKind::Type { span, .. }
            | ErrorKind::Parse { span, .. }
            | ErrorKind::Codegen { span, .. } => span,
        }
    }
}
