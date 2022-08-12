use std::iter::{Iterator};
use crate::interpreter::lexer::{Token, TokenKind};
use crate::{Lexer, Node, Operator};

pub struct Parser<'a> {
    token: Option<Token<'a>>,
    source: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    fn get(&mut self)  {
        self.token = self.source.next();
    }

    fn token(&self) -> Option<&Token<'a>> {
        self.token.as_ref()
    }

    fn parse_number(&mut self) -> Node<'a> {
        let value = self.token().unwrap().value();
        let res = Node::Number(value.parse::<isize>().unwrap());

        self.get();

        res
    }

    fn parse_decimal(&mut self) -> Node<'a> {
        let value = self.token().unwrap().value();
        let res = Node::Decimal(value.parse::<f64>().unwrap());

        self.get();

        res
    }

    fn parse_identifier(&mut self) -> Node<'a> {
        let name = self.token().unwrap().value();
        let var = Node::Variable(name);

        self.get();

        if self.token.is_none() {
            var
        } else {
            let token = self.token().unwrap();

            match token.kind() {
                &TokenKind::Assign => {
                    self.get();

                    let rhs = self.parse_expr().expect("Expected right-hand-side of assignment operation");
                    let res = Node::Assign {
                        lhs: Box::new(var),
                        rhs: Box::new(rhs)
                    };

                    return res
                },
                _ => return var,
            }
        }
    }

    fn parse_paren_expr(&mut self) -> Node<'a> {
        self.get();

        let expr = self.parse_expr().expect("Expected expression inside parenthesis");
        let token = self.token().expect("Expected to find parenthesis end");

        if token.kind() != &TokenKind::RightParen {
            panic!("Expected to find parenthesis end");
        }

        self.get();

        expr
    }

    fn parse_array_expr(&mut self) -> Node<'a> {
        self.get();

        let mut token;
        let mut node;
        let mut nodes = Vec::new();

        loop {
            node = self.parse_primary_expr();

            if node.is_none() {
                break;
            } else {
                nodes.push(Box::new(node.unwrap()));
            }

            token = self.token();

            if !token.is_none() {
                if token.unwrap().kind() != &TokenKind::Separator {
                    break;
                } else {
                    self.get();
                }
            }
        }

        token = self.token();

        if token.is_none() || token.unwrap().kind() != &TokenKind::ArrayEnd {
            panic!("Expected array close");
        }

        Node::Array(nodes)
    }

    fn parse_unary_op(&mut self) -> Node<'a> {
        let op: Operator = self.token().unwrap().value().into();

        self.get();

        let expr = self.parse_primary_expr().expect("Expected a right-hand-side node");

        Node::UnaryOp {
            op,
            rhs: Box::new(expr)
        }
    }

    fn parse_primary_expr(&mut self) -> Option<Node<'a>> {
        if self.token.is_none() {
            return None;
        }

        let token = self.token().unwrap();

        match token.kind() {
            &TokenKind::Add | &TokenKind::Sub => Option::from(self.parse_unary_op()),
            &TokenKind::Number => Option::from(self.parse_number()),
            &TokenKind::Decimal => Option::from(self.parse_decimal()),
            &TokenKind::Identifier => Option::from(self.parse_identifier()),
            &TokenKind::LeftParen => Option::from(self.parse_paren_expr()),
            &TokenKind::ArrayStart => Option::from(self.parse_array_expr()),
            _ => None
        }
    }

    fn parse_expr_right(&mut self, precedence: usize, mut left: Node<'a>) -> Node<'a> {
        loop {
            let token_prec: usize = if self.token.is_none() {
                0
            } else {
                self.token().unwrap().precedence()
            };

            if token_prec < precedence {
                return left;
            }

            let op: Operator = self.token().unwrap().value().into();

            // we know this token IS a binary operator
            self.get();

            let mut right = self.parse_primary_expr().expect("Expected right-hand-side expression");

            if !self.token.is_none() {
                let next_prec = self.token().unwrap().precedence();

                if token_prec < next_prec {
                    right = self.parse_expr_right(token_prec + 1, right);
                }
            }

            left = Node::BinaryOp {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right)
            };
        }
    }

    fn parse_expr(&mut self) -> Option<Node<'a>> {
        let left = self.parse_primary_expr();

        if left.is_none() {
            return None;
        }

        Option::from(self.parse_expr_right(1, left.unwrap()))
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a mut Lexer<'a>) -> Parser {
        Parser {
            token: None,
            source,
        }
    }

    pub fn run(&mut self) -> Option<Node<'a>> {
        self.get();
        self.parse_expr()
    }
}