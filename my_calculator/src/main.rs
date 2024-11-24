mod lexer;
mod parser;
mod evaluator;

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

fn main() {
    loop {
        let mut input  = String::new();
        print!("Enter an mathematical expression(type \"quit\" to close application): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input == "quit".to_string() {
            break;
        }

        let mut lexer = Lexer::new();
        lexer.tokenize(input);
        println!("Tokens: {:?}", lexer.tokens);

        let mut parser = Parser::new(lexer.tokens.clone());
        match parser.parse_expression() {
            Ok(ast) => {
                println!("Evaluating...");
                Evaluator::evaluate_and_print(ast);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    
}