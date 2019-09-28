/// Tokens

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // identifiers and literals
    Ident(String),
    Int(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

