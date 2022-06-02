use std::collections::BTreeMap;

use crate::{Token, Value};

#[derive(Debug, Clone)]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> ParserError {
        ParserError {
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

    fn parse_array(&mut self) -> Result<Value, ParserError> {
        todo!()
    }

    /// `Object`のパースを行う。
    /// {
    ///   "key1": 12345,
    ///   "key2": 6789
    /// }
    fn parse_object(&mut self) -> Result<Value, ParserError> {
        let token = self.peek_expect()?;
        if *token != Token::LeftBrace {
            return Err(ParserError::new(&format!(
                "error: JSON object must starts {{ {:?}",
                token
            )));
        }

        self.next_expect()?;

        let mut object = BTreeMap::new();

        if *self.peek_expect()? == Token::RightBrace {
            return Ok(Value::Object(object));
        }

        loop {
            let token1 = self.next_expect()?.clone();
            let token2 = self.next_expect()?;

            match (token1, token2) {
                (Token::String(key), Token::Colon) => {
                    object.insert(key.to_string(), self.parse()?);
                }
                _ => {
                    return Err(ParserError::new(
                        "error: a pair (key(string) and : token) token is expected",
                    ));
                }
            }
            let token3 = self.next_expect()?;
            match token3 {
                Token::RightBrace => {
                    return Ok(Value::Object(object));
                }
                Token::Comma => {
                    continue;
                }
                _ => {
                    return Err(ParserError::new(&format!(
                        "error: a {{ or , token is expected {:?}",
                        token3
                    )));
                }
            }
        }
    }

    pub fn parse(&mut self) -> Result<Value, ParserError> {
        let token = self.peek_expect()?.clone();
        let value = match token {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::String(s) => Ok(Value::String(s)),
            Token::Number(n) => Ok(Value::Number(n)),
            Token::Bool(b) => Ok(Value::Bool(b)),
            Token::Null => Ok(Value::Null),
            _ => {
                return Err(ParserError::new(&format!(
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

    fn peek_expect(&self) -> Result<&Token, ParserError> {
        self.peek()
            .ok_or_else(|| ParserError::new("error: a token is not peekable"))
    }

    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }

    fn next_expect(&mut self) -> Result<&Token, ParserError> {
        self.next()
            .ok_or_else(|| ParserError::new("error: can not get next token"))
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
