use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: CELL - $?[A-Z]+$?[1-9][0-9]*
// Priority: 2
// ============================================================================

#[test]
fn test_cell_simple() {
    let mut lexer = Lexer::new("A1");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
}

#[test]
fn test_cell_double_letter() {
    let mut lexer = Lexer::new("AA10");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "AA10"));
}

#[test]
fn test_cell_triple_letter() {
    let mut lexer = Lexer::new("XFD1048576");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "XFD1048576"));
}

#[test]
fn test_cell_absolute_column() {
    let mut lexer = Lexer::new("$A1");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "$A1"));
}

#[test]
fn test_cell_absolute_row() {
    let mut lexer = Lexer::new("A$1");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A$1"));
}

#[test]
fn test_cell_absolute_both() {
    let mut lexer = Lexer::new("$A$1");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "$A$1"));
}

#[test]
fn test_cell_large_row() {
    let mut lexer = Lexer::new("B999");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "B999"));
}

#[test]
fn test_cell_multiple() {
    let mut lexer = Lexer::new("A1 B2 $C$3");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(&tokens[1], Token::Cell(s) if s == "B2"));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "$C$3"));
}

#[test]
fn test_cell_in_expression() {
    let mut lexer = Lexer::new("A1 + B2");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "B2"));
}

#[test]
fn test_cell_row_zero_variations() {
    // Test various forms of invalid row 0
    let test_cases = vec!["A0", "B0", "$A0", "A$0", "$A$0", "AA0", "XFD0"];

    for input in test_cases {
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        assert!(
            result.is_err(),
            "Input '{}' should error (row 0 is invalid) but got: {:?}",
            input,
            result
        );
    }
}
