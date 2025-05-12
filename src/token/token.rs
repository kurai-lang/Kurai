#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Int,

    Id(String),
    Number(i64),
    Float(f32),
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
}

impl Token {
    pub fn tokenize(code: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        let mut iter = code.chars().peekable();

        while let Some(ch) = iter.next() {
            match ch {
                '=' => tokens.push(Token::Equal),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Dash),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                ';' => tokens.push(Token::Semicolon),
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
                ',' => tokens.push(Token::Comma),
                '0'..='9' => {
                    current.push(ch);
                    while let Some(&next_ch) = iter.peek() {
                        #[allow(warnings)]
                        if next_ch.is_digit(10) {
                            current.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Number(current.parse::<i64>().unwrap()));
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
                        "int" => tokens.push(Token::Int),
                        "let" => tokens.push(Token::Let),
                        "fn" => tokens.push(Token::Function),
                        "use" => tokens.push(Token::Import),
                        "as" => tokens.push(Token::As),
                        _ => tokens.push(Token::Id(current.clone())),
                    }
                    current.clear();
                }
                ' ' | '\n' | '\t' => {}, // skip whitespace
                _ => {
                    // unknown char? throw hands (or error)
                    panic!("Unexpected character: {}", ch);
                }
            }
        }

        // if it was `int `
        tokens
    }
}
