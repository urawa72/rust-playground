use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    WhiteSpace,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
}

/// JSON 文字列を Parse して Token 単位に分割
pub struct Lexer<'a> {
    /// 読み込み中の先頭文字列を指す
    chars: Peekable<Chars<'a>>,
}

#[derive(Debug)]
pub enum LexerError {
    Null(String),
    Unexpected(String),
    Bool(String),
    Number(String),
    Char(String),
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                Token::WhiteSpace => {}
                _ => {
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }

    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.chars.peek() {
            // 1 文字分だけ読み進め Token を返す
            Some(c) => match c {
                // WhiteSpace は ' ' もしくは '\n'
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(self.next_return_token(Token::WhiteSpace))
                }
                '{' => Ok(self.next_return_token(Token::LeftBrace)),
                '}' => Ok(self.next_return_token(Token::RightBrace)),
                '[' => Ok(self.next_return_token(Token::LeftBracket)),
                ']' => Ok(self.next_return_token(Token::RightBracket)),
                ',' => Ok(self.next_return_token(Token::Comma)),
                ':' => Ok(self.next_return_token(Token::Colon)),
                // String は開始文字列 '"'
                '"' => {
                    self.chars.next();
                    self.parse_string_token()
                }
                // Number は開始文字が [0-9] もしくは ('+', '-', '.')
                c if c.is_numeric() || matches!(c, '+' | '-' | '.') => self.parse_number_token(),
                // Boolean の true の開始文字は 't'
                't' => self.parse_bool_token(true),
                // Boolean の false の開始文字は 'f'
                'f' => self.parse_bool_token(false),
                // Null の開始文字は 'n'
                'n' => self.parse_null_token(),
                // 上のルールにマッチしない文字はエラー
                unknown => Err(LexerError::Unexpected(format!("{unknown}"))),
            },
            None => Ok(None),
        }
    }

    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        let s = (0..4).filter_map(|_| self.chars.next()).collect::<String>();
        if s == "null" {
            Ok(Some(Token::Null))
        } else {
            Err(LexerError::Null(format!("{s}")))
        }
    }

    fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
        if b {
            let s = (0..4).filter_map(|_| self.chars.next()).collect::<String>();
            if s == "true" {
                Ok(Some(Token::Bool(true)))
            } else {
                Err(LexerError::Bool(format!("{s}")))
            }
        } else {
            let s = (0..5).filter_map(|_| self.chars.next()).collect::<String>();
            if s == "false" {
                Ok(Some(Token::Bool(false)))
            } else {
                Err(LexerError::Bool(format!("{s}")))
            }
        }
    }

    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut number_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() | matches!(c, '+' | '-' | 'e' | 'E' | '.') {
                self.chars.next();
                number_str.push(c);
            } else {
                break;
            }
        }

        match number_str.parse::<f64>() {
            Ok(n) => Ok(Some(Token::Number(n))),
            Err(e) => Err(LexerError::Number(format!("{e}"))),
        }
    }

    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut utf16 = vec![];
        let mut result = String::new();

        while let Some(c1) = self.chars.next() {
            match c1 {
                // Escapeの開始文字'\\'
                '\\' => {
                    let c2 = self
                        .chars
                        .next()
                        .ok_or_else(|| LexerError::Char(format!("")))?;
                    if matches!(c2, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') {
                        // 特殊なエスケープ文字列の処理
                        // https://www.rfc-editor.org/rfc/rfc8259#section-7
                        // utf16 のバッファを文字列に push しておく
                        Self::push_utf16(&mut result, &mut utf16)?;
                        result.push('\\');
                        result.push(c2);
                    } else if c2 == 'u' {
                        // UTF-16
                        // \u0000 ~ \uFFFF
                        // \u まで読み込んだので残りの 0000 ~ XXXX の 4 文字を読み込む
                        // UTF-16 に関してはエスケープ処理を行う
                        let hexs = (0..4)
                            .filter_map(|_| {
                                let c = self.chars.next()?;
                                if c.is_ascii_hexdigit() {
                                    Some(c)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        // 読み込んだ文字列を 16 進数として評価し utf16 のバッファに push しておく
                        match u16::from_str_radix(&hexs.iter().collect::<String>(), 16) {
                            Ok(code_point) => utf16.push(code_point),
                            Err(e) => {
                                return Err(LexerError::Char(format!("{e}")));
                            }
                        };
                    } else {
                        return Err(LexerError::Char(format!("{c2}")));
                    }
                }
                // 文字列の終端 '"'
                '\"' => {
                    Self::push_utf16(&mut result, &mut utf16)?;
                    return Ok(Some(Token::String(result)));
                }
                // それ以外の文字
                _ => {
                    // utf16 のバッファを文字列に push しておく
                    Self::push_utf16(&mut result, &mut utf16)?;
                    result.push(c1);
                }
            }
        }
        Ok(None)
    }

    /// utf16 のバッファが存在するならば連結しておく
    fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
        if utf16.is_empty() {
            return Ok(());
        }
        match String::from_utf16(utf16) {
            Ok(utf16_str) => {
                result.push_str(&utf16_str);
                utf16.clear();
            }
            Err(e) => {
                return Err(LexerError::Char(format!("{e}")));
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_null() {
        let null = "null";
        let tokens = Lexer::new(null).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Null);
    }

    #[test]
    fn bool() {
        let b = "true";
        let tokens = Lexer::new(b).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Bool(true));

        let b = "false";
        let tokens = Lexer::new(b).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Bool(false));
    }

    #[test]
    fn number() {
        let num = "1234567890";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(1234567890f64));

        let num = "+123";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(123f64));

        let num = "-0.001";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(-0.001));

        let num = ".001";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(0.001));

        let num = "1e-10";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(0.0000000001));

        let num = "+2E10";
        let tokens = Lexer::new(num).tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(20000000000f64));
    }

    #[test]
    fn string() {
        let s = "\"aiueo123\"";
        let tokens = Lexer::new(s).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("aiueo123".to_string()));

        let s = "\"あいうえお\"";
        let tokens = Lexer::new(s).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("あいうえお".to_string()));

        let s = r#""\u3042\u3044\u3046abc""#;
        let tokens = Lexer::new(s).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("あいうabc".to_string()));

        let s = format!(r#" " \b \f \n \r \t \/ \" ""#);
        let tokens = Lexer::new(&s).tokenize().unwrap();
        assert_eq!(
            tokens[0],
            Token::String(r#" \b \f \n \r \t \/ \" "#.to_string())
        );

        let s = r#""\uD83D\uDE04\uD83D\uDE07\uD83D\uDC7A""#;
        let tokens = Lexer::new(&s).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String(r#"😄😇👺"#.to_string()));
    }

    #[test]
    fn tokenize() {
        let obj = r#"
        {
            "number": 123,
            "boolean": true,
            "string": "aiueo",
            "object": {
               "number": 2E10
            }
         }
         "#;
        let actual = Lexer::new(obj).tokenize().unwrap();
        let expected = vec![
            Token::LeftBrace,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(123f64),
            Token::Comma,
            Token::String("boolean".to_string()),
            Token::Colon,
            Token::Bool(true),
            Token::Comma,
            Token::String("string".to_string()),
            Token::Colon,
            Token::String("aiueo".to_string()),
            Token::Comma,
            Token::String("object".to_string()),
            Token::Colon,
            Token::LeftBrace,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(20000000000f64),
            Token::RightBrace,
            Token::RightBrace,
        ];
        assert_eq!(actual, expected);

        let arr = "[true, {\"キー\": null}]";
        let actual = Lexer::new(arr).tokenize().unwrap();
        let expected = vec![
            Token::LeftBracket,
            Token::Bool(true),
            Token::Comma,
            Token::LeftBrace,
            Token::String("キー".to_string()),
            Token::Colon,
            Token::Null,
            Token::RightBrace,
            Token::RightBracket,
        ];
        assert_eq!(actual, expected);
    }
}
