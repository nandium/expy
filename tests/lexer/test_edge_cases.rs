use expy::bindings::lexer::Lexer;
use expy::bindings::token::{LexerError, Token};

// ============================================================================
// Edge cases and whitespace handling
// ============================================================================

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}

#[test]
fn test_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n  ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}

#[test]
fn test_invalid_column_only() {
    // "A" without a row number should error
    let mut lexer = Lexer::new("A");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('A'))));
}

#[test]
fn test_invalid_number_then_letter() {
    // "1A" should be parsed as number 1, then error on 'A'
    let mut lexer = Lexer::new("1A");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('A'))));
}

#[test]
fn test_invalid_dollar_column_column() {
    // "$A$B" - two columns with $ but no row should error
    let mut lexer = Lexer::new("$A$B");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('$'))));
}
