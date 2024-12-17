use crate::tokenizer::Token;

pub fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();       // Final RPN output
    let mut operators = Vec::new();    // Operator stack

    for token in tokens {
        match token {
            Token::Number(_) | Token::Variable(_) => {
                output.push(token); // Push numbers and variables directly to the output
            }
            Token::Operator(op) => {
                while let Some(Token::Operator(top)) = operators.last() {
                    if precedence(*top) >= precedence(op) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(Token::Operator(op));
            }
            Token::LeftParen => {
                operators.push(Token::LeftParen);
            }
            Token::RightParen => {
                while let Some(top) = operators.pop() {
                    if top == Token::LeftParen {
                        break;
                    }
                    output.push(top);
                }
            }
            Token::Equals => {
                // Optional: Log when Equals is encountered, but don't require it
                while let Some(op) = operators.pop() {
                    output.push(op);
                }
                output.push(Token::Equals);
            }
            _ => {
                return Err(format!("Unexpected token: {:?}", token));
            }
        }
    }

    // Flush any remaining operators to the output
    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}


fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}
