#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Not,
    Minus,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Box<Expr>, Infix, Box<Expr>),
}
#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i64),
    String(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Blank,
    Let(Ident, Expr),
    Return(Option<Expr>),
    Expr(Expr),
}

pub type Program = Vec<Statement>;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -x or !x
    Call,        // my_function(x)
}
