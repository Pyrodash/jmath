# jmath

jmath is a simple tree-visiting interpreter that evaluates mathematical expressions.

## Features
* Addition
* Subtraction
* Multiplication
* Division
* Arrays
* Matrix Multiplication (2d arrays are matrices)
* Variables

__Note:__ All math operations that you can perform on a matrix can be performed on arrays

## Usage
```rust
use crate::interpreter::lexer::Lexer;
use crate::interpreter::parser::Parser;
use crate::interpreter::interpreter::{Interpreter, NodeVisitor};
use crate::interpreter::memory::{Memory, Value};

fn main() {
    let mut lexer = Lexer::new("a = b+2; a");
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
```