use kurai_types::typ::Type;

#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    If,
    Else,

    // Int
    Type(Type),

    Id(String),
    Number(i64),
    Float(f64),
    Bool(bool),
    StringLiteral(String),
    Equal,

    Plus,
    Dash,
    Star,
    Slash,
    Semicolon,

    Quote,
    DoubleQuotes,
    OpenParenthese,
    CloseParenthese,
    Comma,
    OpenBracket,
    CloseBracket,

    Function,
    Import,
    As,

    Colon,
    Less,
    LessEqual,
    GreaterEqual,
    Greater,
    BangEqual,
    EqualEqual,
    Bang,
    Range,
    For,
    While,
    In,
    Loop,
    Break,
    Dot,
    Hash,
    CloseSquareBracket,
    OpenSquareBracket,
    Comment,
    Return,
}

impl Token {
    pub fn tokenize(code: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        let mut iter = code.chars().peekable();

        while let Some(ch) = iter.next() {
            match ch {
                '=' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::EqualEqual)
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Dash),
                '*' => tokens.push(Token::Star),
                '/' => {
                    if let Some('/') = iter.peek() {
                        iter.next();
                        while let Some(&next_char) = iter.peek() {
                            if next_char == '\n' {
                                break;
                            }
                            iter.next();
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                ';' => tokens.push(Token::Semicolon),
                ':' => tokens.push(Token::Colon),
                '!' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::BangEqual);
                    } else {
                        tokens.push(Token::Bang)
                    }
                }
                '\'' => tokens.push(Token::Quote),
                '"' => {
                    let mut string_literal = String::new();

                    while let Some(&next_char) = iter.peek() {
                        if next_char == '"' { // Another quote found? welp, thats the ending
                            iter.next();
                            break;
                        } else {
                            string_literal.push(next_char);
                            iter.next();
                        }
                    }

                    tokens.push(Token::StringLiteral(string_literal));
                }
                '(' => tokens.push(Token::OpenParenthese),
                ')' => tokens.push(Token::CloseParenthese),
                '{' => tokens.push(Token::OpenBracket),
                '}' => tokens.push(Token::CloseBracket),
                '[' => tokens.push(Token::OpenSquareBracket),
                ']' => tokens.push(Token::CloseSquareBracket),
                ',' => tokens.push(Token::Comma),
                '#' => tokens.push(Token::Hash),
                '<' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::LessEqual);
                    } else {
                        tokens.push(Token::Less);
                    }
                }
                '>' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        tokens.push(Token::GreaterEqual);
                    } else {
                        tokens.push(Token::Greater);
                    }
                }
                '.' => {
                    if let Some('.') = iter.peek() {
                        iter.next();
                        tokens.push(Token::Range);
                    } else {
                        tokens.push(Token::Dot);
                    }
                }
                '0'..='9' => {
                    current.push(ch);
                    let mut is_float = false;

                    while let Some(&next_ch) = iter.peek() {
                        #[allow(warnings)]
                        if next_ch.is_digit(10) {
                            current.push(iter.next().unwrap());
                        } else if next_ch == '.' {
                            let mut clone = iter.clone();
                            clone.next();
                            if let Some(&after_dot) = clone.peek() {
                                if after_dot.is_digit(10) {
                                    is_float = true;
                                    current.push(iter.next().unwrap());

                                    while let Some(&float_ch) = iter.peek() {
                                        if float_ch.is_digit(10) {
                                            current.push(iter.next().unwrap());
                                        } else {
                                            break;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    if is_float {
                        tokens.push(Token::Float(current.parse::<f64>().unwrap()));
                    } else {
                        tokens.push(Token::Number(current.parse::<i64>().unwrap()));
                    }
                    current.clear(); // Reset for next token
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    current.push(ch);
                    while let Some(&next_ch) = iter.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_'{
                            current.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    // keywords i guess
                    match current.as_str() {
                        "let" => tokens.push(Token::Let),
                        "fn" => tokens.push(Token::Function),
                        "use" => tokens.push(Token::Import),
                        "as" => tokens.push(Token::As),
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        "true" => tokens.push(Token::Bool(true)),
                        "false" => tokens.push(Token::Bool(false)),
                        "for" => tokens.push(Token::For),
                        "loop" => tokens.push(Token::Loop),
                        "while" => tokens.push(Token::While),
                        "in" => tokens.push(Token::In),
                        "break" => tokens.push(Token::Break),
                        "return" => tokens.push(Token::Return),

                        // NOTE: Data types
                        "i8" => tokens.push(Token::Type(Type::I8)),
                        "i16" => tokens.push(Token::Type(Type::I16)),
                        "i32" => tokens.push(Token::Type(Type::I32)),
                        "i64" => tokens.push(Token::Type(Type::I64)),
                        "i128" => tokens.push(Token::Type(Type::I128)),

                        "f32" => tokens.push(Token::Type(Type::F32)),
                        "f64" => tokens.push(Token::Type(Type::F64)),
                        "f128" => tokens.push(Token::Type(Type::F128)),

                        "bool" => tokens.push(Token::Type(Type::Bool)),
                        "void" => tokens.push(Token::Type(Type::Void)),
                        _ => tokens.push(Token::Id(current.clone())),
                    }
                    current.clear();
                }
                ' ' | '\n' | '\t' => {}, // skip whitespace
                _ => {
                    // unknown char? throw hands (or error)
                    panic!("Unexpected character {}", ch);
                }
            }
        }

        // if it was `int `
        tokens
    }
}
