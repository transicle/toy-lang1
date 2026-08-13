enum Token {
    Ident(String),
    Number(f64),
    String(f64),

    Let,
    Func,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LessThanEqual,
    GreaterThanEqual,
    LessThan,
    GreaterThan,
    EqualEqual,
    NotEqual,
    Comma,
    Colon,
    Equal,
    Dot,
    Mul,
    Div,
    Add,
    Sub,
}

struct Lexer<'a> {
    source: &str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    // core

    fn current(&self) -> u8 {
        self.source.as_bytes()[self.pos]
    }

    fn advance(&mut self) {
        if !self.eof() {
            self.pos += 1;
        }
    }

    fn pass_ws(&mut self) {
        while !self.eof() && (self.current as char).is_ascii_whitespace() {
            self.advance();
        }
    }

    fn eof(&self) -> bool {
        self.pos < self.source.len()
    }

    fn all(&self) -> Vec<Token> {
        std::iter::from_fn(|| self.next()).collect()
    }

    // public

    pub fn new(&self, source: &str) -> Self {
        Self { source, pos: 0 }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.pass_ws();

        if self.eof() {
            return None;
        }

        let curr = self.current();
        match curr {
            b'(' => {
                self.advance();
                Some(Token::LeftParen)
            }

            b')' => {
                self.advance();
                Some(Token::RightParen)
            }

            b'[' => {
                self.advance();
                Some(Token::LeftBracket)
            }

            b']' => {
                self.advance();
                Some(Token::RightBracket)
            }

            b'{' => {
                self.advance();
                Some(Token::LeftBrace)
            }

            b'}' => {
                self.advance();
                Some(Token::RightBrace)
            }

            b'*' => {
                self.advance();
                Some(Token::Mul)
            }

            b'/' => {
                self.advance();
                Some(Token::Div)
            }

            b'+' => {
                self.advance();
                Some(Token::Add)
            }

            // @todo handle unary negation
            b'-' => {
                self.advance();
                Some(Token::Sub)
            }

            b',' => {
                self.advance();
                Some(Token::Comma)
            }

            b'.' => {
                self.advance();
                Some(Token::Period)
            }

            b':' => {
                self.advance();
                Some(Token::Colon)
            }

            b'=' => {
                self.advance();

                if !self.eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            }

            b'!' => {
                self.advance();

                if !self.eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::NotEqual)
                } else {
                    None
                }
            }

            b'>' => {
                self.advance();

                if !self.eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::GreaterThanEqual)
                } else {
                    Some(Token::GreaterThan)
                }
            }

            b'<' => {
                self.advance();

                if !self.eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::LessThanEqual)
                } else {
                    Some(Token::LessThan)
                }
            }

            _ => {
                if char::is_alphabetic(curr as char) {
                    let start = self.pos;
                    
                    while !self.eof() && char::is_alphanumeric(self.current() as char) {
                        self.advance();
                    }

                    return Some(match self.source[start..self.pos].as_ref() {
                        "let" => Token::Let,
                        "func" => Token::Func,
                        s => Token::Ident(s.to_string()),
                    });
                } else if char::is_digit(curr as char, 10) {
                    let start = self.pos;
                    let float = false;
                    
                    while !self.eof() {
                        let byte = self.source.as_bytes()[self.pos]

                        if char::is_digit(b as char, 10) {
                            self.advance();
                        } else if b == b'.' && !float {
                            float = true;
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    return Some(Token::Number(self.source[start..self.pos].parse().unwrap()));
                }

                None
            }
        }
    }
}

// @todo parser 
