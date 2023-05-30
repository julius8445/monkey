#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(Let),
    Return(Return),
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
pub enum Expression {
    Ident(Ident),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ident {
    pub value: String,
}
