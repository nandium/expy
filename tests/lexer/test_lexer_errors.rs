use expy::bindings::lexer::Lexer;
use expy::bindings::token::LexerError;

// ============================================================================
// SPEC: Lexer Error Handling
// ============================================================================

#[test]
fn test_unexpected_char_at() {
    let mut lexer = Lexer::new("@");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('@'))));
}

#[test]
fn test_unexpected_char_in_expression() {
    let mut lexer = Lexer::new("1 + @ - 2");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('@'))));
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new(r#""hello"#);
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnterminatedString)));
}

#[test]
fn test_unterminated_string_with_escape() {
    let mut lexer = Lexer::new(r#""hello""world"#);
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnterminatedString)));
}

#[test]
fn test_unknown_identifier() {
    let mut lexer = Lexer::new("foo");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('f'))));
}

#[test]
fn test_unknown_identifier_mixed() {
    let mut lexer = Lexer::new("1 + bar - 2");
    let result = lexer.tokenize();
    assert!(matches!(result, Err(LexerError::UnexpectedChar('b'))));
}
