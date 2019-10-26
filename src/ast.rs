#[derive(Debug, PartialEq)]
pub enum Stmt<'input> {
    Statement(Identifier<'input>, Operator, Number),
}

#[derive(Debug, PartialEq)]
pub enum Identifier<'input> {
    Name { name: &'input str },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Assign,
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: u32 },
}

#[derive(Debug, PartialEq)]
pub struct Program<'input> {
    pub stmts: Vec<Stmt<'input>>,
}

impl<'input> Program<'input> {
    pub fn new(stmts: Vec<Stmt<'input>>) -> Self {
        Program { stmts }
    }

    pub fn dump(&self) {
        for ident in self.stmts.iter() {
            println!("{:?}", ident);
        }
    }
}

pub fn number(value: u32) -> Number {
    Number::Integer { value }
}

pub fn identifier<'input>(name: &'input str) -> Identifier<'input> {
    Identifier::Name { name: name }
}
