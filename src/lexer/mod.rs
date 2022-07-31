use crate::token::{get_keyword, Token};
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    ch: u8,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }
    // Helper function to traverse the input string
    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            0 => Token::Eof,
            _ => {
                // Need to early return because we want to treat the blob of text as 1 identifier
                if is_letter(self.ch) {
                    return self.read_identifier();
                };
                if is_number(self.ch) {
                    return self.read_number();
                }
                Token::Illegal
            }
        };
        self.read_char();
        token
    }
    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while is_letter(self.ch) {
            self.read_char();
        }
        let literal = &self.input[start..self.pos];
        get_keyword(literal)
    }
    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while is_number(self.ch) {
            self.read_char();
        }
        let literal = &self.input[start..self.pos];
        Token::Int(literal.parse::<i64>().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }
    fn peek_char(&mut self) -> u8 {
        if self.next_pos >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.next_pos]
        }
    }
}

fn is_number(ch: u8) -> bool {
    (b'0'..=b'9').contains(&ch)
}
fn is_letter(ch: u8) -> bool {
    (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch) || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    #[test]
    fn test_lexer() {
        let input = "=+(){},;";
        let tests = [
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input);
        for res in tests {
            let curr = lexer.next_token();
            assert_eq!(res, curr);
        }
    }
    #[test]
    fn test_lexer2() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        "#;
        let tests = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input);
        for res in tests {
            let curr = lexer.next_token();
            assert_eq!(res, curr);
        }
    }
    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }`
        "#;

        //
    }
}
