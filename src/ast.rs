#[derive(Debug, PartialEq, Eq)]
pub enum Precedence {
    Lowest = 1,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Bang,
    Minus,    
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(Let),
    Return(Return),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(Ident),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Let {
    pub name: Ident,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Return {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ident {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IntegerLiteral {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Prefix {
    pub operator: Operator,
    pub right: Box<Expression>,
}
