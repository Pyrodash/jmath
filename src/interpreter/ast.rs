use std::fmt;

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

impl<'a> From<&'a str> for Operator {
    fn from(item: &'a str) -> Self {
        match item {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => panic!("unknown operator")
        }
    }
}

#[derive(Debug)]
pub enum Node<'a> {
    Number(isize),
    Decimal(f64),
    Array(Vec<Box<Node<'a>>>),
    Variable(&'a str),
    Assign {
        lhs: Box<Node<'a>>,
        rhs: Box<Node<'a>>
    },
    UnaryOp {
        op: Operator,
        rhs: Box<Node<'a>>
    },
    BinaryOp {
        op: Operator,
        lhs: Box<Node<'a>>,
        rhs: Box<Node<'a>>
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Number(value) => write!(f, "NumberNode({})", value),
            Node::Decimal(value) => write!(f, "DecimalNode({})", value),
            Node::Array(value) => write!(f, "ArrayNode({:?})", value),
            Node::Variable(value) => write!(f, "VariableNode({})", value),
            Node::Assign { lhs, rhs } => write!(f, "AssignNode({}, {})", lhs, rhs),
            Node::UnaryOp { op, rhs} => write!(f, "UnaryOpNode({}, {})", op, rhs),
            Node::BinaryOp { op, lhs, rhs} => write!(f, "BinaryOpNode({}, {}, {})", lhs, op, rhs),
        }
    }
}