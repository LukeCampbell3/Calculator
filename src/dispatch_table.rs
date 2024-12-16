use std::collections::HashMap;
use crate::tree::ExprNode;

// Updated evaluate function to use Box<dyn Fn>
pub fn evaluate(
    node: &ExprNode,
    dispatch: &HashMap<&str, Box<dyn Fn(f64, f64) -> f64>>,
    log: &mut Vec<String>,
) -> f64 {
    match node {
        ExprNode::Value(val) => *val,
        ExprNode::Op(op, left, right) => {
            let left_val = evaluate(left, dispatch, log);
            let right_val = evaluate(right, dispatch, log);

            if let Some(func) = dispatch.get(op.as_str()) {
                let result = func(left_val, right_val);
                log.push(format!("{} {} {} = {}", left_val, op, right_val, result));
                result
            } else {
                panic!("Unknown operator: {}", op);
            }
        }
    }
}
