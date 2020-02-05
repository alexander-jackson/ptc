use ast::{Operator, Identifier, Literal};

mod generate;
mod datatype;

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
    FunctionCall {
        name: Identifier,
        args: Option<Vec<Expression>>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Literal,
    },
}
