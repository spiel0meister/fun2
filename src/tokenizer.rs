use std::io::{self, Error, ErrorKind};

macro_rules! add_token {
    ($tokenizer:ident, $token_type:expr, $token_value:expr) => {
        $tokenizer.tokens.push(Token {
            token_type: $token_type,
            value: $token_value,
        });
    };
}

#[derive(Clone, Debug)]
pub enum LiteralType {
    Int,
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Semicolon,
    Ident,
    Literal(LiteralType),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Tokenizer {
    text: String,
    index: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn peek(&mut self, offset: usize) -> Option<char> {
        if self.index + offset > self.text.len() {
            None
        } else {
            self.text.chars().nth(self.index + offset)
        }
    }

    fn consume(&mut self) -> io::Result<char> {
        let Some(cur) = self.text.chars().nth(self.index) else {
            return Err(Error::new(ErrorKind::Other, "EOF"));
        };
        self.index += 1;
        Ok(cur)
    }

    pub fn tokenize(&mut self) -> io::Result<Vec<Token>> {
        while self.peek(0).is_some() {
            let mut char = self.peek(0).unwrap();

            if char.is_whitespace() {
                self.consume()?;
                continue;
            } else if char.is_ascii_alphabetic() || char == '_' {
                let mut buf = String::new();
                buf.push(char);
                self.consume()?;
                char = self.peek(0).unwrap();
                while char.is_ascii_alphanumeric() {
                    buf.push(char);
                    self.consume()?;
                    char = self.peek(0).unwrap();
                }
                add_token!(self, TokenType::Ident, buf);
            } else if char.is_ascii_digit() {
                let mut buf = String::new();
                buf.push(char);
                self.consume()?;
                char = self.peek(0).unwrap();
                while char.is_ascii_alphanumeric() {
                    buf.push(char);
                    self.consume()?;
                    char = self.peek(0).unwrap();
                }
                add_token!(self, TokenType::Literal(LiteralType::Int), buf);
            } else if char == ';' {
                add_token!(self, TokenType::Semicolon, ";".to_string());
                self.consume()?;
            }
        }

        Ok(self.tokens.to_vec())
    }
}
