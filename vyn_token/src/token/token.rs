use std::{iter::Peekable, str::Chars};

use vyn_types::typ::Type;

use crate::token::spanned_token::SpannedToken;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    If,
    Else,

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
    Extern,
}

fn advance(iter: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Option<char> {
    let ch = iter.next()?;

    match ch {
        '\n' => {
            *line += 1;
            *column = 1;
        }
        _ => { 
            *column += 1;
        },
    }

    Some(ch)
}

impl Token {
    pub fn tokenize(code: &str) -> (Vec<Token>, Vec<SpannedToken>) {
        let mut tokens = Vec::new();
        let mut spanned = Vec::new();

        let mut current = String::new();
        let mut line = 1;
        let mut column = 1;

        let mut iter = code.chars().peekable();

        while let Some(ch) = advance(&mut iter, &mut line, &mut column) {
            let start_line = line;
            let start_column = column;

            let mut push = |token: Token, width: usize| {
                tokens.push(token.clone());
                spanned.push(SpannedToken {
                    token,
                    line: start_line,
                    column: start_column,
                    width,
                });
            };

            match ch {
                '=' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        push(Token::EqualEqual, 2);
                    } else {
                        push(Token::Equal, 1);
                    }
                }
                '+' => push(Token::Plus, 1),
                '-' => push(Token::Dash, 1),
                '*' => push(Token::Star, 1),
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
                        push(Token::Slash, 1);
                    }
                }
                ';' => push(Token::Semicolon, 1),
                ':' => push(Token::Colon, 1),
                '!' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        push(Token::BangEqual, 2);
                    } else {
                        push(Token::Bang, 1);
                    }
                }
                '\'' => push(Token::Quote, 1),
                '"' => {
                    let mut string_literal = String::new();
                    let mut width = 1; // for the opening quote

                    while let Some(&next_char) = iter.peek() {
                        width += 1;
                        if next_char == '"' {
                            iter.next();
                            break;
                        } else {
                            string_literal.push(next_char);
                            iter.next();
                        }
                    }

                    width += 1; // for the closing quote
                    push(Token::StringLiteral(string_literal), width);
                }
                '(' => push(Token::OpenParenthese, 1),
                ')' => push(Token::CloseParenthese, 1),
                '{' => push(Token::OpenBracket, 1),
                '}' => push(Token::CloseBracket, 1),
                '[' => push(Token::OpenSquareBracket, 1),
                ']' => push(Token::CloseSquareBracket, 1),
                ',' => push(Token::Comma, 1),
                '#' => push(Token::Hash, 1),
                '<' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        push(Token::LessEqual, 2);
                    } else {
                        push(Token::Less, 1);
                    }
                }
                '>' => {
                    if let Some('=') = iter.peek() {
                        iter.next();
                        push(Token::GreaterEqual, 2);
                    } else {
                        push(Token::Greater, 1);
                    }
                }
                '.' => {
                    if let Some('.') = iter.peek() {
                        iter.next();
                        push(Token::Range, 2);
                    } else {
                        push(Token::Dot, 1);
                    }
                }
                '0'..='9' => {
                    current.push(ch);
                    let mut is_float = false;
                    let mut width = 1;

                    while let Some(&next_ch) = iter.peek() {
                        if next_ch.is_digit(10) {
                            current.push(iter.next().unwrap());
                            width += 1;
                        } else if next_ch == '.' {
                            let mut clone = iter.clone();
                            clone.next();
                            if let Some(&after_dot) = clone.peek() {
                                if after_dot.is_digit(10) {
                                    is_float = true;
                                    current.push(iter.next().unwrap());
                                    width += 1;

                                    while let Some(&float_ch) = iter.peek() {
                                        if float_ch.is_digit(10) {
                                            current.push(iter.next().unwrap());
                                            width += 1;
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
                        push(Token::Float(current.parse::<f64>().unwrap()), width);
                    } else {
                        push(Token::Number(current.parse::<i64>().unwrap()), width);
                    }
                    current.clear();
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    current.push(ch);
                    let mut width = 1;

                    while let Some(&next_ch) = iter.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            current.push(iter.next().unwrap());
                            width += 1;
                        } else {
                            break;
                        }
                    }

                    let token = match current.as_str() {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        "use" => Token::Import,
                        "as" => Token::As,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        "for" => Token::For,
                        "loop" => Token::Loop,
                        "while" => Token::While,
                        "in" => Token::In,
                        "break" => Token::Break,
                        "return" => Token::Return,
                        "extern" => Token::Extern,
                        "i8" => Token::Type(Type::I8),
                        "i16" => Token::Type(Type::I16),
                        "i32" => Token::Type(Type::I32),
                        "i64" => Token::Type(Type::I64),
                        "i128" => Token::Type(Type::I128),
                        "u8" => Token::Type(Type::U8),
                        "u16" => Token::Type(Type::U16),
                        "u32" => Token::Type(Type::U32),
                        "u64" => Token::Type(Type::U64),
                        "u128" => Token::Type(Type::U128),
                        "f32" => Token::Type(Type::F32),
                        "f64" => Token::Type(Type::F64),
                        "f128" => Token::Type(Type::F128),
                        "bool" => Token::Type(Type::Bool),
                        "void" => Token::Type(Type::Void),
                        _ => Token::Id(current.clone()),
                    };

                    push(token, width);
                    current.clear();
                }
                ' ' | '\n' | '\t' => {}, // skip whitespace
                _ => panic!("Unexpected character {}", ch),
            }
        }

        (tokens, spanned)
    }
}
