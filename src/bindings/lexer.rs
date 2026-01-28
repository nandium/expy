use super::token::{LexerError, Token};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    /// Saves position, runs closure, restores position only if closure returns None
    fn backtrack_if_needed<F, T>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce(&mut Self) -> Option<T>,
    {
        let saved = self.position;
        let result = f(self);
        if result.is_none() {
            self.position = saved;
        }
        result
    }

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

    // Read a column reference: $?[A-Z]+
    fn read_column(&mut self) -> String {
        let start = self.position;

        // Optional $ prefix
        if self.current() == Some('$') {
            self.advance();
        }

        // Read column letters [A-Z]+
        while let Some(c) = self.current() {
            if c.is_ascii_uppercase() {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    // Read a row reference: $?[1-9][0-9]*
    fn read_row(&mut self) -> String {
        let start = self.position;

        // Optional $ prefix
        if self.current() == Some('$') {
            self.advance();
        }

        // Row must start with [1-9]
        if matches!(self.current(), Some('1'..='9')) {
            self.advance();
            // Followed by [0-9]*
            while let Some(c) = self.current() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.input[start..self.position].iter().collect()
    }

    // Try to read a cell reference or vertical range starting with column letters
    fn try_read_cell_or_vertical_range(&mut self) -> Option<Token> {
        self.backtrack_if_needed(|lexer| {
            let col1 = lexer.read_column();

            // Must have at least one letter (not just $)
            let has_letters = col1.chars().any(|c| c.is_ascii_uppercase());
            if !has_letters {
                return None;
            }

            // Check if followed by ':'
            if lexer.current() == Some(':') {
                lexer.advance();
                let col2 = lexer.read_column();

                // Verify col2 has letters
                let has_letters2 = col2.chars().any(|c| c.is_ascii_uppercase());
                if has_letters2 {
                    return Some(Token::VerticalRange(format!("{}:{}", col1, col2)));
                } else {
                    return None;
                }
            }

            // Check if followed by a row number (making it a CELL)
            if let Some('$' | '1'..='9') = lexer.current() {
                let row = lexer.read_row();
                if !row.is_empty() && row.chars().any(|ch| ch.is_ascii_digit()) {
                    return Some(Token::Cell(format!("{}{}", col1, row)));
                }
            }

            None
        })
    }

    // Try to read a horizontal range: $?[0-9]+:$?[0-9]+
    fn try_read_horizontal_range(&mut self) -> Option<String> {
        self.backtrack_if_needed(|lexer| {
            // Read first row number
            let row1_start = lexer.position;
            if lexer.current() == Some('$') {
                lexer.advance();
            }

            // Must start with a digit
            if let Some(c) = lexer.current() {
                if !c.is_ascii_digit() {
                    return None;
                }
            } else {
                return None;
            }

            while let Some(c) = lexer.current() {
                if c.is_ascii_digit() {
                    lexer.advance();
                } else {
                    break;
                }
            }

            let row1: String = lexer.input[row1_start..lexer.position].iter().collect();

            // Check for ':'
            if lexer.current() != Some(':') {
                return None;
            }
            lexer.advance();

            // Read second row number
            let row2_start = lexer.position;
            if lexer.current() == Some('$') {
                lexer.advance();
            }

            if let Some(c) = lexer.current() {
                if !c.is_ascii_digit() {
                    return None;
                }
            } else {
                return None;
            }

            while let Some(c) = lexer.current() {
                if c.is_ascii_digit() {
                    lexer.advance();
                } else {
                    break;
                }
            }

            let row2: String = lexer.input[row2_start..lexer.position].iter().collect();
            Some(format!("{}:{}", row1, row2))
        })
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
            Some(':') => {
                self.advance();
                Ok(Token::Colon)
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
                // Try horizontal range first (e.g., 1:10)
                if let Some(range) = self.try_read_horizontal_range() {
                    Ok(Token::HorizontalRange(range))
                } else {
                    // Otherwise it's a number
                    let num = self.read_number()?;
                    Ok(Token::Number(num))
                }
            }
            Some('$') => {
                // Could be a cell reference like $A$1 or range like $A:$B or $1:$10
                // Try cell/vertical range first
                if let Some(token) = self.try_read_cell_or_vertical_range() {
                    Ok(token)
                } else if let Some(range) = self.try_read_horizontal_range() {
                    Ok(Token::HorizontalRange(range))
                } else {
                    // Invalid $ usage
                    let c = self.current().unwrap_or('$');
                    Err(LexerError::UnexpectedChar(c))
                }
            }
            Some(c) if c.is_ascii_uppercase() => {
                // Try cell/vertical range first (e.g., A1, A:Z)
                if let Some(token) = self.try_read_cell_or_vertical_range() {
                    Ok(token)
                } else {
                    // Try identifier for TRUE/FALSE
                    let ident = self.read_identifier();
                    match ident.to_uppercase().as_str() {
                        "TRUE" => Ok(Token::Bool(true)),
                        "FALSE" => Ok(Token::Bool(false)),
                        _ => Err(LexerError::UnexpectedChar(c)),
                    }
                }
            }
            Some(c) if c.is_alphabetic() => {
                // Lowercase letters - try identifier for true/false
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
