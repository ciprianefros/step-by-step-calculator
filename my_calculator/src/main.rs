mod evaluator;
mod lexer;
mod parser;
mod utils;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};
use std::{thread, time};
use utils::{delete_saved_evaluations, save_to_file};

fn main() {
    println!("Welcome to the Step-by-Step Calculator!");
    println!("This calculator evaluates mathematical expressions step by step!");
    loop {
        println!("\nMain Menu:");
        println!("1. Start a new calculation");
        println!("2. View available commands and calculator operations");
        println!("3. Delete saved evaluations");
        println!("4. Quit");

        println!("Choose an option (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => start_calculator(),
            "2" => show_available_commands(),
            "3" => match delete_saved_evaluations() {
                Ok(_) => println!("All saved evaluations have been deleted successfully"),
                Err(e) => eprintln!("Failed to delete evaluations: {}", e),
            },
            "4" => {
                println!("See you next time!");
                break;
            }
            _ => println!("Invalid option. Please choose a valid number (1-4)."),
        }
    }
}

fn start_calculator() {
    loop {
        let mut input = String::new();
        print!("Enter a mathematical expression (or type \"quit\" to return to the main menu): ");

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        let mut lexer = Lexer::new();
        lexer.tokenize(input);
        //println!("Tokens: {:?}", lexer.tokens);

        let mut parser = Parser::new(lexer.tokens.clone());
        let mut evaluator = Evaluator::new();
        match parser.parse_expression() {
            Ok(ast) => {
                println!("Evaluating...");
                evaluator.evaluate_and_print(ast);
                println!("Would you like to save this evaluation process?(y/n)");
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read input");
                let answer = answer.trim();
                if answer.eq_ignore_ascii_case("y") {
                    println!("Give it a name: ");
                    let mut file_name = String::new();
                    io::stdin()
                        .read_line(&mut file_name)
                        .expect("Failed to read input");
                    let file_name = file_name.trim();
                    match save_to_file(file_name, &evaluator.get_evaluation_steps()) {
                        Ok(_) => println!("Evaluation saved succesfully."),
                        Err(e) => eprintln!("Failed to save file: {}", e),
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

fn show_available_commands() {
    println!("\nAvailable Calculator operators and Commands:");
    println!("- Basic arithmetic operators: +, -, *, /");
    println!("- Exponentiation: ^ (e.g., 2 ^ 3)");
    println!("- Trigonometric functions: sin, cos, tg, cotg (in degrees)");
    println!("- Logarithmic functions: log (base 10)");
    println!("- Square root: sqrt");
    println!("- Absolute value: abs");
    println!("- Constants: pi (3.14159), e (2.71828)");
    println!("- Parentheses for grouping: ( and )");
    println!("- Step-by-step evaluation of expressions.");
    println!("- Save evaluations to files.");
    println!("- Delete all saved evaluations.");
    println!("\nType \"quit\" at any time to exit a sub-menu.");
    let sleep_time = time::Duration::from_secs(10);
    thread::sleep(sleep_time);
}
