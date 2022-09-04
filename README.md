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
use crate::interpreter::interpreter::{Interpreter, NodeVisitor, WithNatives};
use crate::interpreter::memory::{ActivationRecord, Value};

fn main() {
    let mut lexer = Lexer::new("a = b+2; a");
    let mut parser = Parser::new(&mut lexer);

    let mut ar = ActivationRecord::with_natives();

    ar.insert(String::from("b"), Value::Number(2));

    let mut interpreter = Interpreter::from_record(ar);

    let nodes = parser.run().unwrap();

    println!("{:?}", nodes);

    for node in nodes.iter() {
        println!("{}", interpreter.visit(node).unwrap())
    }
}
```

### REPL
To use the REPL, clone the project and then run
```shell
cargo run
```