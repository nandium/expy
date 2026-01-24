use expy::bindings::lexer::Lexer;
use expy::bindings::token::TokenKind;

// ============================================================================
// SPEC: BOOL - Boolean literal TRUE | FALSE
// ============================================================================

#[test]
fn test_bool_true_uppercase() {
    let mut lexer = Lexer::new("TRUE");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Bool(true)));
}

#[test]
fn test_bool_false_uppercase() {
    let mut lexer = Lexer::new("FALSE");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Bool(false)));
}

#[test]
fn test_bool_true_lowercase() {
    let mut lexer = Lexer::new("true");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Bool(true)));
}

#[test]
fn test_bool_false_lowercase() {
    let mut lexer = Lexer::new("false");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Bool(false)));
}

#[test]
fn test_bool_mixed_case() {
    let mut lexer = Lexer::new("True False TrUe FaLsE");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Bool(true)));
    assert!(matches!(tokens[1].kind, TokenKind::Bool(false)));
    assert!(matches!(tokens[2].kind, TokenKind::Bool(true)));
    assert!(matches!(tokens[3].kind, TokenKind::Bool(false)));
}
