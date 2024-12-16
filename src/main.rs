mod tokenizer;
mod rpn;
mod tree;
mod dispatch_table;

use tokenizer::tokenize;
use rpn::to_rpn;
use tree::build_tree;
use dispatch_table::evaluate;

use std::collections::HashMap;
use std::io;


fn main() {

    println!("Enter the operation you want done...\n");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input :(");

    let input = input.trim();
    
    // Define the dispatch table explicitly
    let mut dispatch: HashMap<&str, Box<dyn Fn(f64, f64) -> f64>> = HashMap::new();
    dispatch.insert("+", Box::new(|a, b| a + b));
    dispatch.insert("-", Box::new(|a, b| a - b));
    dispatch.insert("*", Box::new(|a, b| a * b));
    dispatch.insert("/", Box::new(|a, b| a / b));

    // Tokenize input
    let tokens = tokenize(input);
    // Convert to RPN
    let rpn = to_rpn(tokens);
    // Build the expression tree
    let tree = build_tree(rpn);

    // Log the evaluation steps
    let mut log = Vec::new();
    // Evaluate the expression tree
    let result = evaluate(&tree, &dispatch, &mut log);

    // Print the result and steps
    println!("Result: {}", result);
    println!("Steps:");
    for step in log {
        println!("{}", step);
    }
}
