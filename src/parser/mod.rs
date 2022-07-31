use crate::{lexer::Lexer, token::Token};

use self::ast::*;
mod ast;
type ParseError = String;
type ParseErrors = Vec<ParseError>;
struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    next_token: Token,
    errors: ParseErrors,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::Eof,
            next_token: Token::Eof,
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn get_errors(&self) -> ParseErrors {
        self.errors.clone()
    }
    fn peek_error(&mut self, token: Token) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token, self.next_token
        );
        self.errors.push(msg);
    }

    fn next_precedence(&self) -> Precedence {
        get_precedence(&self.next_token)
    }
    fn curr_precedence(&self) -> Precedence {
        get_precedence(&self.curr_token)
    }
    fn next_token(&mut self) {
        // TODO: Check if the commented line works instead of cloning
        // self.curr_token = mem::replace(&mut self.peek_token, Token::Illegal);
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }
    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while !self.curr_token_is(Token::Eof) {
            let statement = self.parse_statement();

            if let Some(val) = statement {
                program.push(val);
            }
            self.next_token();
        }
        program
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expr_statement(),
        }
    }
    fn parse_let_statement(&mut self) -> Option<Statement> {
        match &self.next_token {
            Token::Ident(_) => self.next_token(),
            _ => return None,
        };
        let name = match self.parse_ident() {
            Some(name) => name,
            _ => return None,
        };
        if !self.expect_next_token(Token::Assign) {
            return None;
        }
        self.next_token();
        let expression = match self.parse_expr(Precedence::Lowest) {
            Some(val) => val,
            _ => return None,
        };
        while !self.curr_token_is(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::Let(name, expression))
    }
    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let expression = self.parse_expr(Precedence::Lowest);
        while !self.curr_token_is(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::Return(expression))
    }
    fn parse_expr_statement(&mut self) -> Option<Statement> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&Token::Semicolon) {
                    self.next_token();
                }
                Some(Statement::Expr(expr))
            }
            None => None,
        }
    }
    fn curr_token_is(&self, token: Token) -> bool {
        self.curr_token == token
    }
    fn next_token_is(&self, token: &Token) -> bool {
        self.next_token == *token
    }
    fn expect_next_token(&mut self, token: Token) -> bool {
        if self.next_token_is(&token) {
            println!(
                "expect peek, curr {:?} next {:?}",
                self.curr_token, self.next_token
            );
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }
    fn parse_ident(&mut self) -> Option<Ident> {
        match self.curr_token {
            Token::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }
    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        // prefix
        let mut left = match self.curr_token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Int(_) => self.parse_int_expr(),
            Token::Bang => self.parse_prefix_expr(),
            Token::Minus => self.parse_prefix_expr(),
            _ => None,
        };
        // infix
        while !self.next_token_is(&Token::Semicolon) && precedence < self.next_precedence() {
            match self.next_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Eq
                | Token::NotEq
                | Token::Lt
                | Token::Gt => {
                    self.next_token();
                    left = self.parse_infix_expr(left.unwrap());
                }
                _ => return left,
            }
        }
        left
    }
    fn parse_ident_expr(&mut self) -> Option<Expr> {
        self.parse_ident().map(Expr::Ident)
    }
    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.curr_token {
            Token::Int(int) => Some(Expr::Literal(Literal::Int(int))),
            _ => None,
        }
    }
    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let left = match self.curr_token {
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            _ => return None,
        };
        self.next_token();
        let right = match self.parse_expr(Precedence::Prefix) {
            Some(val) => val,
            _ => return None,
        };

        Some(Expr::Prefix(left, Box::new(right)))
    }
    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.curr_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Divide,
            Token::Asterisk => Infix::Multiply,
            Token::Eq => Infix::Equal,
            Token::NotEq => Infix::NotEqual,
            Token::Lt => Infix::LessThan,
            Token::Gt => Infix::GreaterThan,
            _ => return None,
        };

        let precedence = self.curr_precedence();
        self.next_token();
        self.parse_expr(precedence)
            .map(|expr| Expr::Infix(Box::new(left), infix, Box::new(expr)))
    }
}

fn get_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq | Token::NotEq => Precedence::Equals,
        Token::Lt | Token::Gt => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Slash | Token::Asterisk => Precedence::Product,
        _ => Precedence::Lowest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    fn check_errors(parser: Parser) {
        let errors = parser.get_errors();
        if errors.is_empty() {
            return;
        }
        eprintln!("parser has {} errors", errors.len());
        for msg in errors.iter() {
            eprintln!("parser error: {:?}", msg)
        }
        panic!("Failed")
    }
    #[test]
    fn check_parse_errors() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 123412345;
        "#;
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        check_errors(parser);
        assert_eq!(3, program.len());
        let tests = [
            Statement::Let(Ident(String::from("x")), Expr::Literal(Literal::Int(5))),
            Statement::Let(Ident(String::from("y")), Expr::Literal(Literal::Int(10))),
            Statement::Let(
                Ident(String::from("foobar")),
                Expr::Literal(Literal::Int(123412345)),
            ),
        ];
        for (i, test) in tests.iter().enumerate() {
            let curr = program.get(i).unwrap();
            assert_eq!(curr, test)
        }
    }
    #[test]
    fn test_returns() {
        let input = r#"
        return 5;
        return 10;
        return;
        "#;
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        check_errors(parser);
        assert_eq!(3, program.len());
        let test = [
            Statement::Return(Some(Expr::Literal(Literal::Int(5)))),
            Statement::Return(Some(Expr::Literal(Literal::Int(10)))),
            Statement::Return(None),
        ];
        for (i, st) in program.iter().enumerate() {
            assert_eq!(st, test.get(i).unwrap())
        }
    }
    #[test]
    fn test_expression() {
        let input = "foobar;";
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        check_errors(parser);
        assert_eq!(program.len(), 1);
        assert_eq!(
            Statement::Expr(Expr::Ident(Ident(String::from("foobar")))),
            program[0]
        );
    }
    #[test]
    fn test_prefix() {
        let input = r#"
        !5;
        -15;
        "#;
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        check_errors(parser);
        assert!(!program.is_empty());
        assert_eq!(
            *program.get(0).unwrap(),
            Statement::Expr(Expr::Prefix(
                Prefix::Not,
                Box::new(Expr::Literal(Literal::Int(5)))
            ))
        );
        assert_eq!(
            *program.get(1).unwrap(),
            Statement::Expr(Expr::Prefix(
                Prefix::Minus,
                Box::new(Expr::Literal(Literal::Int(15)))
            ))
        )
    }
    #[test]
    fn test_infix() {
        let input = r#"
        5 + 5;
        5 - 5;
        5 * 5
        5 / 5
        5 > 5
        5 < 5
        5 == 5
        5 != 5
        "#;
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        check_errors(parser);
        let test = [
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::Plus,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::Minus,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::Multiply,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::Divide,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::GreaterThan,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::LessThan,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::Equal,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
            Statement::Expr(Expr::Infix(
                Box::new(Expr::Literal(Literal::Int(5))),
                Infix::NotEqual,
                Box::new(Expr::Literal(Literal::Int(5))),
            )),
        ];
        assert!(!program.is_empty());
        for (i, statement) in program.iter().enumerate() {
            assert_eq!(statement, test.get(i).unwrap())
        }
    }
}
