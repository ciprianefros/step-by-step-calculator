use crate::lexer::Token;
use crate::parser::ASTNode;
use std::f64::consts::{PI, E};


#[derive(Clone, Debug)]
pub struct Evaluator {
    evaluation_steps: Vec<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            evaluation_steps: Vec::new(),
        }
    }
    pub fn get_evaluation_steps(self) -> Vec<String> {
        self.evaluation_steps.clone()
    }
    pub fn evaluate_and_print(&mut self, mut ast: ASTNode) -> f64 {
        let mut previous_step: Option<String> = None;
        while !Self::is_single_node(&ast) {
            let expression_string = Self::ast_to_string(&ast);
            if Some(&expression_string) != previous_step.as_ref() {
                println!("= {}", expression_string.clone());
                previous_step = Some(expression_string.clone());
                self.evaluation_steps.push(format!("= {}",expression_string));
            }

            ast = Self::reduce_ast(ast);
        }

        if let ASTNode::Number(result) = ast {
            println!("= {:.2}", result);
            self.evaluation_steps.push(format!("= {:.2}", result));
            result
        } else {
            panic!("Evaluation did not reduce to a single number!");
        }
    }

    fn is_single_node(ast: &ASTNode) -> bool {
        matches!(ast, ASTNode::Number(_))
    }

    fn reduce_ast(ast: ASTNode) -> ASTNode {
        match ast {
            ASTNode::BinaryOp { left, op, right } => {
                if let ASTNode::Number(left_val) = *left {
                    if let ASTNode::Number(right_val) = *right {
                        let result = Self::evaluate_binary_op(left_val, op, right_val);
                        ASTNode::Number(result)
                    } else {
                        ASTNode::BinaryOp {
                            left: Box::new(ASTNode::Number(left_val)),
                            op,
                            right: Box::new(Self::reduce_ast(*right)),
                        }
                    }
                } else {
                    ASTNode::BinaryOp {
                        left: Box::new(Self::reduce_ast(*left)),
                        op,
                        right,
                    }
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                if let ASTNode::Number(operand_val) = *operand {
                    let result = Self::evaluate_unary_op(op, operand_val);
                    ASTNode::Number(result)
                } else {
                    ASTNode::UnaryOp {
                        op,
                        operand: Box::new(Self::reduce_ast(*operand)),
                    }
                }
            }
            ASTNode::Function { func, argument } => {
                if let ASTNode::Number(arg_val) = *argument {
                    let result = Self::evaluate_function(func, arg_val);
                    ASTNode::Number(result)
                } else {
                    ASTNode::Function {
                        func,
                        argument: Box::new(Self::reduce_ast(*argument)),
                    }
                }
            }
            ASTNode::Grouping(expression) => {
                Self::reduce_ast(*expression)
            }
            ASTNode::Pi => ASTNode::Number(PI),
            ASTNode::Euler => ASTNode::Number(E),
            _ => ast,
        }
    }

    fn evaluate_binary_op(left: f64, op: Token, right: f64) -> f64 {
        match op {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Multiply => left * right,
            Token::Divide => {
                if right == 0.0 {
                    panic!("Can't divide number by 0")
                } else {
                    left / right
                }
            }
            Token::Exponent => left.powf(right),
            _ => panic!("Unknown binary operator"),
        }
    }

    fn evaluate_unary_op(op: Token, operand: f64) -> f64 {
        match op {
            Token::Minus => -operand,
            _ => panic!("Unknown unary operator"),
        }
    }

    fn evaluate_function(func: Token, arg: f64) -> f64 {
        match func {
            Token::Sqrt => {
                if arg < 0.0 {
                    panic!("Can't calculate square root of negative number!");
                }
                else {
                    arg.sqrt()
                }
            }
            Token::Log => {
                if arg < 0.0 {
                    panic!("Can't calculate logarithm of negative number!");
                } else {
                    arg.log10()
                } 
            }
            Token::Sin => arg.to_radians().sin(),
            Token::Cos => arg.to_radians().cos(),
            Token::Tg => {
                let radians = arg.to_radians();

                if (radians / (PI / 2.0)).rem_euclid(2.0).abs() < 1e-10 {
                    panic!("Can't calculate tg for that number!");
                } else {
                    radians.tan()
                }
            }
            Token::Cotg => {
                let radians = arg.to_radians();

                if (radians / (PI)).rem_euclid(1.0).abs() < 1e-10 {
                    panic!("Can't calculate cotg for that number!");
                } else {
                    1.0 / radians.tan()
                }
            }
            _ => panic!("Unknown function"),
        }
    }
    fn ast_to_string(ast: &ASTNode) -> String {
        match ast {
            ASTNode::Number(value) => format!("{}", value),
            ASTNode::Pi => "π".to_string(),
            ASTNode::Euler => "e".to_string(),
            ASTNode::BinaryOp { left, op, right } => {
                let left_str = Self::ast_to_string(left);
                let right_str = Self::ast_to_string(right);
                let op_str = match op {
                    Token::Plus => "+",
                    Token::Minus => "-",
                    Token::Multiply => "*",
                    Token::Divide => "/",
                    Token::Exponent => "^^",
                    _ => panic!("Unknown binary operator"),
                };
                format!("{} {} {}", left_str, op_str, right_str)
            }
            ASTNode::UnaryOp { op, operand } => {
                let operand_str = Self::ast_to_string(operand);
                match op {
                    Token::Minus => format!("-{}", operand_str),
                    _ => panic!("Unknown unary operator"),
                }
            }
            ASTNode::Function { func, argument } => {
                let arg_str = Self::ast_to_string(argument);
                let func_str = match func {
                    Token::Sqrt => "sqrt",
                    Token::Log => "log",
                    Token::Sin => "sin",
                    Token::Cos => "cos",
                    Token::Tg => "tg",
                    Token::Cotg => "cotg",
                    _ => panic!("Unknown function"),
                };
                format!("{}({})", func_str, arg_str)
            },
            ASTNode::Grouping(expression) => {
                format!("({})", Self::ast_to_string(expression))
            }
        }
    }
}