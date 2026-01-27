/// Token types for Excel formulas
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Bool(bool),
    Error(String),
    ErrorRef, // #REF! - separate from other errors per spec

    // Operators - Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,

    // Operators - Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Operators - String
    Concatenate,

    // Delimiters
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // End of input
    Eof,
}
