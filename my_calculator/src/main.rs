mod lexer;
mod parser;
mod evaluator;

use lexer::Lexer;
use parser::{ASTNode, Parser};
use evaluator::Evaluator;

fn main() {
    let input = "-(2 - -5)";
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