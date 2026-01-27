use super::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let start = self.position;

        // Read integer part: [0-9]+
        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Read decimal part: .? [0-9]*
        if self.current() == Some('.') {
            self.advance();
            while let Some(c) = self.current() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Read scientific notation part: (e [0-9]+)?
        if self.current() == Some('e') || self.current() == Some('E') {
            self.advance();
            while let Some(c) = self.current() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_str: String = self.input[start..self.position].iter().collect();
        num_str.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> String {
        self.advance(); // skip opening "
        let mut result = String::new();

        while let Some(c) = self.current() {
            if c == '"' {
                // Check for escaped quote ""
                match self.peek(1) {
                    Some('"') => {
                        result.push('"');
                        self.advance(); // skip first "
                        self.advance(); // skip second "
                    }
                    _ => {
                        self.advance(); // skip closing "
                        break;
                    }
                }
            } else {
                result.push(c);
                self.advance();
            }
        }

        result
    }

    fn read_error(&mut self) -> String {
        let start = self.position;

        // Read the entire error token
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '/' || c == '!' || c == '?' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current() {
            None => Token::Eof,
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }
            Some('}') => {
                self.advance();
                Token::RightBrace
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some('"') => {
                let s = self.read_string();
                Token::String(s)
            }
            Some('#') => {
                self.advance(); // Consume the '#' to prevent infinite loop
                let err = self.read_error();
                // Distinguish between ERROR-REF and ERROR
                if err == "REF!" {
                    Token::ErrorRef
                } else {
                    Token::Error(format!("#{}", err))
                }
            }
            Some(c) if c.is_ascii_digit() => {
                let num = self.read_number();
                Token::Number(num)
            }
            Some(c) if c.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.to_uppercase().as_str() {
                    "TRUE" => Token::Bool(true),
                    "FALSE" => Token::Bool(false),
                    _ => Token::Eof, // Unknown for now
                }
            }
            Some(_) => {
                self.advance();
                Token::Eof
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}
