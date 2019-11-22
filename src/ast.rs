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
    Expression {
        expr: Expression
    },
    Pass,
    IfStatement {
        expr: Expression,
        stmt: Suite,
    },
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    LogicalOr,
    LogicalAnd,
    LogicalNot,
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
    UnaryOperation {
        op: Operator,
        expr: Box<Expression>,
    },
    ParenExpression {
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
