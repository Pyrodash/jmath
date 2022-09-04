use std::ops::Deref;
use crate::ast::{Node, Operator};
use crate::error::Error;
use crate::memory::{ActivationRecord, CallStack, Value};

pub struct Interpreter {
    stack: CallStack,
}

fn matrix_transpose(args: Vec<Value>) -> Result<Value, Error> {
    let mat_value_opt = args.first();

    if mat_value_opt.is_none() {
        return Result::Err(Interpreter::error("Expected a matrix"));
    }

    match mat_value_opt.unwrap() {
        Value::Array(mat) => {
            let rows = mat.len();
            let mut cols = 0;

            if rows > 0 {
                cols = mat.iter().next().unwrap().as_array().len();
            }

            let mut res: Vec<Value> = Vec::with_capacity(cols);

            for i in 0..cols {
                let row = Vec::with_capacity(rows);

                res.push(Value::Array(row));
            }

            for i in 0..rows {
                let row_vec = mat[i].as_array();

                for j in 0..cols {
                    let res_vec = res[j].as_array_mut();

                    res_vec.insert(i, row_vec[j].clone());
                }
            }

            Result::Ok(Value::Array(res))
        },
        _ => Result::Err(Interpreter::error("Expected a matrix")),
    }
}

pub fn add_natives(mut ar: ActivationRecord) -> ActivationRecord {
    ar.insert(String::from("trn"), Value::NativeFunction(matrix_transpose));

    return ar;
}

pub trait WithNatives {
    fn with_natives() -> Self;
}

impl WithNatives for ActivationRecord {
    fn with_natives() -> Self {
        add_natives(ActivationRecord::new())
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stack: CallStack::from_record(
                ActivationRecord::with_natives()
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

impl Interpreter {
    pub fn visit(&mut self, node: &Node) -> Result<Value, Error> {
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
            Node::Call { function: fn_name, arguments } => {
                let mut args: Vec<Value> = Vec::new();

                for node in arguments.iter() {
                    args.push(self.visit(node)?);
                }

                let curr_ar = self.stack.peek().unwrap();
                let fn_value = curr_ar.get(&String::from(*fn_name));

                if fn_value.is_none() {
                    return Result::Err(Interpreter::error("Function not found"))
                }

                match fn_value.unwrap() {
                    Value::NativeFunction(fn_ref) => {
                        Result::Ok(fn_ref(args)?)
                    },
                    _ => Result::Err(Interpreter::error("Invalid function")),
                }
            },
            Node::Assign { lhs, rhs } => {
                let value = self.visit(rhs)?;
                let res = value.clone();

                let ar = self.stack.peek_mut().unwrap();

                ar.insert( String::from(*lhs), value);

                Ok(res)
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
            Node::Declaration { name, kind } => todo!("Unimplemented"),
            Node::Function { name, parameters, body } => todo!("Unimplemented"),
        }
    }
}