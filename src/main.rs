mod tokenizer;
mod rpn;
mod tree;
mod dispatch_table;

use dioxus::prelude::*;
use std::collections::HashMap;

fn main() {
    // Launch the Dioxus app
    launch(app);
}

// Define the Dioxus app as a component with no parameters
fn app() -> Element {
    // State for user input and evaluation log
    let mut input = use_signal(|| "".to_string());
    let mut log = use_signal(|| Vec::new());
    let mut result = use_signal(|| None);

    // Define the dispatch table
    let mut dispatch: HashMap<&str, Box<dyn Fn(f64, f64) -> f64>> = HashMap::new();
    dispatch.insert("+", Box::new(|a, b| a + b));
    dispatch.insert("-", Box::new(|a, b| a - b));
    dispatch.insert("*", Box::new(|a, b| a * b));
    dispatch.insert("/", Box::new(|a, b| a / b));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; margin: 20px;",
            h1 { "Rust Calculator" }
    
            // Input field
            input {
                style: "width: 300px; padding: 10px; margin-bottom: 10px;",
                placeholder: "Enter your operation (e.g., 3 + 5 * (2 - 8))",
                value: "{input.read()}",
                oninput: move |e| input.set(e.value().clone()), // Safely update input signal
            }
    
            // Calculate button
            button {
                style: "padding: 10px; margin-bottom: 20px;",
                onclick: move |_| {
                    let tokens = tokenizer::tokenize(&input.read()); // Read input signal
                    let rpn = rpn::to_rpn(tokens);
                    let tree = tree::build_tree(rpn);
    
                    let mut local_log = Vec::new();
                    let local_result = dispatch_table::evaluate(&tree, &dispatch, &mut local_log);
    
                    result.set(Some(local_result)); // Update result
                    log.set(local_log);             // Update log
                },
                "Calculate"
            }
    
            // Result and Steps Display
            div {
                style: "width: 300px; max-height: 200px; overflow-y: auto; border: 1px solid #ccc; padding: 10px;",
                if let Some(res) = result.read().as_ref() {
                    p { "{res}" } // Interpolate result value
                }
                h3 { "Steps:" }
                for step in log.read().iter() {
                    p { "{step}" } // Interpolate log step
                }
            }
        }
    }
    
}
