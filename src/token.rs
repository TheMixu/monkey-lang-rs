#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String),
    Int(i64),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub fn get_keyword(word: &str) -> Token {
    match word {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(String::from(word)),
    }
}
