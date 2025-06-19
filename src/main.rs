mod lexer;
mod parser;
mod evaluator;

use std::io::{self, Write};
use lexer::lex;
use parser::Parser;
use evaluator::evaluate;

fn main() {
    println!("Expression Parser in Rust");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let tokens = lex(&input);
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();
        let result = evaluate(&expr);

        println!("= {}", result);
    }
}
