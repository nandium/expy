use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// Complex Cases
// ============================================================================


#[test]
fn test_cell_followed_by_operators() {
    let mut lexer = Lexer::new("A1+B2*C3");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[3], Token::Multiply));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "C3"));
}

#[test]
fn test_position_restored_on_failed_horizontal_range_parse() {
    // Input "123+456" should parse as Number(123), Plus, Number(456)
    // NOT try to parse "123+" as a horizontal range and fail
    // This tests that try_read_horizontal_range properly restores position on failure
    let mut lexer = Lexer::new("123+456");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Number(n) if n == 123.0));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 456.0));
}

#[test]
fn test_position_restored_on_failed_cell_parse() {
    // Input "123" should parse as Number(123), not try to parse as cell and fail
    // This tests position restoration in try_read_cell_or_vertical_range
    let mut lexer = Lexer::new("123");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Number(n) if n == 123.0));
    assert!(matches!(tokens[1], Token::Eof));
}

#[test]
fn test_complex_formula_with_cells_and_ranges() {
    // SUM(A1:B10) + C5 * D6
    let mut lexer = Lexer::new("A1:B10+C5*D6");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Colon));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "B10"));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "C5"));
    assert!(matches!(tokens[5], Token::Multiply));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "D6"));
}

#[test]
fn test_cells_with_absolute_references_in_expression() {
    let mut lexer = Lexer::new("$A$1 + $B2 - C$3 * D4");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "$A$1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "$B2"));
    assert!(matches!(tokens[3], Token::Minus));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "C$3"));
    assert!(matches!(tokens[5], Token::Multiply));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "D4"));
}

#[test]
fn test_array_with_cell_references() {
    let mut lexer = Lexer::new("{A1,B2,C3;D4,E5,F6}");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(&tokens[1], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(&tokens[3], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(&tokens[5], Token::Cell(s) if s == "C3"));
    assert!(matches!(tokens[6], Token::Semicolon));
    assert!(matches!(&tokens[7], Token::Cell(s) if s == "D4"));
    assert!(matches!(tokens[8], Token::Comma));
    assert!(matches!(&tokens[9], Token::Cell(s) if s == "E5"));
    assert!(matches!(tokens[10], Token::Comma));
    assert!(matches!(&tokens[11], Token::Cell(s) if s == "F6"));
    assert!(matches!(tokens[12], Token::RightBrace));
}

#[test]
fn test_mixed_ranges_and_cells() {
    let mut lexer = Lexer::new("A:C,1:5,A1:B2");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "A:C"));
    assert!(matches!(tokens[1], Token::Comma));
    assert!(matches!(&tokens[2], Token::HorizontalRange(s) if s == "1:5"));
    assert!(matches!(tokens[3], Token::Comma));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[5], Token::Colon));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "B2"));
}

#[test]
fn test_comparison_with_cells() {
    let mut lexer = Lexer::new("A1>B2");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Greater));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "B2"));
}

#[test]
fn test_comparison_operators_with_cells() {
    let mut lexer = Lexer::new("A1<>B2<=C3>=D4=E5<F6");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::NotEqual));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[3], Token::LessEqual));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "C3"));
    assert!(matches!(tokens[5], Token::GreaterEqual));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "D4"));
    assert!(matches!(tokens[7], Token::Equal));
    assert!(matches!(&tokens[8], Token::Cell(s) if s == "E5"));
    assert!(matches!(tokens[9], Token::Less));
    assert!(matches!(&tokens[10], Token::Cell(s) if s == "F6"));
}

#[test]
fn test_string_concatenation_with_cells() {
    let mut lexer = Lexer::new(r#"A1&"text"&B2"#);
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Concatenate));
    assert!(matches!(&tokens[2], Token::String(s) if s == "text"));
    assert!(matches!(tokens[3], Token::Concatenate));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "B2"));
}

#[test]
fn test_cells_with_numbers_and_bools() {
    let mut lexer = Lexer::new("A1+123-TRUE*FALSE/B2");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 123.0));
    assert!(matches!(tokens[3], Token::Minus));
    assert!(matches!(tokens[4], Token::Bool(true)));
    assert!(matches!(tokens[5], Token::Multiply));
    assert!(matches!(tokens[6], Token::Bool(false)));
    assert!(matches!(tokens[7], Token::Divide));
    assert!(matches!(&tokens[8], Token::Cell(s) if s == "B2"));
}

#[test]
fn test_power_operator_with_cells() {
    let mut lexer = Lexer::new("A1^2+B2^C3");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Power));
    assert!(matches!(tokens[2], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[5], Token::Power));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "C3"));
}

#[test]
fn test_nested_arrays_with_ranges() {
    let mut lexer = Lexer::new("{A1:B2,C3:D4}");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::LeftBrace));
    assert!(matches!(&tokens[1], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[2], Token::Colon));
    assert!(matches!(&tokens[3], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(&tokens[5], Token::Cell(s) if s == "C3"));
    assert!(matches!(tokens[6], Token::Colon));
    assert!(matches!(&tokens[7], Token::Cell(s) if s == "D4"));
    assert!(matches!(tokens[8], Token::RightBrace));
}

#[test]
fn test_all_absolute_reference_combinations() {
    let mut lexer = Lexer::new("A1+$A1+A$1+$A$1");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "$A1"));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "A$1"));
    assert!(matches!(tokens[5], Token::Plus));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "$A$1"));
}

#[test]
fn test_large_cell_references() {
    let mut lexer = Lexer::new("XFD1048576+AAA999999");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "XFD1048576"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "AAA999999"));
}

#[test]
fn test_vertical_range_with_absolute_variants() {
    let mut lexer = Lexer::new("A:B+$C:D+E:$F+$G:$H");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::VerticalRange(s) if s == "A:B"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::VerticalRange(s) if s == "$C:D"));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(&tokens[4], Token::VerticalRange(s) if s == "E:$F"));
    assert!(matches!(tokens[5], Token::Plus));
    assert!(matches!(&tokens[6], Token::VerticalRange(s) if s == "$G:$H"));
}

#[test]
fn test_horizontal_range_with_absolute_variants() {
    let mut lexer = Lexer::new("1:10+$20:30+40:$50+$60:$70");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::HorizontalRange(s) if s == "1:10"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::HorizontalRange(s) if s == "$20:30"));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(&tokens[4], Token::HorizontalRange(s) if s == "40:$50"));
    assert!(matches!(tokens[5], Token::Plus));
    assert!(matches!(&tokens[6], Token::HorizontalRange(s) if s == "$60:$70"));
}

#[test]
fn test_errors_with_cells() {
    let mut lexer = Lexer::new("A1+#DIV/0!-B2*#REF!");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Error(e) if e == "#DIV/0!"));
    assert!(matches!(tokens[3], Token::Minus));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[5], Token::Multiply));
    assert!(matches!(tokens[6], Token::ErrorRef));
}

#[test]
fn test_complex_nested_expression() {
    // (A1+B2)*(C3-D4)/(E5^F6)
    let mut lexer = Lexer::new("A1+B2*C3-D4/E5^F6");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "A1"));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "B2"));
    assert!(matches!(tokens[3], Token::Multiply));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "C3"));
    assert!(matches!(tokens[5], Token::Minus));
    assert!(matches!(&tokens[6], Token::Cell(s) if s == "D4"));
    assert!(matches!(tokens[7], Token::Divide));
    assert!(matches!(&tokens[8], Token::Cell(s) if s == "E5"));
    assert!(matches!(tokens[9], Token::Power));
    assert!(matches!(&tokens[10], Token::Cell(s) if s == "F6"));
}

#[test]
fn test_whitespace_with_complex_references() {
    let mut lexer = Lexer::new("  $A$1  :  $B$10  +  C5  ");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::Cell(s) if s == "$A$1"));
    assert!(matches!(tokens[1], Token::Colon));
    assert!(matches!(&tokens[2], Token::Cell(s) if s == "$B$10"));
    assert!(matches!(tokens[3], Token::Plus));
    assert!(matches!(&tokens[4], Token::Cell(s) if s == "C5"));
}
