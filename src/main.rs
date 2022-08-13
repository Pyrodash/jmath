mod interpreter;

use crate::interpreter::lexer::Lexer;
use crate::interpreter::parser::Parser;
use crate::interpreter::interpreter::{Interpreter, NodeVisitor};
use crate::interpreter::ast::*;

fn main() {
    let mut lexer = Lexer::new("[1, 2]*2*2");
    let mut parser = Parser::new(&mut lexer);
    let interpreter = Interpreter{};

    let root_node = parser.run().unwrap();

    println!("{}", root_node);
    println!("{}", interpreter.visit(&root_node))
}
