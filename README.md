# Expression Parser in Rust

A modular, high-performance arithmetic expression parser and evaluator written in Rust. This project tokenises user-inputted math expressions, constructs an abstract syntax tree (AST), and computes the result using recursive descent parsing.

It showcases compiler design fundamentals, including lexical analysis, syntax parsing, and evaluation — all in under 300 lines of safe, idiomatic Rust.

---

## Features

- Tokenisation of arithmetic expressions (lexer)
- AST generation with recursive descent parsing
- Support for `+`, `-`, `*`, `/` operators and parentheses `()`
- Evaluation of expressions using AST traversal
- Clean modular structure (`lexer.rs`, `parser.rs`, `evaluator.rs`)
- Example test inputs included

---

## Project Structure
expression-parser-rust/
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── parser.rs
│   └── evaluator.rs
├── tests/
│   └── sample_expressions.txt
└── README.md

