use crate::lexer::Token;
use crate::parser::ASTNode;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate_and_print(mut ast: ASTNode) -> f64 {
        while !Self::is_single_node(&ast) {
            let expression_string = Self::ast_to_string(&ast);
            println!("= {}", expression_string);

            ast = Self::reduce_ast(ast);
        }

        if let ASTNode::Number(result) = ast {
            println!("= {}", result);
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
            Token::Sqrt => arg.sqrt(),
            Token::Log => arg.log10(),
            Token::Sin => arg.to_radians().sin(),
            Token::Cos => arg.to_radians().cos(),
            Token::Tg => arg.to_radians().tan(),
            Token::Cotg => 1.0 / arg.to_radians().tan(),
            _ => panic!("Unknown function"),
        }
    }
    fn ast_to_string(ast: &ASTNode) -> String {
        match ast {
            ASTNode::Number(value) => format!("{}", value),
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
            }
        }
    }
}