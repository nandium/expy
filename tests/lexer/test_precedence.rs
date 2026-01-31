use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// Precedence Tests - Verify lookahead-based priority matching
// Tests that longer/more specific patterns take precedence over shorter ones
//
// Priority values from paper (Table I):
//   CELL:             2
//   VERTICAL-RANGE:   0
//   HORIZONTAL-RANGE: 0
//   NUMBER:           0
//   BOOL:             0
//   STRING:           0
//
// Note: When priorities are equal (all 0), longest match wins via lookahead
// ============================================================================

#[test]
fn test_precedence_horizontal_range_over_number() {
    // Priority: HORIZONTAL-RANGE (0) vs NUMBER (0) - longest match wins
    // "1:10" should be parsed as HorizontalRange, not Number + Colon + Number
    let mut lexer = Lexer::new("1:10");
    let tokens = lexer.tokenize().unwrap();
    assert!(
        matches!(&tokens[0], Token::HorizontalRange(s) if s == "1:10"),
        "Expected HorizontalRange but got: {:?}",
        tokens[0]
    );
    assert!(matches!(tokens[1], Token::Eof));
}

#[test]
fn test_precedence_cell_over_identifier() {
    // Priority: CELL (2) > BOOL/identifier (0)
    // "A1" should be parsed as Cell, not identifier
    let mut lexer = Lexer::new("A1");
    let tokens = lexer.tokenize().unwrap();
    assert!(
        matches!(&tokens[0], Token::Cell(s) if s == "A1"),
        "Expected Cell but got: {:?}",
        tokens[0]
    );
}

#[test]
fn test_precedence_vertical_range_over_cell() {
    // Priority: VERTICAL-RANGE (0) vs CELL (2) - but CELL pattern doesn't match (no row)
    // "A:Z" should be parsed as VerticalRange, not Cell(A) + ... (which would fail)
    let mut lexer = Lexer::new("A:Z");
    let tokens = lexer.tokenize().unwrap();
    assert!(
        matches!(&tokens[0], Token::VerticalRange(s) if s == "A:Z"),
        "Expected VerticalRange but got: {:?}",
        tokens[0]
    );
}

#[test]
fn test_precedence_lookahead_with_dollar_sign() {
    // Priority: All with $ prefix - HORIZONTAL-RANGE(0), CELL(2), VERTICAL-RANGE(0)
    // "$1:$10" is HorizontalRange (longer match with $ prefix)
    let mut lexer = Lexer::new("$1:$10");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "$1:$10"));

    // "$A$1" is Cell (not vertical range - no colon)
    let mut lexer = Lexer::new("$A$1");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "$A$1"));

    // "$A:$Z" is VerticalRange
    let mut lexer = Lexer::new("$A:$Z");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "$A:$Z"));
}

#[test]
fn test_precedence_lookahead_prevents_incorrect_parse() {
    // Priority: HORIZONTAL-RANGE(0) attempted but fails → NUMBER(0) succeeds
    // "123+456" - lookahead sees "123" has no ":" so it's a number, not horizontal range
    let mut lexer = Lexer::new("123+456");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Number(n) if n == 123.0));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 456.0));
}

#[test]
fn test_precedence_multichar_column_range() {
    // Priority: VERTICAL-RANGE(0) - multi-character columns work
    // "AA:ZZ" is VerticalRange, not Cell
    let mut lexer = Lexer::new("AA:ZZ");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "AA:ZZ"));
}

#[test]
fn test_precedence_large_number_range() {
    // Priority: HORIZONTAL-RANGE(0) vs NUMBER(0) - longest match wins
    // "100:1000" is HorizontalRange, not Number
    let mut lexer = Lexer::new("100:1000");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "100:1000"));
}
