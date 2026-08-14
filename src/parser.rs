enum Token {
    Ident(String),
    Number(f64),
    String(f64),

    Let,
    Func,
    Struct,
    If,
    Else,

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
            b'(' => { self.advance(); Some(Token::LeftParen) }
            b')' => { self.advance(); Some(Token::RightParen) }
            b'[' => { self.advance(); Some(Token::LeftBracket) }
            b']' => { self.advance(); Some(Token::RightBracket) }
            b'{' => { self.advance(); Some(Token::LeftBrace) }
            b'}' => { self.advance(); Some(Token::RightBrace) }
            b'*' => { self.advance(); Some(Token::Mul) }
            b'/' => { self.advance(); Some(Token::Div) }
            b'+' => { self.advance(); Some(Token::Add) }
            b'-' => { self.advance(); Some(Token::Sub) }
            b',' => { self.advance(); Some(Token::Comma) }
            b'.' => { self.advance(); Some(Token::Period) }
            b':' => { self.advance(); Some(Token::Colon) }

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
                        "struct" => Token::Struct,
                        "if" => Token::If,
                        "else" => Token::Else,
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

#[derive(Debug, PartialEq, Clone)]
enum BinaryOps {
    Mul,
    Div,
    Add,
    Sub,
    LessThan,
    GreaterThan,
    GreaterThanEqual,
    LessThanEqual,
    EqualEqual,
    NotEqual,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    Ident(String),
    List(Vec<Expr>),
    BinaryOp {
        op: char,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    Call {
        callee: String,
        args: Vec<Expr>
    },

    FieldAccess {
        receiver: Box<Expr>,
        field: String
    },

    If {
        condition: Box<Expr>,
        then: Vec<Stmt>,
        else_: Option<Vec<Stmt>>
    },
}

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    type_: String
}

#[derive(Debug, PartialEq)]
enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        value: Expr,
    },
    
    FuncDecl {
        name: String,
        params: Vec<Expr>,
        body: Vec<Stmt>,
    },

    Struct {
        name: String,
        fields: Vec<Field>
    }
}

#[derive(Debug, PartialEq)]
struct Program {
    statements: Vec<Stmt>
}

#[derive(Debug)]
enum ParseError {
    Unexpected {
        expected: Token,
        found: Token,
        msg: &'static str
    },

    UnexpectedToken(Token),
    UnexpectedEof,
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a source) -> Self {
        let mut parser = {
            lexer: Lexer::new(source),
            current: None
        };

        parser.consume();
        parser
    }

    fn consume(&mut self) -> Option<Token> {
        let previous = self.current.take();

        self.current = self.lexer.next();
        previous
    }

    fn expect(&mut self, token: Token, msg: &'static str) -> Result<Token, ParseError> {
        match &self.current {
            Some(cur) if *c == token => Ok(self.consume().unwrap()),
            Some(cur) => Err(ParseError::Unexpected {
                expected: token,
                found: cur.clone(),
                msg
            }),

            None => Err(ParseError::UnexpectedEof)
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec<Stmt> = vec![];
        while self.current.is_some() {
            statements.push(self.parse_stmt()?);
        }

        Ok(Program {
            statements
        })
    }

    // line 303 in tut
}