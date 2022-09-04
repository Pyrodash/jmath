use std::iter::{Iterator, Peekable};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::ast::{Block, Node, Operator};
use crate::error::Error;

pub struct Parser<'a> {
    source: Peekable<&'a mut Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn error(msg: &'a str) -> Error {
        Error::ParserError(String::from(msg))
    }

    fn eat(&mut self) -> Option<Token<'a>> {
        self.source.next()
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, Error> {
        let err = Error::ParserError(format!("Expected {}", kind));
        let token_opt = self.eat();

        match token_opt {
            Some(token) => {
                if token.kind() != &kind {
                    return Err(err);
                }

                Ok(token)
            },
            None => Err(err)
        }
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        self.source.peek()
    }

    fn parse_number(&mut self) -> Node<'a> {
        let value = self.eat().unwrap().value();
        let res = Node::Number(value.parse::<i64>().unwrap());

        res
    }

    fn parse_decimal(&mut self) -> Node<'a> {
        let value = self.eat().unwrap().value();
        let res = Node::Decimal(value.parse::<f64>().unwrap());

        res
    }

    fn parse_assignment(&mut self, name: &'a str) -> Result<Node<'a>, Error> {
        self.eat();

        let right = self.parse_expr()?;

        match right {
            Some(rhs) => {
                let res = Node::Assign {
                    lhs: name,
                    rhs: Box::new(rhs)
                };

                Ok(res)
            },
            None => Err(Parser::error("Expected right-hand-side of assignment operation")),
        }
    }

    fn parse_call(&mut self, name: &'a str) -> Result<Node<'a>, Error> {
        self.eat();

        let mut arguments = Vec::new();

        loop {
            let expr = self.parse_expr()?;

            match expr {
                Some(node) => arguments.push(node) ,
                None => {
                    self.expect(TokenKind::RightParen)?;
                    break;
                }
            }
        }

        Ok(Node::Call {
            function: name,
            arguments,
        })
    }

    fn parse_block(&mut self, expect_braces: bool) -> Result<Block<'a>, Error> {
        if expect_braces {
            self.expect(TokenKind::BlockStart)?;
        }

        let mut nodes: Vec<Node<'a>> = Vec::new();

        loop {
            let expr = self.parse_expr()?;

            if !expr.is_none() {
                nodes.push(expr.unwrap());
            }

            let token = self.peek();

            if token.is_none() || token.unwrap().kind() != &TokenKind::Semicolon {
                break;
            }
        }

        if expect_braces {
            self.expect(TokenKind::BlockEnd)?;
        }

        Ok(Block(nodes))
    }

    fn parse_function_parameter(&mut self) -> Result<Option<Node<'a>>, Error> {
        let name_token_opt = self.peek();

        if name_token_opt.is_none() {
            return Ok(None)
        }

        let name_token = name_token_opt.unwrap();

        if name_token.kind() != &TokenKind::Identifier {
            return Ok(None)
        }

        let name = name_token.value();

        self.eat();
        self.expect(TokenKind::Colon)?;

        let kind = self.expect(TokenKind::Identifier)?.value();

        Ok(Some(Node::Declaration {
            name,
            kind
        }))
    }

    fn parse_function(&mut self) -> Result<Node<'a>, Error> {
        let name = self.expect(TokenKind::Identifier)?.value();
        let mut params: Vec<Node<'a>> = Vec::new();

        self.expect(TokenKind::LeftParen)?;

        loop {
            let param = self.parse_function_parameter()?;

            if param.is_none() {
                break;
            } else {
                params.push(param.unwrap());
            }
        }

        self.expect(TokenKind::RightParen)?;

        let nodes = self.parse_block(true)?;

        Ok(Node::Function {
            name,
            parameters: params,
            body: nodes
        })
    }

    fn parse_identifier(&mut self) -> Result<Node<'a>, Error> {
        let name = self.eat().unwrap().value();

        match name {
            "fn" => self.parse_function(),
            _ => {
                let token = self.peek();

                if token.is_none() {
                    Ok(Node::Variable(name))
                } else {
                    match token.unwrap().kind() {
                        &TokenKind::Assign => {
                            self.parse_assignment(name)
                        },
                        &TokenKind::LeftParen => {
                            self.parse_call(name)
                        },
                        _ => Ok(Node::Variable(name)),
                    }
                }
            }
        }
    }

    fn parse_paren_expr(&mut self) -> Result<Node<'a>, Error> {
        self.eat();

        let val = self.parse_expr()?;

        match val {
            Some(expr) => {
                self.expect(TokenKind::RightParen)?;

                Ok(expr)
            },
            None => Err(Parser::error("Expected expression inside parenthesis"))
        }
    }

    fn parse_array_expr(&mut self) -> Result<Node<'a>, Error> {
        self.eat();

        let mut token;
        let mut nodes = Vec::new();

        loop {
            let node = self.parse_expr()?;

            if node.is_none() {
                break;
            } else {
                nodes.push(node.unwrap());
            }

            token = self.peek();

            if !token.is_none() {
                if token.unwrap().kind() != &TokenKind::Separator {
                    // expected separator
                    break;
                } else {
                    self.eat();
                }
            }
        }

        self.expect(TokenKind::ArrayEnd)?;

        Ok(Node::Array(nodes))
    }

    fn parse_unary_op(&mut self) -> Result<Node<'a>, Error> {
        let op: Operator = self.eat().unwrap().value().into();

        let value = self.parse_primary_expr()?;

        match value {
            Some(expr) => Ok(Node::UnaryOp {
                op,
                rhs: Box::new(expr)
            }),
            None => Err(Parser::error("Expected a right-hand-side node"))
        }
    }

    fn parse_primary_expr(&mut self) -> Result<Option<Node<'a>>, Error> {
        let token = self.peek();

        if token.is_none() {
            return Ok(None);
        }

        match token.unwrap().kind() {
            &TokenKind::Add | &TokenKind::Sub => self.parse_unary_op().map(|node| Option::from(node)),
            &TokenKind::Number => Ok(Option::from(self.parse_number())),
            &TokenKind::Decimal => Ok(Option::from(self.parse_decimal())),
            &TokenKind::Identifier => self.parse_identifier().map(|node| Option::from(node)),
            &TokenKind::LeftParen => self.parse_paren_expr().map(|node| Option::from(node)),
            &TokenKind::ArrayStart => self.parse_array_expr().map(|node| Option::from(node)),
            _ => Ok(None),
        }
    }

    fn parse_expr_right(&mut self, precedence: usize, mut left: Node<'a>) -> Result<Node<'a>, Error> {
        loop {
            let mut token = self.peek();
            let token_prec: usize = if token.is_none() {
                0
            } else {
                token.unwrap().precedence()
            };

            if token_prec < precedence {
                return Ok(left);
            }

            let op: Operator = token.unwrap().value().into();

            // we know this token IS a binary operator
            self.eat();

            let value = self.parse_primary_expr()?;

            match value {
                Some(mut right) => {
                    token = self.peek();

                    if !token.is_none() {
                        let next_prec = token.unwrap().precedence();

                        if token_prec < next_prec {
                            right = self.parse_expr_right(token_prec + 1, right)?;
                        }
                    }

                    left = Node::BinaryOp {
                        op,
                        lhs: Box::new(left),
                        rhs: Box::new(right)
                    };
                },
                None => return Err(Parser::error("Expected right-hand-side expression"))
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Option<Node<'a>>, Error> {
        let left = self.parse_primary_expr()?;

        if left.is_none() {
            return Ok(None);
        }

        self.parse_expr_right(1, left.unwrap()).map(|node| Option::from(node))
    }

    fn parse(&mut self) -> Result<Vec<Node<'a>>, Error> {
        let mut vec = Vec::new();

        loop {
            let expr = self.parse_expr()?;

            if !expr.is_none() {
                vec.push(expr.unwrap());
            }

            let token = self.peek();

            if token.is_none() || token.unwrap().kind() != &TokenKind::Semicolon {
                break;
            }

            self.eat();
        }

        Ok(vec)
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a mut Lexer<'a>) -> Parser {
        Parser {
            source: source.peekable(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<Node<'a>>, Error> {
        self.parse()
    }
}