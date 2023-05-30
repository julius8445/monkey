#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
}

#[derive(Debug)]
pub struct Let {
    pub name: Ident,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Return {
    pub value: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Ident(Ident),
}

#[derive(Debug)]
pub struct Ident {
    pub value: String,
}
