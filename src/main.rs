mod error;
mod lexer;
mod ast;
mod parser;
mod memory;
mod interpreter;

use std::io::{Write};
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::{Interpreter};
use crate::memory::{Value};

fn repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new().expect("Failed to initialize CLI");
    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                if line == "exit" {
                    break
                }

                let mut lexer = Lexer::new(line.as_str());
                let mut parser = Parser::new(&mut lexer);

                let nodes = parser.run()?;
                let mut value: Value = Value::Number(0);

                if nodes.len() == 0 {
                    continue;
                }

                for node in nodes {
                    value = interpreter.visit(&node)?
                }

                println!("{}", value)
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => return Result::Err(Box::new(err))
        }
    }

    Result::Ok(())
}

fn main() {
    let res = repl();

    if res.is_err() {
        println!("{}", res.err().unwrap());
        main();
    }
}