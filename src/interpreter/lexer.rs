use std::fmt;

#[derive(PartialEq)]
enum TokenKind {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Assign,
    LeftParen,
    RightParen,
    Identifier,
    Int,
    Double,
    Illegal,
    Unknown,
    End
}

pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,
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

    fn next(&mut self) -> &'a str {
        let pos = self.position;

        self.position += 1;
        self.source.get(pos..pos + 1).unwrap_or("\0")
    }

    fn is_space(&mut self) -> bool {
        let c = self.peek();

        matches!(c, " " | "\t" | "\r" | "\n")
    }

    fn atom(&mut self, kind: TokenKind) -> Token {
        Token {
            kind,
            value: self.next(),
        }
    }

    pub fn read(&mut self) -> Token {
        while self.is_space() {
            self.next();
        }

        match self.peek() {
            "\0" => self.atom(TokenKind::End),
            "+" => self.atom(TokenKind::Add),
            "-" => self.atom(TokenKind::Sub),
            "*" => self.atom(TokenKind::Mul),
            "/" => self.atom(TokenKind::Div),
            "^" => self.atom(TokenKind::Exp),
            "=" => self.atom(TokenKind::Assign),
            "(" => self.atom(TokenKind::LeftParen),
            ")" => self.atom(TokenKind::RightParen),
            s => {
                let b = s.as_bytes()[0];

                return if b.is_ascii_digit() {
                    self.read_number()
                } else if b.is_ascii_alphabetic() {
                    self.read_identifier()
                } else {
                    self.atom(TokenKind::Unknown)
                }
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut kind = TokenKind::Int;

        let mut char: &str;
        let mut byte: u8;

        self.next();

        loop {
            char = self.peek();
            byte = char.as_bytes()[0];

            if char == "." {
                if kind == TokenKind::Double {
                    kind = TokenKind::Illegal;

                    break;
                } else {
                    kind = TokenKind::Double;
                }
            } else if !byte.is_ascii_digit() {
                break;
            }

            self.next();
        }

        let end = self.position;
        let last_char = &self.source[end..end + 1];

        if last_char == "." {
            kind = TokenKind::Illegal;
        }

        Token {
            kind,
            value: &self.source[start..end],
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        let mut char: &str;
        let mut byte: u8;

        self.next();

        loop {
            char = self.peek();
            byte = char.as_bytes()[0];

            if !byte.is_ascii_alphanumeric() {
                break;
            }

            self.next();
        }

        let end = self.position;

        Token {
            kind: TokenKind::Identifier,
            value: &self.source[start..end],
        }
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
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Double => write!(f, "Double"),
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Unknown => write!(f, "Unknown"),
            TokenKind::End => write!(f, "End")
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