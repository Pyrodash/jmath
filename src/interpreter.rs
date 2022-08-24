use std::ops::Deref;
use crate::ast::{Node, Operator};
use crate::error::Error;
use crate::memory::{ActivationRecord, CallStack, Value};

pub trait NodeVisitor<'a> {
    fn visit(&mut self, node: &'a Node) -> Result<Value, Error>;
}

pub struct Interpreter<'a> {
    stack: CallStack<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Interpreter<'a> {
        Interpreter {
            stack: CallStack::from_record(
                ActivationRecord::new()
            ),
        }
    }

    pub fn from_record(record: ActivationRecord<'a>) -> Interpreter<'a> {
        Interpreter {
            stack: CallStack::from_record(record),
        }
    }

    pub fn from_stack(stack: CallStack<'a>) -> Interpreter<'a> {
        Interpreter {
            stack,
        }
    }

    pub(crate) fn error(msg: &'a str) -> Error {
        Error::RuntimeError(String::from(msg))
    }
}

impl<'a> NodeVisitor<'a> for Interpreter<'a> {
    fn visit(&mut self, node: &'a Node) -> Result<Value, Error> {
        match node {
            Node::Number(value) => Ok(Value::Number(*value)),
            Node::Decimal(value) => Ok(Value::Decimal(*value)),
            Node::Array(vec) => {
                let mut new_vec = Vec::new();

                for item in vec.iter() {
                    new_vec.push(self.visit(item)?);
                }

                Ok(Value::Array(new_vec))
            },
            Node::Variable(name) => {
                let ar = self.stack.peek().unwrap();
                let val = ar.get(*name);

                match val {
                    Some(res) => Ok(res.clone()),
                    None => Err(Interpreter::error("Undefined variable")),
                }
            },
            Node::Assign { lhs, rhs } => {
                match lhs.deref() {
                    Node::Variable(name) => {
                        let value = self.visit(rhs)?;
                        let res = value.clone();

                        let ar = self.stack.peek_mut().unwrap();

                        ar.insert(* name, value);

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
                };

                Ok(res)
            },
        }
    }
}