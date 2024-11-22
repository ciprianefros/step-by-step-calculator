mod lexer;
mod parser;
mod evaluator;

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

fn main() {
    
    print!("Enter an mathematical expression: ");
    io::stdout().flush().unwrap();
    let mut input  = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();

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