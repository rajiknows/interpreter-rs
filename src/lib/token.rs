use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone)]
pub enum Token {
    Ident(String),
    Int(String),
    Illegal,
    Eof,
    Assign,
    Bang,
    Dash,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
    Print,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "Ident({})", x),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "Eof"),
            Token::Assign => write!(f, "Assign"),
            Token::Bang => write!(f, "Bang"),
            Token::Dash => write!(f, "Dash"),
            Token::ForwardSlash => write!(f, "ForwardSlash"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::Plus => write!(f, "Plus"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Lparen => write!(f, "Lparen"),
            Token::Rparen => write!(f, "Rparen"),
            Token::LSquirly => write!(f, "LSquirly"),
            Token::RSquirly => write!(f, "RSquirly"),
            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::Return => write!(f, "Return"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Print => write!(f,"Print")
        }
    }
}
