use expy::bindings::lexer::Lexer;
use expy::bindings::token::TokenKind;

// ============================================================================
// SPEC: NUMBER - [0-9]+ .? [0-9]* (e [0-9]+)?
// ============================================================================

#[test]
fn test_number_integer() {
    let mut lexer = Lexer::new("0 1 123 999999");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 0.0));
    assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 1.0));
    assert!(matches!(tokens[2].kind, TokenKind::Number(n) if n == 123.0));
    assert!(matches!(tokens[3].kind, TokenKind::Number(n) if n == 999999.0));
}

#[test]
fn test_number_with_trailing_dot() {
    let mut lexer = Lexer::new("123.");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 123.0));
}

#[test]
fn test_number_with_decimal() {
    let mut lexer = Lexer::new("123.456 0.5 999.001");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if (n - 123.456).abs() < 0.0001));
    assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 0.5));
    assert!(matches!(tokens[2].kind, TokenKind::Number(n) if (n - 999.001).abs() < 0.0001));
}

#[test]
fn test_number_scientific_notation() {
    let mut lexer = Lexer::new("1e5 2e10 999e3");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 1e5));
    assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 2e10));
    assert!(matches!(tokens[2].kind, TokenKind::Number(n) if n == 999e3));
}

#[test]
fn test_number_scientific_with_decimal() {
    let mut lexer = Lexer::new("1.5e5 2.75e10 0.5e3");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 1.5e5));
    assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 2.75e10));
    assert!(matches!(tokens[2].kind, TokenKind::Number(n) if n == 0.5e3));
}

#[test]
fn test_number_scientific_uppercase_e() {
    let mut lexer = Lexer::new("1E5 2.5E10");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 1e5));
    assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 2.5e10));
}

#[test]
fn test_number_trailing_dot_with_scientific() {
    let mut lexer = Lexer::new("123.e5");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 123e5));
}
