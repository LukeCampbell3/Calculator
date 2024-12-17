use std::rc::Rc;

#[derive(Debug)]
pub enum ExprNode {
    Value(f64),
    UnaryOp(String, Rc<ExprNode>),
    BinaryOp(String, Rc<ExprNode>, Rc<ExprNode>),
}

pub fn build_tree(rpn: Vec<String>) -> Result<Rc<ExprNode>, String> {
    let mut stack: Vec<Rc<ExprNode>> = Vec::new();

    for token in rpn {
        if let Ok(value) = token.parse::<f64>() {
            stack.push(Rc::new(ExprNode::Value(value)));
        } else if token == "π" {
            stack.push(Rc::new(ExprNode::Value(std::f64::consts::PI)));
        } else if ["sin", "cos", "tan"].contains(&token.as_str()) {
            let arg = stack.pop().ok_or(format!("Missing argument for '{}'", token))?;
            stack.push(Rc::new(ExprNode::UnaryOp(token, arg)));
        } else if ["+", "-", "*", "/", "^"].contains(&token.as_str()) {
            let right = stack.pop().ok_or(format!("Missing right operand for '{}'", token))?;
            let left = stack.pop().ok_or(format!("Missing left operand for '{}'", token))?;
            stack.push(Rc::new(ExprNode::BinaryOp(token, left, right)));
        } else {
            return Err(format!("Unknown token: {}", token));
        }
    }

    stack.pop().ok_or("Failed to build tree: Stack is empty".to_string())
}
