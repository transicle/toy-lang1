#[derive(Debug)]
pub enum Token {
    Ident(String),
    Number(f64),

//  Keywords
    Let,

//  Symbols
    Plus,
    Minus,
    Mul,
    Div,

    LParen,
    RParen,

    Equals
}

pub struct Lexer<'a> {
    pub source: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
        }
    }

//  Core
    fn eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn skip_ws(&mut self) {
        while !self.eof() && (self.current() as char).is_whitespace() {
            self.consume();
        }
    }

    fn current(&self) -> u8 {
        self.source.as_bytes()[self.pos]
    }

    fn consume(&mut self) {
        if !self.eof() {
            self.pos += 1;
        }
    }

//  Public
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.eof() {
            self.skip_ws();

            match self.current() {
                b'+' => { self.consume(); tokens.push(Token::Plus); }
                b'-' => { self.consume(); tokens.push(Token::Minus); }
                b'/' => { self.consume(); tokens.push(Token::Div); }
                b'*' => { self.consume(); tokens.push(Token::Mul); }
                b'=' => { self.consume(); tokens.push(Token::Equals); }
                b'(' => { self.consume(); tokens.push(Token::LParen); }
                b')' => { self.consume(); tokens.push(Token::RParen); }

                _ => {
                    if char::is_alphabetic(self.current() as char) || self.current() == b'_' {
                        let start = self.pos;

                        while !self.eof() && char::is_alphanumeric(self.current() as char) {
                            self.consume();
                        }

                        tokens.push(match self.source[start..self.pos].as_ref() {
                            "let" => Token::Let,

                            ident => Token::Ident(ident.to_string())
                        });
                    } else if char::is_digit(self.current() as char, 10) {
                        let start = self.pos;
                        let mut is_float = false;

                        while !self.eof() {
                            let byte = self.current();

                            if char::is_digit(byte as char, 10) {
                                self.consume();
                            } else if byte == b'.' && !is_float {
                                is_float = true;
                                self.consume();
                            } else {
                                break;
                            }
                        }

                        tokens.push(Token::Number(self.source[start..self.pos].parse().unwrap()));
                    }
                }
            }

            self.consume();
        }

        tokens
    }
}
