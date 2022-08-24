mod error;
mod lexer;
mod ast;
mod parser;
mod memory;
mod interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::{Interpreter, NodeVisitor};
use crate::memory::{ActivationRecord, Value};

fn main() {
    let mut lexer = Lexer::new("a=(b/2); a");
    let mut parser = Parser::new(&mut lexer);

    let mut ar = ActivationRecord::new();

    ar.insert("b", Value::Number(2));

    let mut interpreter = Interpreter::from_record(ar);

    let nodes = parser.run().unwrap();

    println!("{:?}", nodes);

    for node in nodes.iter() {
        match interpreter.visit(node) {
            Ok(val) => println!("{}", val),
            Err(err) => panic!("{}", err)
        }
    }
}
