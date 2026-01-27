use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: hUnOpPrefixi - '+' | '-'
// ============================================================================

#[test]
fn test_unary_plus() {
    let mut lexer = Lexer::new("+");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Plus));
}

#[test]
fn test_unary_plus_with_number() {
    let mut lexer = Lexer::new("+4.0");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Plus));
    assert!(matches!(tokens[1], Token::Number(4.0)));
}

#[test]
fn test_unary_minus() {
    let mut lexer = Lexer::new("-");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Minus));
}

#[test]
fn test_unary_minus_with_number() {
    let mut lexer = Lexer::new("-4.0");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Minus));
    assert!(matches!(tokens[1], Token::Number(4.0)));
}

#[test]
fn test_unary_operators_both() {
    let mut lexer = Lexer::new("+ -");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Plus));
    assert!(matches!(tokens[1], Token::Minus));
}
