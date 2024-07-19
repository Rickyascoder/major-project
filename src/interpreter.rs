use std::collections::HashMap;
use crate::ast::TreeNode;
use crate::lexer::Token;

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, node: TreeNode) -> f64 {
        match node {
            TreeNode::Number(n) => n,
            TreeNode::Identifier(id) => {
                if let Some(&value) = self.variables.get(&id) {
                    value
                } else {
                    panic!("undefined variable: {}", id);
                }
            },
            TreeNode::BinaryOp(left, op, right) => {
                let left_val = self.interpret(*left);
                let right_val = self.interpret(*right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    _ => panic!("unexpected operator: {:?}", op),
                }
            },
        }
    }

    pub fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }
}