#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
    Variable(String), // Add a variable token
    Equals
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse::<f64>().unwrap()));
            }
            '+' | '-' | '*' | '/' | '^' => {
                tokens.push(Token::Operator(c));
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            'X' | 'x' => {  // Support 'X' or 'x' as the variable
                tokens.push(Token::Variable("X".to_string()));
                chars.next();
            }
            ' ' => { chars.next(); } // Skip whitespace
            _ => panic!("Unexpected character: {}", c),
        }
    }

    Ok(tokens)
}
