use std::fmt;
use std::iter::{Iterator};

#[derive(PartialEq)]
pub enum TokenKind {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Assign,
    LeftParen,
    RightParen,
    ArrayStart,
    ArrayEnd,
    Separator,
    Identifier,
    Number,
    Decimal,
    Illegal,
    Unknown
}

pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,
}

impl<'a> Token<'a> {
    pub fn kind(&self) -> &TokenKind {
        return &self.kind
    }

    pub fn value(&self) -> &'a str {
        return &self.value
    }

    pub fn precedence(&self) -> usize {
        match self.kind {
            TokenKind::Add | TokenKind::Sub => 10,
            TokenKind::Mul | TokenKind::Div => 20,
            _ => 0,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            position: 0,
        }
    }

    fn peek(&mut self) -> &'a str {
        let pos = self.position;

        self.source.get(pos..pos + 1).unwrap_or("\0")
    }

    fn get(&mut self) -> &'a str {
        let pos = self.position;

        self.position += 1;
        self.source.get(pos..pos + 1).unwrap_or("\0")
    }

    fn is_space(&mut self) -> bool {
        let c = self.peek();

        matches!(c, " " | "\t" | "\r" | "\n")
    }

    fn atom(&mut self, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            value: self.get(),
        }
    }

    fn read(&mut self) -> Option<Token<'a>> {
        while self.is_space() {
            self.get();
        }

        match self.peek() {
            "\0" => None,
            "+" => Some(self.atom(TokenKind::Add)),
            "-" => Some(self.atom(TokenKind::Sub)),
            "*" => Some(self.atom(TokenKind::Mul)),
            "/" => Some(self.atom(TokenKind::Div)),
            "^" => Some(self.atom(TokenKind::Exp)),
            "=" => Some(self.atom(TokenKind::Assign)),
            "(" => Some(self.atom(TokenKind::LeftParen)),
            ")" => Some(self.atom(TokenKind::RightParen)),
            "[" => Some(self.atom(TokenKind::ArrayStart)),
            "]" => Some(self.atom(TokenKind::ArrayEnd)),
            "," => Some(self.atom(TokenKind::Separator)),
            s => {
                let b = s.as_bytes()[0];

                if b.is_ascii_digit() {
                    Some(self.read_number())
                } else if b.is_ascii_alphabetic() {
                    Some(self.read_identifier())
                } else {
                    Some(self.atom(TokenKind::Unknown))
                }
            }
        }
    }

    fn read_number(&mut self) -> Token<'a> {
        let start = self.position;
        let mut kind = TokenKind::Number;

        let mut char: &str;
        let mut byte: u8;

        self.get();

        loop {
            char = self.peek();
            byte = char.as_bytes()[0];

            if char == "." {
                if kind == TokenKind::Decimal {
                    kind = TokenKind::Illegal;

                    break;
                } else {
                    kind = TokenKind::Decimal;
                }
            } else if !byte.is_ascii_digit() {
                break;
            }

            self.get();
        }

        let end = self.position;
        let last_char = &self.source[end - 1..end];

        if last_char == "." {
            kind = TokenKind::Illegal;
        }

        Token {
            kind,
            value: &self.source[start..end],
        }
    }

    fn read_identifier(&mut self) -> Token<'a> {
        let start = self.position;

        let mut char: &str;
        let mut byte: u8;

        self.get();

        loop {
            char = self.peek();
            byte = char.as_bytes()[0];

            if !byte.is_ascii_alphanumeric() {
                break;
            }

            self.get();
        }

        let end = self.position;

        Token {
            kind: TokenKind::Identifier,
            value: &self.source[start..end],
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenKind::Add => write!(f, "Add"),
            TokenKind::Sub => write!(f, "Sub"),
            TokenKind::Mul => write!(f, "Mul"),
            TokenKind::Div => write!(f, "Div"),
            TokenKind::Exp => write!(f, "Exp"),
            TokenKind::Assign => write!(f, "Assign"),
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::ArrayStart => write!(f, "ArrayStart"),
            TokenKind::ArrayEnd => write!(f, "ArrayEnd"),
            TokenKind::Separator => write!(f, "Separator"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Number => write!(f, "Number"),
            TokenKind::Decimal => write!(f, "Decimal"),
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Unknown => write!(f, "Unknown"),
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.is_empty() {
            write!(f, "Token({})", self.kind)
        } else {
            write!(f, "Token({}, {})", self.kind, self.value)
        }
    }
}