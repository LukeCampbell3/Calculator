use crate::tree::ExprNode;
use crate::tokenizer::Token;

pub fn evaluate(node: &ExprNode, log: &mut Vec<String>) -> f64 {
    match node {
        ExprNode::Value(val) => *val,
        ExprNode::UnaryOp(op, arg) => {
            let val = evaluate(arg, log);
            let result = match op.as_str() {
                "sin" => val.sin(),
                "cos" => val.cos(),
                "tan" => val.tan(),
                _ => panic!("Unknown unary operator: {}", op),
            };
            log.push(format!("{}({}) = {}", op, val, result));
            result
        }
        ExprNode::BinaryOp(op, left, right) => {
            let left_val = evaluate(left, log);
            let right_val = evaluate(right, log);
            let result = match op.as_str() {
                "+" => left_val + right_val,
                "-" => left_val - right_val,
                "*" => left_val * right_val,
                "/" => left_val / right_val,
                "^" => left_val.powf(right_val),
                _ => panic!("Unknown binary operator: {}", op),
            };
            log.push(format!("{} {} {} = {}", left_val, op, right_val, result));
            result
        }
    }
}

pub fn solve_rpn(rpn: &[Token], log: &mut Vec<String>) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new(); // Numeric stack for evaluation
    let mut x_found = false; // Track if X is in the equation

    for token in rpn {
        match token {
            Token::Number(n) => {
                stack.push(*n);
                log.push(format!("Push {}", n));
            }
            Token::Variable(_) => {
                x_found = true; // X detected
                stack.push(1.0); // Temporary placeholder for X
                log.push("Push X".to_string());
            }
            Token::Operator(op) => {
                if stack.len() < 2 {
                    log.push(format!("Error: Not enough operands for '{}'", op));
                    return None;
                }

                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match *op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 {
                            log.push("Error: Division by zero".to_string());
                            return None;
                        }
                        a / b
                    }
                    '^' => a.powf(b),
                    _ => {
                        log.push(format!("Error: Unsupported operator '{}'", op));
                        return None;
                    }
                };
                stack.push(result);
                log.push(format!("{} {} {} = {}", a, op, b, result));
            }
            Token::Equals => {
                // Ignore the Equals token for numeric evaluation
                log.push("Found '=' - splitting equation".to_string());
            }
            _ => log.push(format!("Error: Unexpected token {:?}", token)),
        }
    }

    // If X was found, return None to signal solving for X; otherwise return the result
    if x_found {
        log.push("X found, solving for X".to_string());
        None
    } else {
        stack.pop()
    }
}


pub fn solve_for_x(rpn: &[Token], log: &mut Vec<String>) -> Option<f64> {
    let mut lhs_stack: Vec<(f64, f64)> = Vec::new(); // (X coefficient, constant value)
    let mut rhs_value = 0.0; // Right-hand side constant
    let mut on_rhs = false; // Track RHS processing

    for token in rpn {
        match token {
            Token::Number(n) => {
                if on_rhs {
                    rhs_value += *n;
                    log.push(format!("RHS += {}", n)); // Log step
                } else {
                    lhs_stack.push((0.0, *n));
                    log.push(format!("Push constant {}", n)); // Log step
                }
            }
            Token::Variable(_) => {
                if on_rhs {
                    log.push("Error: Variable X found on RHS".to_string());
                    return None;
                }
                lhs_stack.push((1.0, 0.0));
                log.push("Push X".to_string()); // Log step
            }
            Token::Operator(op) => {
                if lhs_stack.len() < 2 {
                    log.push(format!("Error: Not enough operands for '{}'", op));
                    return None;
                }

                let (x2, c2) = lhs_stack.pop().unwrap();
                let (x1, c1) = lhs_stack.pop().unwrap();

                match *op {
                    '+' => {
                        lhs_stack.push((x1 + x2, c1 + c2));
                        log.push(format!("Add: ({}X + {}) + ({}X + {})", x1, c1, x2, c2));
                    }
                    '-' => {
                        lhs_stack.push((x1 - x2, c1 - c2));
                        log.push(format!("Subtract: ({}X + {}) - ({}X + {})", x1, c1, x2, c2));
                    }
                    '*' => {
                        if x1 != 0.0 && x2 != 0.0 {
                            log.push("Error: Cannot multiply two X terms".to_string());
                            return None;
                        } else if x1 != 0.0 {
                            lhs_stack.push((x1 * c2, 0.0));
                            log.push(format!("Multiply X by {}", c2));
                        } else if x2 != 0.0 {
                            lhs_stack.push((x2 * c1, 0.0));
                            log.push(format!("Multiply X by {}", c1));
                        } else {
                            lhs_stack.push((0.0, c1 * c2));
                            log.push(format!("Multiply constants: {} * {}", c1, c2));
                        }
                    }
                    '/' => {
                        if x2 != 0.0 {
                            log.push("Error: Cannot divide by X".to_string());
                            return None;
                        }
                        lhs_stack.push((x1 / c2, c1 / c2));
                        log.push(format!("Divide: ({}X + {}) / {}", x1, c1, c2));
                    }
                    _ => {
                        log.push(format!("Error: Unsupported operator '{}'", op));
                        return None;
                    }
                }
            }
            Token::Equals => {
                if let Some((x_coeff, constant)) = lhs_stack.pop() {
                    rhs_value -= constant; // Move constant to RHS
                    lhs_stack.push((x_coeff, 0.0));
                    log.push(format!("Move constant {} to RHS", constant));
                }
                on_rhs = true;
                log.push("Switch to RHS".to_string());
            }
            _ => log.push(format!("Error: Unexpected token {:?}", token)),
        }
    }

    if let Some((x_coeff, constant_sum)) = lhs_stack.pop() {
        if x_coeff == 0.0 {
            log.push("Error: No X term found or coefficient is zero".to_string());
            return None;
        }
        let result = (rhs_value - constant_sum) / x_coeff;
        log.push(format!("Solve: X = ({} - {}) / {}", rhs_value, constant_sum, x_coeff));
        return Some(result);
    }

    None
}


