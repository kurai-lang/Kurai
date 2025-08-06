#[derive(Debug, Clone)]
pub struct Span {
    pub file: String,
    pub range: usize,
    pub line: usize,
    pub column: usize,
    pub width: usize,
}

pub struct SpanBuilderNoLocation {
    file: String,
    range: usize,
    width: usize,
}

impl Span {
    pub fn new(file: impl Into<String>) -> SpanBuilderNoLocation {
        SpanBuilderNoLocation {
            file: file.into(),
            range: 0,
            width: 0,
        }
    }
}

impl SpanBuilderNoLocation {
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = file.into();
        self
    }

    pub fn with_range(mut self, range: usize) -> Self {
        self.range = range;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_line_column(self, line: usize, column: usize) -> Span {
        Span {
            file: self.file,
            range: self.range,
            width: self.width,
            line,
            column,
        }
    }
}
