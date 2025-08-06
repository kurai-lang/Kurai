use colored::Colorize;

use crate::{error_kind::ErrorKind, span::Span};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub err_code: Option<String>,
    // pub span: Option<Span>,
    pub help: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            err_code: None,
            help: None,
        }
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_err_code(mut self, err_code: impl Into<String>) -> Self {
        self.err_code = Some(err_code.into());
        self
    }

    pub fn span(&self) -> &Span {
        match &self.kind {
            ErrorKind::Type { span, .. } => span,
            ErrorKind::Parse { span, .. } => span,
            ErrorKind::Codegen { span, .. } => span,
        }
    }
}

pub fn report_error(err: &Error, src_line: impl Into<String>) {
    let span = err.span();

    let (tag, msg): (&str, String) = match &err.kind {
        ErrorKind::Type { kind, .. } => ("type", kind.to_string()),
        ErrorKind::Codegen { kind, .. } => ("codegen", kind.to_string()),
        ErrorKind::Parse { kind, .. } => ("parse", kind.to_string()),
    };

    eprintln!("{} {}: {}", tag.red().bold(), "error".red().bold(), msg);
    eprintln!("--> {}:{}:{}", span.file, span.line, span.column);

    eprintln!("{:>3} |", " ");
    eprintln!("{:>3} | {}", span.line, src_line.into());

    let caret_line = format!(
        "{:>3} | {}{}",
        " ",
        " ".repeat(span.column),
        "^".repeat(span.width.max(1))
    );
    eprintln!("{caret_line}");

    if let Some(help) = &err.help {
        eprintln!("{}: {}", "help".cyan().bold(), help);
    }

    if let Some(err_code) = &err.err_code {
        eprintln!("{}: [{}]", "error code".bold(), err_code);
    }
}
