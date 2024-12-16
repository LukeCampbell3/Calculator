pub fn to_rpn(tokens: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    let precedence = |op: &str| match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    };

    for token in tokens {
        if token.chars().all(|c| c.is_digit(10) || c == '.') {
            output.push(token);
        } else if token == "(" {
            operators.push(token);
        } else if token == ")" {
            while let Some(op) = operators.pop() {
                if op == "(" {
                    break;
                }
                output.push(op);
            }
        } else {
            while let Some(op) = operators.last() {
                if precedence(op) >= precedence(&token) {
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token);
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}
