use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: ERROR - #NULL! | #DIV/0! | #VALUE! | #NAME? | #NUM! | #N/A
// ============================================================================

#[test]
fn test_error_null() {
    let mut lexer = Lexer::new("#NULL!");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#NULL!"));
}

#[test]
fn test_error_div_zero() {
    let mut lexer = Lexer::new("#DIV/0!");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#DIV/0!"));
}

#[test]
fn test_error_value() {
    let mut lexer = Lexer::new("#VALUE!");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#VALUE!"));
}

#[test]
fn test_error_name() {
    let mut lexer = Lexer::new("#NAME?");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#NAME?"));
}

#[test]
fn test_error_num() {
    let mut lexer = Lexer::new("#NUM!");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#NUM!"));
}

#[test]
fn test_error_na() {
    let mut lexer = Lexer::new("#N/A");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#N/A"));
}

#[test]
fn test_error_all_types() {
    let mut lexer = Lexer::new("#NULL! #DIV/0! #VALUE! #NAME? #NUM! #N/A");
    let tokens = lexer.tokenize();
    assert!(matches!(&tokens[0], Token::Error(e) if e == "#NULL!"));
    assert!(matches!(&tokens[1], Token::Error(e) if e == "#DIV/0!"));
    assert!(matches!(&tokens[2], Token::Error(e) if e == "#VALUE!"));
    assert!(matches!(&tokens[3], Token::Error(e) if e == "#NAME?"));
    assert!(matches!(&tokens[4], Token::Error(e) if e == "#NUM!"));
    assert!(matches!(&tokens[5], Token::Error(e) if e == "#N/A"));
}

// ============================================================================
// SPEC: ERROR-REF - #REF!
// ============================================================================

#[test]
fn test_error_ref() {
    let mut lexer = Lexer::new("#REF!");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::ErrorRef));
}

#[test]
fn test_error_ref_distinct_from_other_errors() {
    let mut lexer = Lexer::new("#REF! #VALUE!");
    let tokens = lexer.tokenize();
    assert!(matches!(tokens[0], Token::ErrorRef));
    assert!(matches!(&tokens[1], Token::Error(e) if e == "#VALUE!"));
}
