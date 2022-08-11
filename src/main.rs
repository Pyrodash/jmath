mod interpreter;

use crate::interpreter::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("123+world");

    println!("{}", lexer.read());
    println!("{}", lexer.read());
    println!("{}", lexer.read());
}
