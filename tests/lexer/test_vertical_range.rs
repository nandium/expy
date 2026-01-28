use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: VERTICAL-RANGE - $?[A-Z]+:$?[A-Z]+
// Priority: 0
// ============================================================================

#[test]
fn test_vertical_range_simple() {
    let mut lexer = Lexer::new("A:Z");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "A:Z"));
}

#[test]
fn test_vertical_range_same_column() {
    let mut lexer = Lexer::new("A:A");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "A:A"));
}

#[test]
fn test_vertical_range_multi_letter() {
    let mut lexer = Lexer::new("AA:ZZ");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "AA:ZZ"));
}

#[test]
fn test_vertical_range_absolute_first() {
    let mut lexer = Lexer::new("$A:Z");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "$A:Z"));
}

#[test]
fn test_vertical_range_absolute_second() {
    let mut lexer = Lexer::new("A:$Z");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "A:$Z"));
}

#[test]
fn test_vertical_range_absolute_both() {
    let mut lexer = Lexer::new("$A:$Z");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "$A:$Z"));
}

#[test]
fn test_vertical_range_in_expression() {
    let mut lexer = Lexer::new("A:C + D:F");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "A:C"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::VerticalRange(s) if s == "D:F"));
}
