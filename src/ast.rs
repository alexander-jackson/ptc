pub type Suite = Vec<Stmt>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub stmts: Suite,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Program { stmts }
    }

    pub fn dump(&self) {
        for ident in self.stmts.iter() {
            println!("{:?}", ident);
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Statement(Identifier, Operator, Expression),
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOperation {
        left: Box<Expression>,
        op: Operator,
        right: Box<Number>,
    },
    ParenExpr { expr: Box<Expression> },
    Identifier { name: Identifier },
    Literal { value: Number },
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: u32 },
}

pub fn number(value: u32) -> Number {
    Number::Integer { value }
}

pub fn identifier(name: String) -> Identifier {
    Identifier::Name { name: name }
}
