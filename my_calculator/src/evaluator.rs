use crate::lexer::Token;
use crate::parser::ASTNode;
use std::f64::consts::{E, PI};

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
                self.evaluation_steps
                    .push(format!("= {}", expression_string));
            }

            ast = Self::reduce_ast(ast);
        }

        if let ASTNode::Number(result) = ast {
            println!("= {}", Self::truncate_number(result));
            self.evaluation_steps.push(format!("= {}", Self::truncate_number(result)));
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
                let reduced_expression = Self::reduce_ast(*expression);
                if let ASTNode::Number(_) = reduced_expression {
                    reduced_expression
                } else {
                    ASTNode::Grouping(Box::new(reduced_expression))
                }
            }
            ASTNode::Pi => ASTNode::Number(Self::truncate_number(PI)),
            ASTNode::Euler => ASTNode::Number(Self::truncate_number(E)),
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
            Token::Fact => {
                if operand != operand.floor() || operand < 0.0 {
                    panic!("Factorial is only defined for non-negative integers!");
                }
                let n = operand as u64;
                if n == 0 {
                    1.0
                } else {
                    (1..=n).map(|x| x as f64).product()
                }
            }
            _ => panic!("Unknown unary operator"),
        }
    }

    fn evaluate_function(func: Token, arg: f64) -> f64 {
        match func {
            Token::Abs => arg.abs(),
            Token::Sqrt => {
                if arg < 0.0 {
                    panic!("Can't calculate square root of negative number!");
                } else {
                    arg.sqrt()
                }
            }
            Token::Log => {
                if arg < 1.0 {
                    panic!("Can't calculate logarithm of negative number!");
                } else {
                    arg.log10()
                }
            }
            Token::Sin => Self::truncate_number(arg.to_radians().sin()),
            Token::Cos => Self::truncate_number(arg.to_radians().cos()),
            Token::Tg => {
                let radians = arg.to_radians();

                if (radians / (PI / 2.0)).rem_euclid(2.0).abs() < 1e-10 {
                    panic!("Can't calculate tg for that number, cosine is 0!");
                } else {
                    Self::truncate_number(radians.tan())
                }
            }
            Token::Cotg => {
                let radians = arg.to_radians();

                if (radians / (PI)).rem_euclid(1.0).abs() < 1e-10 {
                    panic!("Can't calculate cotg for that number, it is 0!");
                } else {
                    Self::truncate_number(1.0 / radians.tan())
                }
            }
            Token::Sec => {
                let radians = arg.to_radians();
                if radians.cos().abs() < 1e-10 {
                    panic!("Can't calculate sec for that number, cosine is 0!");
                } else {
                    Self::truncate_number(1.0 / radians.cos())
                }
            }
            Token::Csc => {
                let radians = arg.to_radians();
                if radians.sin().abs() < 1e-10 {
                    panic!("Can't calculate csc for that number, sine is 0!");
                } else {
                    Self::truncate_number(1.0 / radians.sin())
                }
            }
            Token::Asin => {
                if !(-1.0..=1.0).contains(&arg) {
                    panic!("Can't calculate asin for values outside of [-1, 1]");
                } else {
                    Self::truncate_number(arg.asin())
                }
            }
            Token::Acos => {
                if !(-1.0..=1.0).contains(&arg) {
                    panic!("Can't calculate acos for values outside of [-1, 1]");
                } else {
                    Self::truncate_number(arg.acos())
                }
            }
            Token::Atg => Self::truncate_number(arg.atan()),
            Token::Actg => {
                if arg == 0.0 {
                    panic!("Can't calculate actg for 0!");
                } else {
                    Self::truncate_number((PI / 2.0) - arg.atan())
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
                    Token::Exponent => "^",
                    _ => panic!("Unknown binary operator"),
                };
                format!("{} {} {}", left_str, op_str, right_str)
            }
            ASTNode::UnaryOp { op, operand } => {
                let operand_str = Self::ast_to_string(operand);
                match op {
                    Token::Minus => format!("-{}", operand_str),
                    Token::Fact => format!("{}!", operand_str),
                    _ => panic!("Unknown unary operator"),
                }
            }
            ASTNode::Function { func, argument } => {
                let arg_str = Self::ast_to_string(argument);
                let func_str = match func {
                    Token::Abs => "abs",
                    Token::Sqrt => "sqrt",
                    Token::Log => "log",
                    Token::Sin => "sin",
                    Token::Cos => "cos",
                    Token::Tg => "tg",
                    Token::Cotg => "cotg",
                    Token::Sec => "sec",
                    Token::Csc => "csc",
                    Token::Asin => "asin",
                    Token::Acos => "acos",
                    Token::Atg => "atg",
                    Token::Actg => "actg",
                    _ => panic!("Unknown function"),
                };
                format!("{}({:.2})", func_str, arg_str)
            }
            ASTNode::Grouping(expression) => {
                format!("({})", Self::ast_to_string(expression))
            }
        }
    }
    fn truncate_number(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;
    use crate::parser::ASTNode;

    #[test]
    fn test_basic_arithmetic() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Number(5.0)),
            op: Token::Plus,
            right: Box::new(ASTNode::Number(3.0)),
        };
        assert_eq!(evaluator.evaluate_and_print(ast), 8.0);

        let ast = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Number(5.0)),
            op: Token::Multiply,
            right: Box::new(ASTNode::Number(3.0)),
        };
        assert_eq!(evaluator.evaluate_and_print(ast), 15.0);
    }

    #[test]
    fn test_trigonometric_functions() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::Function {
            func: Token::Sin,
            argument: Box::new(ASTNode::Number(30.0)), // sin(30°) = 0.5
        };
        assert_eq!(evaluator.evaluate_and_print(ast), 0.5);

        let ast = ASTNode::Function {
            func: Token::Cos,
            argument: Box::new(ASTNode::Number(60.0)), // cos(60°) = 0.5
        };
        assert_eq!(evaluator.evaluate_and_print(ast), 0.5);
    }

    #[test]
    fn test_unary_operations() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::UnaryOp {
            op: Token::Minus,
            operand: Box::new(ASTNode::Number(7.0)),
        };
        assert_eq!(evaluator.evaluate_and_print(ast), -7.0);

        let ast = ASTNode::UnaryOp {
            op: Token::Fact,
            operand: Box::new(ASTNode::Number(5.0)),
        };
        assert_eq!(evaluator.evaluate_and_print(ast), 120.0);
    }

    #[test]
    fn test_constants() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::Pi;
        assert_eq!(evaluator.evaluate_and_print(ast), 3.14);

        let ast = ASTNode::Euler;
        assert_eq!(evaluator.evaluate_and_print(ast), 2.72);
    }

    #[test]
    fn test_nested_expressions() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Number(3.0)),
            op: Token::Plus,
            right: Box::new(ASTNode::BinaryOp {
                left: Box::new(ASTNode::Number(4.0)),
                op: Token::Multiply,
                right: Box::new(ASTNode::Number(2.0)),
            }),
        };
        // 3 + (4 * 2) = 3 + 8 = 11
        assert_eq!(evaluator.evaluate_and_print(ast), 11.0);
    }
    #[test]
    fn test_edge_case_trigonometric() {
        let mut evaluator = Evaluator::new();

        //test pentru tg unde a aprope de infint
        let ast = ASTNode::Function {
            func: Token::Tg,
            argument: Box::new(ASTNode::Number(89.999)), 
        };
        let result = evaluator.evaluate_and_print(ast);
        assert!(result.is_finite());

        // Test pentru cotg unde este aproape 0
        let ast = ASTNode::Function {
            func: Token::Cotg,
            argument: Box::new(ASTNode::Number(179.999)), 
        };
        let result = evaluator.evaluate_and_print(ast);
        assert!(result.is_finite());
    }
    #[test]
    #[should_panic(expected = "Can't divide number by 0")]
    fn test_division_by_zero() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Number(5.0)),
            op: Token::Divide,
            right: Box::new(ASTNode::Number(0.0)),
        };
        evaluator.evaluate_and_print(ast);
}
}
