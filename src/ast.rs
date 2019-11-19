pub type Suite = Vec<Stmt>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub stmts: Suite,
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
    ParenExpr {
        expr: Box<Expression>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Number,
    },
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: u32 },
}
