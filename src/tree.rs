use std::rc::Rc;


#[derive(Debug)]
pub enum ExprNode {
    Value(f64),
    Op(String, Rc<ExprNode>, Rc<ExprNode>),
}

pub fn build_tree(rpn: Vec<String>) -> Rc<ExprNode> {
    let mut stack: Vec<Rc<ExprNode>> = Vec::new();

    for token in rpn {
        if token.chars().all(|c| c.is_digit(10) || c == '.') {
            stack.push(Rc::new(ExprNode::Value(token.parse().unwrap())));
        } else {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            stack.push(Rc::new(ExprNode::Op(token, left, right))); // Corrected `toekn` to `token`
        }
    }

    stack.pop().unwrap() // Removed unnecessary semicolon to return value
}
