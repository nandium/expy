/// Lexer error types
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidNumber(String),
}

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

    // References
    Cell(String),            // $?[A-Z]+$?[1-9][0-9]* e.g., A1, $B$2, AA100
    VerticalRange(String),   // $?[A-Z]+:$?[A-Z]+ e.g., A:Z, $A:$C
    HorizontalRange(String), // $?[0-9]+:$?[0-9]+ e.g., 1:10, $5:$8

    // Delimiters
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,

    // End of input
    Eof,
}
