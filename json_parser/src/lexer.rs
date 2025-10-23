use crate::errors::LexError;
use crate::consts::*;

pub struct Lexer {
    raw: Vec<char>,
    current_position: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftCurly,
    RightCurly,
    LeftBracket,
    RigthBracket,
    Colon,
    Literal(String),
    Number(f64),
}

impl Lexer {
    pub fn new(json_raw: &str) -> Self {
        Self {
            raw: json_raw.chars().collect(),
            current_position: 0,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.raw.get(self.current_position)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(&ch) = self.peek() {
            self.current_position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn scan_string(&mut self) -> Result<String, LexError> {
        let mut value = String::new();
        while let Some(ch) = self.advance() {
            if self.is_at_end() {
                return Err(LexError::UnexpectedEOF);
            }
            if ch == DOUBLE_QUOTE {
                // self.advance();
                return Ok(value);
            }

            if ch.is_alphanumeric() {
                value.push(ch);
            }

            // self.advance();
        }
        println!("{}", value);
        Ok(value)
    }
    pub fn is_at_end(&self) -> bool {
        self.current_position >= self.raw.len()
    }

    fn scan_number(&mut self, first_digit: char) -> Result<f64, LexError> {
        let mut num = String::from(first_digit);
        let mut seen_dot = false;
        while let Some(ch) = self.advance() {
            if ch.is_numeric() {
                num.push(ch);
            }

            if ch == RIGHT_CURLY || ch == COMA {
                break;
            }
            if ch == '.' && !seen_dot {
                seen_dot = true;
                num.push(ch);
            } 

        }
        let parsed = num.parse::<f64>().map_err(|_| LexError::FailedNumberParse);
        parsed
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();
        match self.advance() {
            Some(ch) => {
                println!("Current char: {}", ch);
                let token = match ch {
                    LEFT_CURLY => Token::LeftCurly,
                    RIGHT_CURLY => Token::RightCurly,
                    LEFT_BRACKET => Token::LeftBracket,
                    RIGHT_BRACKET => Token::RigthBracket,
                    COLON => Token::Colon,
                    DOUBLE_QUOTE => Token::Literal(self.scan_string()?),
                    ch if ch.is_ascii_digit() => Token::Number(self.scan_number(ch)?),
                    _ => return Err(LexError::UnexpectedChar),
                };
                // self.advance();
                Ok(token)
            }
            None => return Err(LexError::UnexpectedEOF),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut lex = Lexer::new("{}");
        assert_eq!(Token::LeftCurly, lex.next_token().unwrap())
    }

    #[test]
    fn test_with_whitespace() {
        let mut lex = Lexer::new("{   }");
        lex.next_token().unwrap();
        assert_eq!(Token::RightCurly, lex.next_token().unwrap())
    }

    #[test]
    fn test_string_token() {
        let mut lex = Lexer::new("{\"22key\": \"value\"}");
        lex.next_token().unwrap();
        assert_eq!(Token::Literal("22key".to_string()), lex.next_token().unwrap());
        assert_eq!(Token::Colon, lex.next_token().unwrap());
        assert_eq!(
            Token::Literal("value".to_string()),
            lex.next_token().unwrap()
        );
    }

    #[test]
    fn test_number_without_dot() {
        let mut lex = Lexer::new("{\"key\": 123}");
        lex.next_token().unwrap();
        lex.next_token().unwrap();
        lex.next_token().unwrap();
        assert_eq!(Token::Number(123_f64), lex.next_token().unwrap());
    }

    #[test]
    fn test_number_with_dot() {
        let mut lex = Lexer::new("{\"key\": 123.45}");
        lex.next_token().unwrap();
        lex.next_token().unwrap();
        lex.next_token().unwrap();
        assert_eq!(Token::Number(123.45), lex.next_token().unwrap());
    }
}
