use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = self.next().unwrap().clone();
                    let rhs = self.parse_factor();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op,
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while let Some(token) = self.peek() {
            match token {
                Token::Star | Token::Slash => {
                    let op = self.next().unwrap().clone();
                    let rhs = self.parse_primary();
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op,
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.next() {
            Some(Token::Number(n)) => Expr::Number(*n),
            Some(Token::LParen) => {
                let expr = self.parse_expr();
                assert_eq!(self.next(), Some(&Token::RParen));
                expr
            }
            other => panic!("Unexpected token: {:?}", other),
        }
    }
}
