pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = input.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_digit(10) || c == '.' {
            current.push(c);
        } else {
            // Push the current number (if any) as a token
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }

            if c == '(' {
                // Check for implicit multiplication before an opening parenthesis
                if let Some(last_token) = tokens.last() {
                    if last_token.chars().last().unwrap().is_digit(10) || last_token == ")" {
                        tokens.push("*".to_string());
                    }
                }
                tokens.push("(".to_string());
            } else if c == ')' {
                tokens.push(")".to_string());

                // Check for implicit multiplication after a closing parenthesis
                if i + 1 < chars.len() && chars[i + 1].is_digit(10) {
                    tokens.push("*".to_string());
                }
            } else if c.is_whitespace() {
                continue; // Skip spaces
            } else {
                tokens.push(c.to_string());
            }
        }
    }

    // Push the last token if it exists
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
