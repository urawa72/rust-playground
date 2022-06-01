use crate::{Token, Value};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub msg: String,
}

impl ParseError {
    pub fn new(msg: &str) -> ParseError {
        ParseError {
            msg: msg.to_string(),
        }
    }
}

pub struct Parser {
    /// Lexer で tokenize した Token
    tokens: Vec<Token>,
    /// tokens の先頭
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    fn parse_array(&mut self) -> Result<Value, ParseError> {
        todo!()
    }

    fn parse_object(&mut self) -> Result<Value, ParseError> {
        todo!()
    }

    pub fn parse(&mut self) -> Result<Value, ParseError> {
        let token = self.peek_expect()?.clone();
        let value = match token {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::String(s) => {
                Ok(Value::String(s.to_string()))
            }
            Token::Number(n) => {
                Ok(Value::Number(*n))
            }
            Token::Bool(b) => {
                Ok(Value::Bool(*b))
            }
            Token::Null => {
                Ok(Value::Null)
            }
            _ => {
                return Err(ParseError::new(&format!(
                    "error: a token must start {{, [, string, number, bool or null {:?}",
                    token
                )));
            }
        };
        self.next_expect()?;
        value
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn peek_expect(&self) -> Result<&Token, ParseError> {
        self.peek()
            .ok_or_else(|| ParseError::new("error: a token is not peekable"))
    }

    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }

    fn next_expect(&mut self) -> Result<&Token, ParseError> {
        self.next()
            .ok_or_else(|| ParseError::new("error: can not get next token"))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{lexer::Lexer, Value};

    #[test]
    fn test_parse_object() {
        todo!()
    }

    #[test]
    fn test_parse_array() {
        todo!()
    }

    #[test]
    fn test_parse() {
        todo!()
    }
}
