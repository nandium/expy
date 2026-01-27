use super::token::{LexerError, Token};

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

    fn read_number(&mut self) -> Result<f64, LexerError> {
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
        num_str
            .parse()
            .map_err(|_| LexerError::InvalidNumber(num_str))
    }

    fn read_string(&mut self) -> Result<String, LexerError> {
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
                        return Ok(result);
                    }
                }
            } else {
                result.push(c);
                self.advance();
            }
        }

        // If we reach here, the string was not terminated
        Err(LexerError::UnterminatedString)
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

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        match self.current() {
            None => Ok(Token::Eof),
            Some('+') => {
                self.advance();
                Ok(Token::Plus)
            }
            Some('-') => {
                self.advance();
                Ok(Token::Minus)
            }
            Some('*') => {
                self.advance();
                Ok(Token::Multiply)
            }
            Some('/') => {
                self.advance();
                Ok(Token::Divide)
            }
            Some('^') => {
                self.advance();
                Ok(Token::Power)
            }
            Some('&') => {
                self.advance();
                Ok(Token::Concatenate)
            }
            Some('=') => {
                self.advance();
                Ok(Token::Equal)
            }
            Some('<') => {
                self.advance();
                match self.current() {
                    Some('=') => {
                        self.advance();
                        Ok(Token::LessEqual)
                    }
                    Some('>') => {
                        self.advance();
                        Ok(Token::NotEqual)
                    }
                    _ => Ok(Token::Less),
                }
            }
            Some('>') => {
                self.advance();
                match self.current() {
                    Some('=') => {
                        self.advance();
                        Ok(Token::GreaterEqual)
                    }
                    _ => Ok(Token::Greater),
                }
            }
            Some('{') => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            Some('}') => {
                self.advance();
                Ok(Token::RightBrace)
            }
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            }
            Some(';') => {
                self.advance();
                Ok(Token::Semicolon)
            }
            Some('"') => {
                let s = self.read_string()?;
                Ok(Token::String(s))
            }
            Some('#') => {
                self.advance(); // Consume the '#' to prevent infinite loop
                let err = self.read_error();
                // Distinguish between ERROR-REF and ERROR
                if err == "REF!" {
                    Ok(Token::ErrorRef)
                } else {
                    Ok(Token::Error(format!("#{}", err)))
                }
            }
            Some(c) if c.is_ascii_digit() => {
                let num = self.read_number()?;
                Ok(Token::Number(num))
            }
            Some(c) if c.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.to_uppercase().as_str() {
                    "TRUE" => Ok(Token::Bool(true)),
                    "FALSE" => Ok(Token::Bool(false)),
                    _ => Err(LexerError::UnexpectedChar(c)),
                }
            }
            Some(c) => Err(LexerError::UnexpectedChar(c)),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}
