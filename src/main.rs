use std::io::{self};

use lexer::Lexer;
use token::Token;

mod lexer;
mod parser;
mod token;

fn main() {
    println!("Hello this is the Monkey programming language!\n");
    repl()
}

fn repl() {
    let lines = io::stdin().lines();
    for line in lines {
        let line = line.unwrap();
        let mut lexer = Lexer::new(&line);
        let mut tok = lexer.next_token();
        while tok != Token::Eof {
            println!("{:?}", tok);
            tok = lexer.next_token();
        }
    }
}
