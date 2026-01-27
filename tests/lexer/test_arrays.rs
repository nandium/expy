use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: Array delimiters - { } , ;
// ============================================================================

#[test]
fn test_array_left_brace() {
    let mut lexer = Lexer::new("{");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
}

#[test]
fn test_array_right_brace() {
    let mut lexer = Lexer::new("}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::RightBrace));
}

#[test]
fn test_array_comma() {
    let mut lexer = Lexer::new(",");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Comma));
}

#[test]
fn test_array_semicolon() {
    let mut lexer = Lexer::new(";");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::Semicolon));
}

#[test]
fn test_array_all_delimiters() {
    let mut lexer = Lexer::new("{ } , ;");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::RightBrace));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Semicolon));
}

// ============================================================================
// SPEC: hArrayConsti - hConstanti | hUnOpPrefixi NUMBER | ERROR-REF
// Testing array constant compositions
// ============================================================================

#[test]
fn test_array_const_numbers() {
    let mut lexer = Lexer::new("{123,45.6,7e2}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 123.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Number(n) if (n - 45.6).abs() < 0.01));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Number(n) if n == 7e2));
    assert!(matches!(tokens[6], Token::RightBrace));
}

#[test]
fn test_array_const_strings() {
    let mut lexer = Lexer::new(r#"{"a","b","c"}"#);
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(&tokens[1], Token::String(s) if s == "a"));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(&tokens[3], Token::String(s) if s == "b"));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(&tokens[5], Token::String(s) if s == "c"));
    assert!(matches!(tokens[6], Token::RightBrace));
}

#[test]
fn test_array_const_booleans() {
    let mut lexer = Lexer::new("{TRUE,FALSE,TRUE}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Bool(true)));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Bool(false)));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Bool(true)));
    assert!(matches!(tokens[6], Token::RightBrace));
}

#[test]
fn test_array_const_errors() {
    let mut lexer = Lexer::new("{#DIV/0!,#VALUE!,#N/A}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(&tokens[1], Token::Error(e) if e == "#DIV/0!"));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(&tokens[3], Token::Error(e) if e == "#VALUE!"));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(&tokens[5], Token::Error(e) if e == "#N/A"));
    assert!(matches!(tokens[6], Token::RightBrace));
}

#[test]
fn test_array_const_unary_prefix_positive() {
    let mut lexer = Lexer::new("{+1,+2.5,+3e2}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[3], Token::Comma));
    assert!(matches!(tokens[4], Token::Plus));
    assert!(matches!(tokens[5], Token::Number(n) if n == 2.5));
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(tokens[7], Token::Plus));
    assert!(matches!(tokens[8], Token::Number(n) if n == 3e2));
    assert!(matches!(tokens[9], Token::RightBrace));
}

#[test]
fn test_array_const_unary_prefix_negative() {
    let mut lexer = Lexer::new("{-1,-2.5,-3e2}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Minus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[3], Token::Comma));
    assert!(matches!(tokens[4], Token::Minus));
    assert!(matches!(tokens[5], Token::Number(n) if n == 2.5));
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(tokens[7], Token::Minus));
    assert!(matches!(tokens[8], Token::Number(n) if n == 3e2));
    assert!(matches!(tokens[9], Token::RightBrace));
}

#[test]
fn test_array_const_with_error_ref() {
    let mut lexer = Lexer::new("{1,#REF!,3}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::ErrorRef));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[6], Token::RightBrace));
}

#[test]
fn test_array_const_mixed_types() {
    let mut lexer = Lexer::new(r#"{1,"test",TRUE,#DIV/0!,+5,-3.2,#REF!}"#);
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(&tokens[3], Token::String(s) if s == "test"));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Bool(true)));
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(&tokens[7], Token::Error(e) if e == "#DIV/0!"));
    assert!(matches!(tokens[8], Token::Comma));
    assert!(matches!(tokens[9], Token::Plus));
    assert!(matches!(tokens[10], Token::Number(n) if n == 5.0));
    assert!(matches!(tokens[11], Token::Comma));
    assert!(matches!(tokens[12], Token::Minus));
    assert!(matches!(tokens[13], Token::Number(n) if (n - 3.2).abs() < 0.01));
    assert!(matches!(tokens[14], Token::Comma));
    assert!(matches!(tokens[15], Token::ErrorRef));
    assert!(matches!(tokens[16], Token::RightBrace));
}

// ============================================================================
// SPEC: hArrayRowsi - hArrayConsti { ',' hArrayConsti }
// Testing array rows (comma-separated values)
// ============================================================================

#[test]
fn test_array_single_row() {
    let mut lexer = Lexer::new("{1,2,3}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[6], Token::RightBrace));
}

// ============================================================================
// SPEC: hArrayColumnsi - hArrayRowsi { ';' hArrayRowsi }
// Testing array columns (semicolon-separated rows)
// ============================================================================

#[test]
fn test_array_two_rows() {
    let mut lexer = Lexer::new("{1,2;3,4}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[4], Token::Semicolon));
    assert!(matches!(tokens[5], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(tokens[7], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[8], Token::RightBrace));
}

#[test]
fn test_array_three_rows() {
    let mut lexer = Lexer::new("{1,2;3,4;5,6}");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[4], Token::Semicolon));
    assert!(matches!(tokens[5], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(tokens[7], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[8], Token::Semicolon));
    assert!(matches!(tokens[9], Token::Number(n) if n == 5.0));
    assert!(matches!(tokens[10], Token::Comma));
    assert!(matches!(tokens[11], Token::Number(n) if n == 6.0));
    assert!(matches!(tokens[12], Token::RightBrace));
}

#[test]
fn test_array_matrix_3x3() {
    let mut lexer = Lexer::new("{1,2,3;4,5,6;7,8,9}");
    let tokens = lexer.tokenize();

    assert!(matches!(tokens[0], Token::LeftBrace));
    // Row 1
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[6], Token::Semicolon));
    // Row 2
    assert!(matches!(tokens[7], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[8], Token::Comma));
    assert!(matches!(tokens[9], Token::Number(n) if n == 5.0));
    assert!(matches!(tokens[10], Token::Comma));
    assert!(matches!(tokens[11], Token::Number(n) if n == 6.0));
    assert!(matches!(tokens[12], Token::Semicolon));
    // Row 3
    assert!(matches!(tokens[13], Token::Number(n) if n == 7.0));
    assert!(matches!(tokens[14], Token::Comma));
    assert!(matches!(tokens[15], Token::Number(n) if n == 8.0));
    assert!(matches!(tokens[16], Token::Comma));
    assert!(matches!(tokens[17], Token::Number(n) if n == 9.0));
    assert!(matches!(tokens[18], Token::RightBrace));
}

#[test]
fn test_array_with_whitespace() {
    let mut lexer = Lexer::new("{ 1 , 2 ; 3 , 4 }");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(tokens[1], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[4], Token::Semicolon));
    assert!(matches!(tokens[5], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(tokens[7], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[8], Token::RightBrace));
}
