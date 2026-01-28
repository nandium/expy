use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: HORIZONTAL-RANGE - $?[0-9]+:$?[0-9]+
// Priority: 0
// ============================================================================

#[test]
fn test_horizontal_range_simple() {
    let mut lexer = Lexer::new("1:10");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "1:10"));
}

#[test]
fn test_horizontal_range_same_row() {
    let mut lexer = Lexer::new("5:5");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "5:5"));
}

#[test]
fn test_horizontal_range_large_numbers() {
    let mut lexer = Lexer::new("100:1000");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "100:1000"));
}

#[test]
fn test_horizontal_range_absolute_first() {
    let mut lexer = Lexer::new("$1:10");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "$1:10"));
}

#[test]
fn test_horizontal_range_absolute_second() {
    let mut lexer = Lexer::new("1:$10");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "1:$10"));
}

#[test]
fn test_horizontal_range_absolute_both() {
    let mut lexer = Lexer::new("$1:$10");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "$1:$10"));
}

#[test]
fn test_horizontal_range_in_expression() {
    let mut lexer = Lexer::new("1:5 + 10:15");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "1:5"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::HorizontalRange(s) if s == "10:15"));
}

#[test]
fn test_horizontal_range_invalid_only_dollar_after_colon() {
    // 1:$ should error ($ alone is not valid)
    let mut lexer = Lexer::new("1:$");
    let result = lexer.tokenize();
    assert!(result.is_err(), "1:$ should error but got: {:?}", result);
}

#[test]
fn test_horizontal_range_invalid_letters_after_colon() {
    // 1:A should error (A is not a horizontal range)
    let mut lexer = Lexer::new("1:A");
    let result = lexer.tokenize();
    assert!(result.is_err(), "1:A should error but got: {:?}", result);
}
