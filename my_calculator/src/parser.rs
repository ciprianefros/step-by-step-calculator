use crate::lexer::Token;

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
    Grouping(Box<ASTNode>),
}
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
                            operand : Box::new(node),
                        }
                    }
                    Ok(node)
                }
                Token::Pi => {
                    self.next_token();
                    Ok(ASTNode::Pi)
                }
                Token::Euler => {
                    self.next_token();
                    Ok(ASTNode::Euler)
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
                                op : Token::Fact,
                                operand : Box::new(node),
                            };
                        }

                        Ok(node)
                    } else {
                        Err("Expected right parenthesis".to_string())
                    }
                }
                Token::Sin
                | Token::Cos
                | Token::Tg
                | Token::Cotg
                | Token::Log
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
                                    op : Token::Fact,
                                    operand : Box::new(node),
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
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Exponent => 3,
            _ => 0,
        }
    }
    fn parse_binary_op(&mut self, min_precedence: u8) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.current_token() {
            if op == &Token::Eof || op == &Token::RParen {
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
