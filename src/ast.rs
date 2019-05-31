use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Assignment(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Statement>, Vec<Box<Statement>>, Option<Box<Statement>>),
    ElIf(Box<Expression>, Box<Statement>),
    While(Box<Expression>, Box<Statement>),
    Block(Vec<Box<Statement>>),
    Break,
}

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression)
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Token),
    Integer(Token),
    Float(Token),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),

    Boolean(Token),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Negate(Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>)
}