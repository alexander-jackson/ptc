pub type Suite = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Suite,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        ident: Identifier,
        expr: Expression
    },
    Pass
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equals,
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
        right: Box<Expression>,
    },
    ParenExpr {
        expr: Box<Expression>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Literal,
    },
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}
