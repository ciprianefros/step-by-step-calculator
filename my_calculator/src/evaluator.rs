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
    pub fn evaluate_and_print(&mut self, mut ast: ASTNode) -> Result<f64, String> {
        let mut previous_step: Option<String> = None;
        while !Self::is_single_node(&ast) {
            let expression_string = Self::ast_to_string(&ast);
            if Some(&expression_string) != previous_step.as_ref() {
                println!("= {}", expression_string.clone());
                previous_step = Some(expression_string.clone());
                self.evaluation_steps
                    .push(format!("= {}", expression_string));
            }

            ast = Self::reduce_ast(ast)?;
        }

        if let ASTNode::Number(result) = ast {
            let truncated = Self::truncate_number(result);
            println!("= {}", truncated);
            self.evaluation_steps.push(format!("= {}", truncated));
            Ok(truncated)
        } else {
            Err("Evaluation did not reduce to a single number!".to_string())
        }
    }

    fn is_single_node(ast: &ASTNode) -> bool {
        matches!(ast, ASTNode::Number(_))
    }

    fn reduce_ast(ast: ASTNode) -> Result<ASTNode, String> {
        match ast {
            ASTNode::BinaryOp { left, op, right } => {
                if let ASTNode::Number(left_val) = *left {
                    if let ASTNode::Number(right_val) = *right {
                        let result = Self::evaluate_binary_op(left_val, op, right_val)?;
                        Ok(ASTNode::Number(result))
                    } else {
                        Ok(ASTNode::BinaryOp {
                            left: Box::new(ASTNode::Number(left_val)),
                            op,
                            right: Box::new(Self::reduce_ast(*right)?),
                        })
                    }
                } else {
                    Ok(ASTNode::BinaryOp {
                        left: Box::new(Self::reduce_ast(*left)?),
                        op,
                        right,
                    })
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                if let ASTNode::Number(operand_val) = *operand {
                    let result = Self::evaluate_unary_op(op, operand_val)?;
                    Ok(ASTNode::Number(result))
                } else {
                    let reduced_operand = Self::reduce_ast(*operand)?;
                    Ok(ASTNode::UnaryOp {
                        op,
                        operand: Box::new(reduced_operand),
                    })
                }
            }
            ASTNode::Function { func, argument } => {
                if let ASTNode::Number(arg_val) = *argument {
                    let result = Self::evaluate_function(func, arg_val)?;
                    Ok(ASTNode::Number(result))
                } else {
                    let reduced_argument = Self::reduce_ast(*argument)?;
                    Ok(ASTNode::Function {
                        func,
                        argument: Box::new(reduced_argument),
                    })
                }
            }
            ASTNode::LogBase { base, number } => {
                let reduced_base = Self::reduce_ast(*base)?;
                let reduced_number = Self::reduce_ast(*number)?;

                match (reduced_base, reduced_number) {
                    (ASTNode::Number(base_val), ASTNode::Number(number_val)) => {
                        let result = Self::evaluate_log_base(base_val, number_val)?;
                        Ok(ASTNode::Number(result))
                    }
                    (reduced_base, reduced_number) => Ok(ASTNode::LogBase {
                        base: Box::new(reduced_base),
                        number: Box::new(reduced_number),
                    }),
                }
            }
            ASTNode::Grouping(expression) => {
                let reduced_expression = Self::reduce_ast(*expression)?;
                if let ASTNode::Number(_) = reduced_expression {
                    Ok(reduced_expression)
                } else {
                    Ok(ASTNode::Grouping(Box::new(reduced_expression)))
                }
            }
            ASTNode::Pi => Ok(ASTNode::Number(Self::truncate_number(PI))),
            ASTNode::Euler => Ok(ASTNode::Number(Self::truncate_number(E))),
            _ => Ok(ast),
        }
    }

    fn evaluate_binary_op(left: f64, op: Token, right: f64) -> Result<f64, String> {
        match op {
            Token::Plus => Ok(left + right),
            Token::Minus => Ok(left - right),
            Token::Multiply => Ok(left * right),
            Token::Divide => {
                if right == 0.0 {
                    Err("Can't divide number by 0".to_string())
                } else {
                    Ok(left / right)
                }
            }
            Token::Exponent => Ok(left.powf(right)),
            _ => Err("Unknown binary operator".to_string()),
        }
    }

    fn evaluate_unary_op(op: Token, operand: f64) -> Result<f64, String> {
        match op {
            Token::Minus => Ok(-operand),
            Token::Fact => {
                if operand != operand.floor() || operand < 0.0 {
                    return Err("Factorial is only defined for non-negative integers!".to_string());
                }
                let n = operand as u64;
                if n == 0 {
                    Ok(1.0)
                } else {
                    Ok((1..=n).map(|x| x as f64).product())
                }
            }
            _ => Err("Unknown unary operator".to_string()),
        }
    }
    fn evaluate_log_base(base: f64, number: f64) -> Result<f64, String> {
        if base <= 0.0 {
            return Err("The base of logarithm must be greater than zero!".to_string());
        }

        if (base - 1.0).abs() <= f64::EPSILON {
            return Err("The base of logarithm cannot be 1!".to_string());
        }

        if number <= 0.0 {
            return Err("Can't calculate logarithm of negative number!".to_string());
        }

        Ok(number.ln() / base.ln())
    }

    fn evaluate_function(func: Token, arg: f64) -> Result<f64, String> {
        match func {
            Token::Abs => Ok(arg.abs()),
            Token::Sqrt => {
                if arg < 0.0 {
                    Err("Can't calculate square root of negative number!".to_string())
                } else {
                    Ok(arg.sqrt())
                }
            }
            Token::Ln => {
                if arg <= 0.0 {
                    Err("Can't calculate logarithm of negative number!".to_string())
                } else {
                    Ok(arg.ln())
                }
            }
            Token::Sin => Ok(Self::truncate_number(arg.to_radians().sin())),
            Token::Cos => Ok(Self::truncate_number(arg.to_radians().cos())),
            Token::Tg => {
                let radians = arg.to_radians();

                if (radians / (PI / 2.0)).rem_euclid(2.0).abs() < 1e-10 {
                    Err("Can't calculate tg for that number, cosine is 0!".to_string())
                } else {
                    Ok(Self::truncate_number(radians.tan()))
                }
            }
            Token::Cotg => {
                let radians = arg.to_radians();

                if (radians / (PI)).rem_euclid(1.0).abs() < 1e-10 {
                    Err("Can't calculate cotg for that number, it is 0!".to_string())
                } else {
                    Ok(Self::truncate_number(1.0 / radians.tan()))
                }
            }
            Token::Sec => {
                let radians = arg.to_radians();
                if radians.cos().abs() < 1e-10 {
                    Err("Can't calculate sec for that number, cosine is 0!".to_string())
                } else {
                    Ok(Self::truncate_number(1.0 / radians.cos()))
                }
            }
            Token::Csc => {
                let radians = arg.to_radians();
                if radians.sin().abs() < 1e-10 {
                    Err("Can't calculate csc for that number, sine is 0!".to_string())
                } else {
                    Ok(Self::truncate_number(1.0 / radians.sin()))
                }
            }
            Token::Asin => {
                if !(-1.0..=1.0).contains(&arg) {
                    Err("Can't calculate asin for values outside of [-1, 1]".to_string())
                } else {
                    Ok(Self::truncate_number(arg.asin()))
                }
            }
            Token::Acos => {
                if !(-1.0..=1.0).contains(&arg) {
                    Err("Can't calculate acos for values outside of [-1, 1]".to_string())
                } else {
                    Ok(Self::truncate_number(arg.acos()))
                }
            }
            Token::Atg => Ok(Self::truncate_number(arg.atan())),
            Token::Actg => {
                if arg == 0.0 {
                    Err("Can't calculate actg for 0!".to_string())
                } else {
                    Ok(Self::truncate_number((PI / 2.0) - arg.atan()))
                }
            }
            _ => Err("Unknown function".to_string()),
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
                    _ => "Unknown binary operator",
                };
                format!("{} {} {}", left_str, op_str, right_str)
            }
            ASTNode::UnaryOp { op, operand } => {
                let operand_str = Self::ast_to_string(operand);
                match op {
                    Token::Minus => format!("-{}", operand_str),
                    Token::Fact => format!("{}!", operand_str),
                    _ => "Unknown unary operator".to_string(),
                }
            }
            ASTNode::Function { func, argument } => {
                let arg_str = Self::ast_to_string(argument);
                let func_str = match func {
                    Token::Abs => "abs",
                    Token::Sqrt => "sqrt",
                    Token::Ln => "ln",
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
                    _ => "Unknown function",
                };
                format!("{}({})", func_str, arg_str)
            }
            ASTNode::LogBase { base, number } => {
                let base_str = Self::ast_to_string(base);
                let number_str = Self::ast_to_string(number);

                format!("log({},{})", base_str, number_str)
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
        let result = evaluator.evaluate_and_print(ast).unwrap();
        assert_eq!(result, 8.0);

        let ast = ASTNode::BinaryOp {
            left: Box::new(ASTNode::Number(5.0)),
            op: Token::Multiply,
            right: Box::new(ASTNode::Number(3.0)),
        };
        let result = evaluator.evaluate_and_print(ast).unwrap();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_trigonometric_functions() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::Function {
            func: Token::Sin,
            argument: Box::new(ASTNode::Number(30.0)), // sin(30°) = 0.5
        };
        let result = evaluator.evaluate_and_print(ast).unwrap();
        assert_eq!(result, 0.5);

        let ast = ASTNode::Function {
            func: Token::Cos,
            argument: Box::new(ASTNode::Number(60.0)), // cos(60°) = 0.5
        };
        let result = evaluator.evaluate_and_print(ast).unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_unary_operations() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::UnaryOp {
            op: Token::Minus,
            operand: Box::new(ASTNode::Number(7.0)),
        };
        assert_eq!(evaluator.evaluate_and_print(ast).unwrap(), -7.0);

        let ast = ASTNode::UnaryOp {
            op: Token::Fact,
            operand: Box::new(ASTNode::Number(5.0)),
        };
        assert_eq!(evaluator.evaluate_and_print(ast).unwrap(), 120.0);
    }

    #[test]
    fn test_constants() {
        let mut evaluator = Evaluator::new();

        let ast = ASTNode::Pi;
        assert_eq!(evaluator.evaluate_and_print(ast).unwrap(), 3.14);

        let ast = ASTNode::Euler;
        assert_eq!(evaluator.evaluate_and_print(ast).unwrap(), 2.72);
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
        assert_eq!(evaluator.evaluate_and_print(ast).unwrap(), 11.0);
    }
    #[test]
    fn test_edge_case_trigonometric() {
        let mut evaluator = Evaluator::new();

        //test pentru tg unde a aprope de infint
        let ast = ASTNode::Function {
            func: Token::Tg,
            argument: Box::new(ASTNode::Number(89.999)),
        };
        let result = evaluator.evaluate_and_print(ast).unwrap();
        assert!(result.is_finite());

        // Test pentru cotg unde este aproape 0
        let ast = ASTNode::Function {
            func: Token::Cotg,
            argument: Box::new(ASTNode::Number(179.999)),
        };
        let result = evaluator.evaluate_and_print(ast).unwrap();
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
        let result = evaluator.evaluate_and_print(ast);
        match result {
            Ok(_) => print!("Didn't panic!"),
            Err(e) => panic!("{}", e),
        }
    }
}
