mod lexer;
mod parser;
mod evaluator;
mod utils;

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;
use utils::save_to_file;

fn main() {
    loop {
        let mut input  = String::new();
        print!("Enter an mathematical expression(type \"quit\" to close application): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input == "quit" {
            break;
        }

        let mut lexer = Lexer::new();
        lexer.tokenize(input);
        println!("Tokens: {:?}", lexer.tokens);

        let mut parser = Parser::new(lexer.tokens.clone());
        let mut evaluator = Evaluator::new();
        match parser.parse_expression() {
            Ok(ast) => {
                println!("Evaluating...");
                evaluator.evaluate_and_print(ast);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        println!("Would you like to save this evaluation process?(y/n)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read input");
        let answer = answer.trim();
        if answer.to_lowercase() == "y" {
            println!("Give it a name: ");
            let mut file_name = String::new();
            io::stdin().read_line(& mut file_name).expect("Failed to read input");
            let file_name = file_name.trim();
            match save_to_file(file_name, &evaluator.get_evaluation_steps()) {
                Ok(_) => println!("Evaluation saved succesfully."),
                Err(e) => eprintln!("Failed to save file: {}", e),
            }
        }

    }
    
}