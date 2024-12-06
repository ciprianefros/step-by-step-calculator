use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(f64),
    Pi,
    Euler,
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    UnaryOp {
        op: Token,
        operand: Box<ASTNode>,
    },
    Function {
        func: Token,
        argument: Box<ASTNode>,
    },
    LogBase {
        base: Box<ASTNode>,
        number : Box<ASTNode>,
    },
    Grouping(Box<ASTNode>),
}
#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    fn next_token(&mut self) -> Option<&Token> {
        self.position += 1;
        //println!("Current token at position {}: {:?}", self.position, self.current_token());
        self.current_token()
    }
    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_binary_op(0)?;
        if let Some(Token::Eof) = self.current_token() {
            Ok(expr)
        } else {
            Err("Unexpected input after end of expression".to_string())
        }
    }
    fn parse_inner_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_binary_op(0)
    }
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        if let Some(token) = self.current_token().cloned() {
            match token {
                Token::Number(value) => {
                    self.next_token();
                    let mut node = ASTNode::Number(value);

                    if let Some(Token::Fact) = self.current_token() {
                        self.next_token();
                        node = ASTNode::UnaryOp {
                            op: Token::Fact,
                            operand: Box::new(node),
                        }
                    }
                    Ok(node)
                }
                Token::Pi => {
                    self.next_token();
                    let mut node = ASTNode::Pi;
                    if let Some(Token::Fact) = self.current_token() {
                        self.next_token();
                        node = ASTNode::UnaryOp {
                            op: Token::Fact,
                            operand: Box::new(node),
                        };
                    }
                    Ok(node)
                }
                Token::Euler => {
                    self.next_token();
                    let mut node = ASTNode::Euler;
                    if let Some(Token::Fact) = self.current_token() {
                        self.next_token();
                        node = ASTNode::UnaryOp {
                            op: Token::Fact,
                            operand: Box::new(node),
                        };
                    }
                    Ok(node)
                }
                Token::Minus => {
                    self.next_token();
                    let operand = self.parse_primary()?;
                    Ok(ASTNode::UnaryOp {
                        op: Token::Minus,
                        operand: Box::new(operand),
                    })
                }
                Token::LParen => {
                    self.next_token();
                    let expr = self.parse_inner_expression()?;
                    if let Some(Token::RParen) = self.current_token() {
                        self.next_token();
                        let mut node = ASTNode::Grouping(Box::new(expr));

                        if let Some(Token::Fact) = self.current_token() {
                            self.next_token();
                            node = ASTNode::UnaryOp {
                                op: Token::Fact,
                                operand: Box::new(node),
                            };
                        }

                        Ok(node)
                    } else {
                        Err("Expected right parenthesis".to_string())
                    }
                }
                Token::Log => {
                    self.next_token();
                    if let Some(Token::LParen) = self.current_token() {
                        self.next_token();
                    } else {
                        return Err("Expected '(' after log function".to_string());
                    }

                    let first_arg = self.parse_inner_expression()?;


                    let base;
                    let number;

                    if let Some(Token::Comma) = self.current_token() {
                        self.next_token();
                        base = first_arg;
                        number = self.parse_inner_expression()?;
                    } else {
                        base = ASTNode::Number(2.0);
                        number = first_arg;
                    }

                    if let Some(Token::RParen) = self.current_token() {
                        self.next_token();
                    } else {
                        return Err("After the log function arguments there should be ')'".to_string());
                    }
                    let mut node = ASTNode::LogBase {
                        base: Box::new(base),
                        number: Box::new(number),
                    };
                    if let Some(Token::Fact) = self.current_token() {
                        self.next_token();
                        node = ASTNode::UnaryOp {
                            op: Token::Fact,
                            operand: Box::new(node),
                        };
                    }
                    Ok(node)
                },
                Token::Sin
                | Token::Cos
                | Token::Tg
                | Token::Cotg
                | Token::Ln
                | Token::Sqrt
                | Token::Abs
                | Token::Sec
                | Token::Csc
                | Token::Asin
                | Token::Acos
                | Token::Atg
                | Token::Actg => {
                    let func = token;
                    self.next_token();

                    if let Some(Token::LParen) = self.current_token() {
                        self.next_token();
                        let argument = self.parse_inner_expression()?;

                        if let Some(Token::RParen) = self.current_token() {
                            self.next_token();
                            let mut node = ASTNode::Function {
                                func,
                                argument: Box::new(argument),
                            };
                            if let Some(Token::Fact) = self.current_token() {
                                self.next_token();
                                node = ASTNode::UnaryOp {
                                    op: Token::Fact,
                                    operand: Box::new(node),
                                };
                            }

                            Ok(node)
                        } else {
                            Err("Expected right parenthesis after function argument".to_string())
                        }
                    } else {
                        Err("Expected '(' after function name".to_string())
                    }
                }
                _ => Err("Unexpected token".to_string()),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
    fn get_precedence(op: &Token) -> u8 {
        match op {
            Token::Plus | Token::Minus  => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Exponent => 3,
            _ => 0,
        }
    }
    fn parse_binary_op(&mut self, min_precedence: u8) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.current_token() {
            if op == &Token::Eof || op == &Token::RParen || op == &Token::Comma {
                break;
            }

            let precedence = Parser::get_precedence(op);
            if precedence < min_precedence {
                break;
            }

            let op = *self.current_token().unwrap();
            self.next_token();

            let right = self.parse_binary_op(precedence + 1)?;
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Lexer, Token};

    fn lex_input(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new();
        lexer.tokenize(input);
        lexer.tokens
    }

    #[test]
    fn test_simple_addition() {
        let tokens = lex_input("2 + 3");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::BinaryOp {
                left: Box::new(ASTNode::Number(2.0)),
                op: Token::Plus,
                right: Box::new(ASTNode::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_constants() {
        let tokens = lex_input("pi + e");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::BinaryOp {
                left: Box::new(ASTNode::Pi),
                op: Token::Plus,
                right: Box::new(ASTNode::Euler),
            }
        );
    }

    #[test]
    fn test_unary_operations() {
        let tokens = lex_input("-5!");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::UnaryOp {
                op: Token::Minus,
                operand: Box::new(ASTNode::UnaryOp {
                    op: Token::Fact,
                    operand: Box::new(ASTNode::Number(5.0)),
                }),
            }
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = lex_input("sin(pi)");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::Function {
                func: Token::Sin,
                argument: Box::new(ASTNode::Pi),
            }
        );
    }

    #[test]
    fn test_grouping_and_precedence() {
        let tokens = lex_input("(2 + 3) * 4");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::BinaryOp {
                left: Box::new(ASTNode::Grouping(Box::new(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::Number(2.0)),
                    op: Token::Plus,
                    right: Box::new(ASTNode::Number(3.0)),
                }))),
                op: Token::Multiply,
                right: Box::new(ASTNode::Number(4.0)),
            }
        );
    }

    #[test]
    fn test_complex_expression() {
        let tokens = lex_input("3 + sin(2 * pi) - log(2,10) ^ 2");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::BinaryOp {
                left: Box::new(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::Number(3.0)),
                    op: Token::Plus,
                    right: Box::new(ASTNode::Function {
                        func: Token::Sin,
                        argument: Box::new(ASTNode::BinaryOp {
                            left: Box::new(ASTNode::Number(2.0)),
                            op: Token::Multiply,
                            right: Box::new(ASTNode::Pi),
                        }),
                    }),
                }),
                op: Token::Minus,
                right: Box::new(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::LogBase {
                        base: Box::new(ASTNode::Number(2.0)),
                        number: Box::new(ASTNode::Number(10.0)),
                    }),
                    op: Token::Exponent,
                    right: Box::new(ASTNode::Number(2.0)),
                }),
            }
        );
    }

    #[test]
    fn check_factorial_after_functions() {
        let tokens = lex_input("sin(30) + 4!");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression().unwrap();

        assert_eq!(
            ast,
            ASTNode::BinaryOp {
                left: Box::new(ASTNode::Function { func: Token::Sin, argument: Box::new(ASTNode::Number(30.0)) }),
                op: Token::Plus,
                right: Box::new(ASTNode::UnaryOp { op: Token::Fact, operand: Box::new(ASTNode::Number(4.0)) }),     
            }
        );
    }
}