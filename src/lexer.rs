#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut num = 0;
                while let Some(d) = chars.peek().and_then(|c| c.to_digit(10)) {
                    num = num * 10 + d as i32;
                    chars.next();
                }
                tokens.push(Token::Number(num));
            }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '*' => { chars.next(); tokens.push(Token::Star); }
            '/' => { chars.next(); tokens.push(Token::Slash); }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            ' ' => { chars.next(); } // Skip whitespace
            _ => panic!("Unknown character: {}", ch),
        }
    }

    tokens
}
