mod interpreter;

use crate::interpreter::lexer::Lexer;
use crate::interpreter::parser::Parser;
use crate::interpreter::ast::*;

fn main() {
    let mut lexer = Lexer::new("1+++1");
    let mut parser = Parser::new(&mut lexer);

    println!("{}", parser.run().unwrap());
}
