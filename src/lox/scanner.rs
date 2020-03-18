use crate::lox::source;
use crate::lox::tokens;

use std::iter::Peekable;
use std::str::Chars;

type Result = std::result::Result<tokens::Token, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    location: source::Location,
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ScannerError<{}>: {}", self.location, self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None // Generic for now
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    location: source::Location,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result;

    fn next(&mut self) -> Option<Result> {
        self.next_token()
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: Peekable<Chars<'a>>) -> Scanner {
        Scanner {
            source: source,
            location: source::Location { line: 1, column: 0 },
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            if c == '\n' {
                self.location.line += 1;
                self.location.column = 0;
            } else {
                self.location.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn is_at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    fn skip_whitespace(&mut self) -> () {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                let _ = self.advance();
            } else {
                break;
            }
        }
    }

    // The book just calls this "match", but that's reserved
    fn consume_if_match(&mut self, expected: char) -> Option<char> {
        if let Some(&c) = self.peek() {
            if &expected == &c {
                return self.advance();
            }
        }
        None
    }

    fn consume_comment(&mut self) -> Result {
        let mut value: String = String::new();
        while let Some(c) = self.advance() {
            value.push(c);
            if c == '\n' {
                break;
            }
        }
        Ok(tokens::Token::new(
            tokens::TokenType::Comment(value),
            self.location,
        ))
    }

    // The book just calls this "string" but that seems ambiguous
    fn consume_string(&mut self) -> Result {
        let mut value: String = String::new();
        while let Some(c) = self.peek() {
            if c == &'\"' {
                break;
            } else {
                if let Some(c) = self.advance() {
                    value.push(c)
                }
            }
        }
        if self.is_at_end() {
            Err(Error {
                location: self.location,
                message: format!("Unterminated string literal \"{}", value),
            })
        } else {
            let _ = self.advance();
            Ok(tokens::Token::new(
                tokens::TokenType::Str(value),
                self.location,
            ))
        }
    }

    fn consume_digits(&mut self) -> String {
        let mut value: String = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                if let Some(d) = self.advance() {
                    value.push(d);
                }
            } else {
                break;
            }
        }
        value
    }

    fn consume_number(&mut self, start: char) -> Result {
        let mut value: String = String::new();
        value.push(start);
        value.push_str(&self.consume_digits());
        // Out of digit chars. Is there a decimal?
        if let Some(d) = self.consume_if_match('.') {
            value.push(d);
            // If there was a decimal point, there must be a digit followin
            value.push_str(&self.consume_digits());
        }
        if value.ends_with('.') {
            return Err(Error {
                location: self.location,
                message: format!("Incomplete number literal {}", value),
            });
        }
        match value.parse() {
            Ok(n) => Ok(tokens::Token::new(
                tokens::TokenType::Number(n),
                self.location,
            )),
            Err(why) => Err(Error {
                location: self.location,
                message: why.to_string(),
            }),
        }
    }

    fn consume_identifier(&mut self, first: char) -> Result {
        let mut value = String::new();
        value.push(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == &'_' {
                if let Some(c) = self.advance() {
                    value.push(c);
                }
            } else {
                break;
            }
        }
        Ok(tokens::Token::from_identifier(&value, self.location))
    }

    pub fn next_token(&mut self) -> Option<Result> {
        self.skip_whitespace();
        if let Some(c) = self.advance() {
            match c {
                // Single characters
                '(' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::LeftParen,
                    self.location,
                ))),
                ')' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::RightParen,
                    self.location,
                ))),
                '{' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::LeftBrace,
                    self.location,
                ))),
                '}' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::RightBrace,
                    self.location,
                ))),
                ',' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::Comma,
                    self.location,
                ))),
                '.' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::Dot,
                    self.location,
                ))),
                '-' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::Minus,
                    self.location,
                ))),
                '+' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::Plus,
                    self.location,
                ))),
                ';' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::Semicolon,
                    self.location,
                ))),
                '*' => Some(Ok(tokens::Token::new(
                    tokens::TokenType::Star,
                    self.location,
                ))),
                // Double characters
                '!' => {
                    if let Some(_) = self.consume_if_match('=') {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::BangEqual,
                            self.location,
                        )))
                    } else {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::Bang,
                            self.location,
                        )))
                    }
                }
                '=' => {
                    if let Some(_) = self.consume_if_match('=') {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::EqualEqual,
                            self.location,
                        )))
                    } else {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::Equal,
                            self.location,
                        )))
                    }
                }
                '<' => {
                    if let Some(_) = self.consume_if_match('=') {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::LessEqual,
                            self.location,
                        )))
                    } else {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::Less,
                            self.location,
                        )))
                    }
                }
                '>' => {
                    if let Some(_) = self.consume_if_match('=') {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::GreaterEqual,
                            self.location,
                        )))
                    } else {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::Greater,
                            self.location,
                        )))
                    }
                }
                // Everything else
                // Slash is special because comments; it's 1 character if it is a token at all
                '/' => {
                    if let Some(_) = self.consume_if_match('/') {
                        Some(self.consume_comment())
                    } else {
                        Some(Ok(tokens::Token::new(
                            tokens::TokenType::Slash,
                            self.location,
                        )))
                    }
                }
                '\"' => Some(self.consume_string()),
                x if x.is_alphabetic() || x == '_' => Some(self.consume_identifier(x)),
                n if n.is_digit(10) => Some(self.consume_number(n)),
                _ => Some(Err(Error {
                    location: self.location,
                    message: format!(
                        "Illegal token: {}",
                        tokens::Token::new(tokens::TokenType::Illegal(c), self.location)
                    ),
                })),
            }
        } else {
            None
        }
    }
}
