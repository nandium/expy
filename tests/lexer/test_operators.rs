use expy::bindings::lexer::Lexer;
use expy::bindings::token::Token;

// ============================================================================
// SPEC: hUnOpPrefixi - '+' | '-'
// ============================================================================

#[test]
fn test_unary_plus() {
    let mut lexer = Lexer::new("+");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Plus));
}

#[test]
fn test_unary_plus_with_number() {
    let mut lexer = Lexer::new("+4.0");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Plus));
    assert!(matches!(tokens[1], Token::Number(4.0)));
}

#[test]
fn test_unary_minus() {
    let mut lexer = Lexer::new("-");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Minus));
}

#[test]
fn test_unary_minus_with_number() {
    let mut lexer = Lexer::new("-4.0");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Minus));
    assert!(matches!(tokens[1], Token::Number(4.0)));
}

#[test]
fn test_unary_operators_both() {
    let mut lexer = Lexer::new("+ -");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Plus));
    assert!(matches!(tokens[1], Token::Minus));
}

// ============================================================================
// SPEC: hBinOpi - '+' | '-' | '*' | '/' | '^' | '&'
//                | '<' | '>' | '=' | '<=' | '>=' | '<>'
// ============================================================================

// Arithmetic Operators
#[test]
fn test_binary_multiply() {
    let mut lexer = Lexer::new("*");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Multiply));
}

#[test]
fn test_binary_divide() {
    let mut lexer = Lexer::new("/");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Divide));
}

#[test]
fn test_binary_power() {
    let mut lexer = Lexer::new("^");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Power));
}

#[test]
fn test_binary_arithmetic_expression() {
    let mut lexer = Lexer::new("1 + 2 * 3 / 4 - 5 ^ 6");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[3], Token::Multiply));
    assert!(matches!(tokens[4], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[5], Token::Divide));
    assert!(matches!(tokens[6], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[7], Token::Minus));
    assert!(matches!(tokens[8], Token::Number(n) if n == 5.0));
    assert!(matches!(tokens[9], Token::Power));
    assert!(matches!(tokens[10], Token::Number(n) if n == 6.0));
}

// String Concatenation Operator
#[test]
fn test_binary_concatenate() {
    let mut lexer = Lexer::new("&");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Concatenate));
}

#[test]
fn test_binary_concatenate_strings() {
    let mut lexer = Lexer::new(r#""hello" & "world""#);
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::String(s) if s == "hello"));
    assert!(matches!(tokens[1], Token::Concatenate));
    assert!(matches!(&tokens[2], Token::String(s) if s == "world"));
}

// Comparison Operators - Single Character
#[test]
fn test_binary_equal() {
    let mut lexer = Lexer::new("=");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Equal));
}

#[test]
fn test_binary_less() {
    let mut lexer = Lexer::new("<");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Less));
}

#[test]
fn test_binary_greater() {
    let mut lexer = Lexer::new(">");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Greater));
}

// Comparison Operators - Two Character
#[test]
fn test_binary_less_equal() {
    let mut lexer = Lexer::new("<=");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::LessEqual));
}

#[test]
fn test_binary_greater_equal() {
    let mut lexer = Lexer::new(">=");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::GreaterEqual));
}

#[test]
fn test_binary_not_equal() {
    let mut lexer = Lexer::new("<>");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::NotEqual));
}

#[test]
fn test_binary_comparison_expression() {
    let mut lexer = Lexer::new("1 < 2 <= 3 > 4 >= 5 = 6 <> 7");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[1], Token::Less));
    assert!(matches!(tokens[2], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[3], Token::LessEqual));
    assert!(matches!(tokens[4], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[5], Token::Greater));
    assert!(matches!(tokens[6], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[7], Token::GreaterEqual));
    assert!(matches!(tokens[8], Token::Number(n) if n == 5.0));
    assert!(matches!(tokens[9], Token::Equal));
    assert!(matches!(tokens[10], Token::Number(n) if n == 6.0));
    assert!(matches!(tokens[11], Token::NotEqual));
    assert!(matches!(tokens[12], Token::Number(n) if n == 7.0));
}

// Edge Cases - Operators without spaces
#[test]
fn test_binary_operators_no_spaces() {
    let mut lexer = Lexer::new("1+2*3/4-5^6");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Number(n) if n == 1.0));
    assert!(matches!(tokens[1], Token::Plus));
    assert!(matches!(tokens[2], Token::Number(n) if n == 2.0));
    assert!(matches!(tokens[3], Token::Multiply));
    assert!(matches!(tokens[4], Token::Number(n) if n == 3.0));
    assert!(matches!(tokens[5], Token::Divide));
    assert!(matches!(tokens[6], Token::Number(n) if n == 4.0));
    assert!(matches!(tokens[7], Token::Minus));
    assert!(matches!(tokens[8], Token::Number(n) if n == 5.0));
    assert!(matches!(tokens[9], Token::Power));
    assert!(matches!(tokens[10], Token::Number(n) if n == 6.0));
}

#[test]
fn test_binary_all_operators() {
    let mut lexer = Lexer::new("+ - * / ^ & = < > <= >= <>");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::Plus));
    assert!(matches!(tokens[1], Token::Minus));
    assert!(matches!(tokens[2], Token::Multiply));
    assert!(matches!(tokens[3], Token::Divide));
    assert!(matches!(tokens[4], Token::Power));
    assert!(matches!(tokens[5], Token::Concatenate));
    assert!(matches!(tokens[6], Token::Equal));
    assert!(matches!(tokens[7], Token::Less));
    assert!(matches!(tokens[8], Token::Greater));
    assert!(matches!(tokens[9], Token::LessEqual));
    assert!(matches!(tokens[10], Token::GreaterEqual));
    assert!(matches!(tokens[11], Token::NotEqual));
}
