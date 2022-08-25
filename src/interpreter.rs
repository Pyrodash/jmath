use std::ops::Deref;
use crate::ast::{Node, Operator};
use crate::error::Error;
use crate::memory::{ActivationRecord, CallStack, Value};

pub trait NodeVisitor {
    fn visit(&mut self, node: &Node) -> Result<Value, Error>;
}

pub struct Interpreter {
    stack: CallStack,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stack: CallStack::from_record(
                ActivationRecord::new()
            ),
        }
    }

    pub fn from_record(record: ActivationRecord) -> Interpreter {
        Interpreter {
            stack: CallStack::from_record(record),
        }
    }

    pub fn from_stack(stack: CallStack) -> Interpreter {
        Interpreter {
            stack,
        }
    }

    pub(crate) fn error(msg: &str) -> Error {
        Error::RuntimeError(String::from(msg))
    }
}

impl NodeVisitor for Interpreter {
    fn visit(&mut self, node: &Node) -> Result<Value, Error> {
        match node {
            Node::Number(value) => Ok(Value::Number(value.clone())),
            Node::Decimal(value) => Ok(Value::Decimal(value.clone())),
            Node::Array(vec) => {
                let mut new_vec = Vec::new();

                for item in vec.iter() {
                    new_vec.push(self.visit(item)?);
                }

                Ok(Value::Array(new_vec))
            },
            Node::Variable(name) => {
                let ar = self.stack.peek().unwrap();
                let val = ar.get(&String::from(*name));

                match val {
                    Some(res) => Ok(res.clone()),
                    None => Err(Interpreter::error("Undefined variable")),
                }
            },
            Node::Call { function, arguments } => {
                panic!("Unimplemented")
            },
            Node::Assign { lhs, rhs } => {
                match lhs.deref() {
                    Node::Variable(name) => {
                        let value = self.visit(rhs)?;
                        let res = value.clone();

                        let ar = self.stack.peek_mut().unwrap();

                        ar.insert( String::from(*name), value);

                        Ok(res)
                    },
                    _ => Err(Interpreter::error("Invalid assignment left-hand-side"))
                }
            },
            Node::UnaryOp { op, rhs } => {
                let right = self.visit(rhs.deref())?;

                match op {
                    Operator::Add => Ok(right),
                    Operator::Sub => Ok(-right),
                    _ => Err(Interpreter::error("Invalid unary operation")),
                }
            },
            Node::BinaryOp { op, lhs, rhs } => {
                let left = self.visit(lhs.deref())?;
                let right = self.visit(rhs.deref())?;

                let res = match op {
                    Operator::Add => left + right,
                    Operator::Sub => left - right,
                    Operator::Mul => left * right,
                    Operator::Div => left / right,
                    Operator::Exp => left.pow(right),
                };

                Ok(res)
            },
        }
    }
}