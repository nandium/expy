/// Token types for Excel formulas
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64),
    String(String),
    Bool(bool),
    Error(String),
    ErrorRef, // #REF! - separate from other errors per spec

    // Operators
    Plus,
    Minus,

    // Delimiters
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // End of input
    Eof,
}

/// A token with its kind
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}
