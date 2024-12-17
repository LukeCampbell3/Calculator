mod tokenizer;
mod rpn;
mod tree;
mod dispatch_table;

use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut input = use_signal(|| String::new());
    let mut log = use_signal(|| Vec::<String>::new());
    let mut result = use_signal(|| None::<f64>);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; margin: 20px;",
            h1 { "Rust Calculator" }

            input {
                style: "width: 300px; padding: 10px; margin-bottom: 10px;",
                placeholder: "Enter your operation (e.g., 3 + 5 * (2 - 8))",
                value: "{input}",
                oninput: move |e| input.set(e.value().clone()),
            }

            button {
                style: "padding: 10px; margin-bottom: 20px;",
                onclick: move |_| {
                    // Tokenize the input
                    match tokenizer::tokenize(&input.read()) {
                        Ok(tokens) => {
                            // Convert to RPN
                            match rpn::to_rpn(tokens) {
                                Ok(rpn_tokens) => {
                                    let mut local_log = Vec::new();
            
                                    // Check if the input contains '=' or 'X' to solve for X
                                    if input.read().contains("=") || input.read().contains("X") {
                                        if let Some(x_value) = dispatch_table::solve_for_x(&rpn_tokens, &mut local_log) {
                                            result.set(Some(x_value));
                                            log.set(local_log);
                                        } else {
                                            log.set(vec!["Error: Could not solve for X".to_string()]);
                                        }
                                    } else {
                                        // Evaluate the RPN numerically
                                        if let Some(value) = dispatch_table::solve_rpn(&rpn_tokens, &mut local_log) {
                                            result.set(Some(value));
                                            log.set(local_log);
                                        } else {
                                            log.set(vec!["Error: Invalid RPN conversion".to_string()]);
                                        }
                                    }
                                }
                                Err(e) => log.set(vec![format!("Error: {}", e)]),
                            }
                        }
                        Err(e) => log.set(vec![format!("Error: {}", e)]), // Log tokenizer errors
                    }
                },
                "Calculate"
            }

            div {
                style: "width: 300px; max-height: 200px; overflow-y: auto; border: 1px solid #ccc; padding: 10px;",
                if let Some(res) = result.as_ref() {
                    p { "Result: {res}" }
                }
                h3 { "Steps:" }
                for step in log.iter() {
                    p { "{step}" }
                }
            }
        }
    }
}
