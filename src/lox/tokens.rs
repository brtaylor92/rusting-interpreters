use crate::lox::source;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType, // This is just "type" in the book but that's reserved for Rust
    pub location: source::Location, // The book only tracks line number but where's the fun in that
                               // The book uses 2 other fields here to track lexeme and literal value
                               // Lexeme doesn't seem that useful and literal values can be embedded in the TokenTypes which require them
}

impl Token {
    pub fn new(token_type: TokenType, location: source::Location) -> Token {
        Token {
            token_type: token_type,
            location: location,
        }
    }

    pub fn from_identifier(identifier: &str, location: source::Location) -> Token {
        match identifier {
            "and" => Token::new(TokenType::And, location),
            "class" => Token::new(TokenType::Class, location),
            "else" => Token::new(TokenType::Else, location),
            "false" => Token::new(TokenType::False, location),
            "for" => Token::new(TokenType::For, location),
            "fun" => Token::new(TokenType::Fun, location),
            "if" => Token::new(TokenType::If, location),
            "nil" => Token::new(TokenType::Nil, location),
            "or" => Token::new(TokenType::Or, location),
            "print" => Token::new(TokenType::Print, location),
            "return" => Token::new(TokenType::Return, location),
            "super" => Token::new(TokenType::Super, location),
            "this" => Token::new(TokenType::This, location),
            "true" => Token::new(TokenType::True, location),
            "var" => Token::new(TokenType::Var, location),
            "while" => Token::new(TokenType::While, location),
            _ => Token::new(TokenType::Identifier(String::from(identifier)), location),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    Str(String), // This is "String" in the book, but that's reserved for Rust
    Number(f64),
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Special
    Illegal(char),   // This one isn't in the book, it's just useful
    Comment(String), // This one isn't in the book either, it's discussed at length in the README
    _EOF, // Provided by the book but since this program parses line by line it's not used
}

// Just to make the tokens print the same as in the book, more or less
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Normalize names with the book when printing
        write!(
            f,
            "{}",
            match self {
                TokenType::LeftParen => String::from("LEFT_PAREN"),
                TokenType::RightParen => String::from("RIGHT_PAREN"),
                TokenType::LeftBrace => String::from("LEFT_BRACE"),
                TokenType::RightBrace => String::from("RIGHT_BRACE"),
                TokenType::Comma => String::from("COMMA"),
                TokenType::Dot => String::from("DOT"),
                TokenType::Minus => String::from("MINUS"),
                TokenType::Plus => String::from("PLUS"),
                TokenType::Semicolon => String::from("SEMICOLON"),
                TokenType::Slash => String::from("SLASH"),
                TokenType::Star => String::from("STAR"),
                TokenType::Bang => String::from("BANG"),
                TokenType::BangEqual => String::from("BANG_EQUAL"),
                TokenType::Equal => String::from("EQUAL"),
                TokenType::EqualEqual => String::from("EQUAL_EQUAL"),
                TokenType::Greater => String::from("GREATER"),
                TokenType::GreaterEqual => String::from("GREATER_EQUAL"),
                TokenType::Less => String::from("LESS"),
                TokenType::LessEqual => String::from("LESS_EQUAL"),
                TokenType::Identifier(s) => format!("IDENTIFIER({})", s),
                TokenType::Str(s) => format!("STRING({})", s),
                TokenType::Number(n) => format!("NUMBER({})", n),
                TokenType::And => String::from("AND"),
                TokenType::Class => String::from("CLASS"),
                TokenType::Else => String::from("ELSE"),
                TokenType::False => String::from("FALSE"),
                TokenType::Fun => String::from("FUN"),
                TokenType::For => String::from("FOR"),
                TokenType::If => String::from("IF"),
                TokenType::Nil => String::from("NIL"),
                TokenType::Or => String::from("OR"),
                TokenType::Print => String::from("PRINT"),
                TokenType::Return => String::from("RETURN"),
                TokenType::Super => String::from("SUPER"),
                TokenType::This => String::from("THIS"),
                TokenType::True => String::from("TRUE"),
                TokenType::Var => String::from("VAR"),
                TokenType::While => String::from("WHILE"),
                TokenType::Comment(s) => format!("COMMENT({})", s),
                TokenType::Illegal(c) => format!("ILLEGAL({})", c),
                TokenType::_EOF => String::from("EOF"),
            }
        )
    }
}
