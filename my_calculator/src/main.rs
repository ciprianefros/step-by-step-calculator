mod evaluator;
mod lexer;
mod parser;
mod utils;
use lexer::Lexer;

fn main() {
    let input = "30.6 + 4 * sin(2) - pi / 2";
    let mut lexer = Lexer::new();
    lexer.tokenize(input);
    println!("{:?}", lexer.tokens);
}

