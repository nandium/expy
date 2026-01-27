use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// Edge cases and whitespace handling
// ============================================================================

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}

#[test]
fn test_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n  ");
    let tokens = lexer.tokenize();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}
