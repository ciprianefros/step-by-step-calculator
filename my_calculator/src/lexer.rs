#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Pi,
    Euler,
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Abs,
    Sqrt,
    Log,
    Sin,
    Cos,
    Tg,
    Cotg,
    LParen,
    RParen,
    Eof,
}
#[derive(Clone, Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}
impl Lexer {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    pub fn tokenize(&mut self, input: &str) {
        if input.len() > 10_000 {
            eprintln!(
                "The mathematical expression is too large! Please enter a reasonable expression!"
            );
            return;
        }
        let mut chars = input.chars().peekable();
        let mut buffer = String::new();

        self.tokens.clear();

        while let Some(&ch) = chars.peek() {
            match ch {
                '+' => {
                    self.tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    self.tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    self.tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    self.tokens.push(Token::Divide);
                    chars.next();
                }
                '(' => {
                    self.tokens.push(Token::LParen);
                    chars.next();
                }
                ')' => {
                    self.tokens.push(Token::RParen);
                    chars.next();
                }
                '^' => {
                    chars.next();
                    if chars.peek() == Some(&'^') {
                        self.tokens.push(Token::Exponent);
                        chars.next();
                    } else {
                        eprintln!("Invalid token: ^");
                    }
                }
                '0'..='9' | '.' => {
                    buffer.clear();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_numeric() || ch == '.' {
                            buffer.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(number) = buffer.parse::<f64>() {
                        self.tokens.push(Token::Number(number));
                    } else {
                        eprintln!("Invalid number: {}", buffer);
                    }
                }
                'a'..='z' => {
                    buffer.clear();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphabetic() {
                            buffer.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match buffer.as_str() {
                        "abs" => self.tokens.push(Token::Abs),
                        "sqrt" => self.tokens.push(Token::Sqrt),
                        "log" => self.tokens.push(Token::Log),
                        "sin" => self.tokens.push(Token::Sin),
                        "cos" => self.tokens.push(Token::Cos),
                        "tg" => self.tokens.push(Token::Tg),
                        "cotg" => self.tokens.push(Token::Cotg),
                        "pi" => self.tokens.push(Token::Pi),
                        "e" => self.tokens.push(Token::Euler),
                        _ => eprintln!("Invalid keyword: {}", buffer),
                    }
                }
                ' ' => {
                    chars.next();
                }
                _ => {
                    eprintln!("Unrecongnized character: {}", ch);
                    chars.next();
                }
            }
        }
        self.tokens.push(Token::Eof);
    }
    //pub fn get_tokens(self) -> Vec<Token> {
    //   self.tokens.clone()
    //}
}
