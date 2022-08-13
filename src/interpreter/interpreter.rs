use std::fmt;
use std::fmt::Formatter;
use std::ops;
use std::ops::{Deref};
use crate::{Node, Operator};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Decimal(f64),
    Array(Vec<Value>),
}

impl Value {
    fn as_array(&self) -> &Vec<Value> {
        if let Value::Array(v) = self {
            v
        } else {
            panic!("Invalid array")
        }
    }

    fn as_array_mut(&mut self) -> &mut Vec<Value> {
        if let Value::Array(v) = self {
            v
        } else {
            panic!("Invalid array")
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{}", value),
            Value::Decimal(value) => write!(f, "{}", value),
            Value::Array(value) => write!(f, "{:?}", value),
        }
    }
}

impl ops::Add<i64> for Value {
    type Output = Value;

    fn add(self, rhs: i64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Number(lhs + rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs + (rhs as f64)),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| value + rhs).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Add<f64> for Value {
    type Output = Value;

    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Decimal((lhs as f64) + rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs + rhs),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| value + rhs).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Add<Vec<Value>> for Value {
    type Output = Value;

    fn add(self, rhs: Vec<Value>) -> Self::Output {
        match self {
            Value::Number(lhs) => {
                let arr = rhs.into_iter().map(|value| value + lhs).collect();

                Value::Array(arr)
            },
            Value::Decimal(lhs) => {
                let arr = rhs.into_iter().map(|value| value + lhs).collect();

                Value::Array(arr)
            },
            Value::Array(lhs) => {
                let arr = rhs.into_iter().zip(lhs.into_iter()).map(|(x, y)| x + y).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => rhs + lhs,
            Value::Decimal(lhs) => rhs + lhs,
            Value::Array(lhs) => rhs + lhs,
        }
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Number(-lhs),
            Value::Decimal(lhs) => Value::Decimal(-lhs),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| -value).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Sub<i64> for Value {
    type Output = Value;

    fn sub(self, rhs: i64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Number(lhs - rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs - (rhs as f64)),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| value - rhs).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Sub<f64> for Value {
    type Output = Value;

    fn sub(self, rhs: f64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Decimal((lhs as f64) - rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs - rhs),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| value - rhs).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Sub<Vec<Value>> for Value {
    type Output = Value;

    fn sub(self, rhs: Vec<Value>) -> Self::Output {
        match self {
            Value::Number(lhs) => {
                let arr = rhs.into_iter().map(|value| value - lhs).collect();

                Value::Array(arr)
            },
            Value::Decimal(lhs) => {
                let arr = rhs.into_iter().map(|value| value - lhs).collect();

                Value::Array(arr)
            },
            Value::Array(lhs) => {
                let arr = rhs.into_iter().zip(lhs.into_iter()).map(|(x, y)| x - y).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => rhs - lhs,
            Value::Decimal(lhs) => rhs - lhs,
            Value::Array(lhs) => rhs - lhs,
        }
    }
}

impl ops::Mul<i64> for Value {
    type Output = Value;

    fn mul(self, rhs: i64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Number(lhs * rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs * (rhs as f64)),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| value * rhs).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Mul<f64> for Value {
    type Output = Value;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Decimal((lhs as f64) * rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs * rhs),
            Value::Array(lhs) => {
                let arr = lhs.into_iter().map(|value| value * rhs).collect();

                Value::Array(arr)
            },
        }
    }
}

impl ops::Mul<Vec<Value>> for Value {
    type Output = Value;

    fn mul(self, lhs: Vec<Value>) -> Self::Output {
        match self {
            Value::Number(rhs) => {
                let arr = lhs.into_iter().map(|value| value * rhs).collect();

                Value::Array(arr)
            }
            Value::Decimal(rhs) => {
                let arr = lhs.into_iter().map(|value| value * rhs).collect();

                Value::Array(arr)
            },
            Value::Array(rhs) => {
                let r1 = lhs.len();
                let r2 = rhs.len();

                if r1 == 0 || r2 == 0 {
                    panic!("Cannot multiply an empty matrix")
                } else {
                    let c1 = lhs.iter().next().unwrap().as_array().len();
                    let c2 = rhs.iter().next().unwrap().as_array().len();

                    if c1 != r2 {
                        panic!("Cannot multiply matrices");
                    }

                    // lhs_iter = lhs.into_iter();
                    // rhs_iter = rhs.into_iter();

                    let mut res: Vec<Value> = Vec::with_capacity(r1);

                    for _ in 0..r1 {
                        let mut vec = Vec::with_capacity(c2);

                        for _ in 0..c2 {
                            vec.push(Value::Number(0));
                        }

                        res.push(Value::Array(vec));
                    }

                    for i in 0..r1 {
                        let res_vec = res[i].as_array_mut();

                        for j in 0..c2 {
                            for k in 0..c1 {
                                let lhs_vec = lhs[i].as_array();
                                let rhs_vec = rhs[k].as_array();


                                res_vec[j] = res_vec[j].clone() + lhs_vec[k].clone() * rhs_vec[j].clone();
                            }
                        }
                    }

                    return Value::Array(res);
                }
            },
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => rhs * lhs,
            Value::Decimal(lhs) => rhs * lhs,
            Value::Array(lhs) => rhs * lhs,
        }
    }
}

impl ops::Div<i64> for Value {
    type Output = Value;

    fn div(self, rhs: i64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Number(lhs / rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs / (rhs as f64)),
            Value::Array(_) => panic!("Matrix division is impossible"),
        }
    }
}

impl ops::Div<f64> for Value {
    type Output = Value;

    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Value::Number(lhs) => Value::Decimal((lhs as f64) / rhs),
            Value::Decimal(lhs) => Value::Decimal(lhs / rhs),
            Value::Array(_) => panic!("Matrix division is impossible"),
        }
    }
}

impl ops::Div<Vec<Value>> for Value {
    type Output = Value;

    fn div(self, _: Vec<Value>) -> Self::Output {
        panic!("Matrix division is impossible")
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => rhs / lhs,
            Value::Decimal(lhs) => rhs / lhs,
            Value::Array(lhs) => rhs / lhs,
        }
    }
}

pub trait NodeVisitor {
    fn visit(&self, node: &Node) -> Value;
}

pub struct Interpreter {}

impl NodeVisitor for Interpreter {
    fn visit(&self, node: &Node) -> Value {
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
                todo!("memory")

            },
            Node::Assign { lhs, rhs } => {
                todo!("memory")
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