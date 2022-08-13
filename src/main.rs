mod error;
mod lexer;
mod ast;
mod parser;
mod memory;
mod interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::{Interpreter, NodeVisitor};
use crate::memory::{Memory, Value};

fn main() {
    let mut lexer = Lexer::new("a=b+2; a");
    let mut parser = Parser::new(&mut lexer);

    let mut memory = Memory::new();

    memory.insert("b", Value::Number(2));

    let mut interpreter = Interpreter::new(&mut memory);

    let nodes = parser.run();

    println!("{:?}", nodes);

    for node in nodes.iter() {
        println!("{}", interpreter.visit(node))
    }
}
