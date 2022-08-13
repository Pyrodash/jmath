use std::collections::HashMap;
use std::ops::Deref;
use crate::{Node, Operator};
use crate::interpreter::memory::{Memory, Value};

pub trait NodeVisitor<'a> {
    fn visit(&mut self, node: &'a Node) -> Value;
}

pub struct Interpreter<'a> {
    memory: &'a mut Memory<'a>
}

impl<'a> Interpreter<'a> {
    pub fn new(memory: &'a mut Memory<'a>) -> Interpreter<'a> {
        Interpreter {
            memory,
        }
    }
}

impl<'a> NodeVisitor<'a> for Interpreter<'a> {
    fn visit(&mut self, node: &'a Node) -> Value {
        match node {
            Node::Number(value) => Value::Number(*value),
            Node::Decimal(value) => Value::Decimal(*value),
            Node::Array(vec) => {
                let mut new_vec = Vec::new();

                for item in vec.iter() {
                    new_vec.push(self.visit(item));
                }

                Value::Array(new_vec)
            },
            Node::Variable(name) => {
                self.memory.get(*name).expect("Variable not found").clone()
            },
            Node::Assign { lhs, rhs } => {
                match lhs.deref() {
                    Node::Variable(name) => {
                        let value = self.visit(rhs);
                        let res = value.clone();

                        self.memory.insert(*name, value);

                        res
                    },
                    _ => panic!("Invalid assignment left-hand-side")
                }
            },
            Node::UnaryOp { op, rhs } => {
                let right = self.visit(rhs.deref());

                match op {
                    Operator::Add => right,
                    Operator::Sub => -right,
                    _ => panic!("Invalid unary operation"),
                }
            },
            Node::BinaryOp { op, lhs, rhs } => {
                let left = self.visit(lhs.deref());
                let right = self.visit(rhs.deref());

                match op {
                    Operator::Add => left + right,
                    Operator::Sub => left - right,
                    Operator::Mul => left * right,
                    Operator::Div => left / right,
                }
            },
        }
    }
}