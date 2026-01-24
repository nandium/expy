use expy::bindings::lexer::Lexer;
use expy::bindings::token::TokenKind;

// ============================================================================
// SPEC: STRING - " ([^ "] | "")* "
// String starts and ends with ", contains any chars except ", but "" = escaped "
// ============================================================================

#[test]
fn test_string_empty() {
    let mut lexer = Lexer::new(r#""""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == ""));
}

#[test]
fn test_string_simple() {
    let mut lexer = Lexer::new(r#""hello""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello"));
}

#[test]
fn test_string_with_spaces() {
    let mut lexer = Lexer::new(r#""hello world""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello world"));
}

#[test]
fn test_string_with_numbers() {
    let mut lexer = Lexer::new(r#""test123""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "test123"));
}

#[test]
fn test_string_with_escaped_quote() {
    let mut lexer = Lexer::new(r#""hello""world""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello\"world"));
}

#[test]
fn test_string_with_multiple_escaped_quotes() {
    let mut lexer = Lexer::new(r#""say ""hi"" to ""bob""""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "say \"hi\" to \"bob\""));
}

#[test]
fn test_string_only_escaped_quotes() {
    let mut lexer = Lexer::new(r#""""""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "\""));
}

#[test]
fn test_string_multiple() {
    let mut lexer = Lexer::new(r#""first" "second" "third""#);
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "first"));
    assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s == "second"));
    assert!(matches!(&tokens[2].kind, TokenKind::String(s) if s == "third"));
}
