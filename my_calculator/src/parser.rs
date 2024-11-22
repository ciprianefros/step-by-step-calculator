use crate::lexer::Token;

pub enum ASTNode {
    Number(f64),
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
        if let Some(Token::EOF) = self.current_token() {
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
                    self.next_token(); // Consume the number
                    Ok(ASTNode::Number(value))
                }
                Token::Minus => {
                    self.next_token(); // Consume the minus
                    let operand = self.parse_primary()?; // Parse the operand
                    Ok(ASTNode::UnaryOp {
                        op: Token::Minus,
                        operand: Box::new(operand),
                    })
                }
                Token::LParen => {
                    self.next_token(); // Consume '('
                    let expr = self.parse_inner_expression()?; // Parse the inner expression
                    if let Some(Token::RParen) = self.current_token() {
                        self.next_token(); // Consume ')'
                        Ok(expr)
                    } else {
                        Err("Expected right parenthesis".to_string())
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
            if op == &Token::EOF || op == &Token::RParen {
                break; 
            }
    
            let precedence = Parser::get_precedence(op);
            if precedence < min_precedence {
                break; // Stop parsing if operator precedence is too low
            }
    
            let op = self.current_token().unwrap().clone();
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
