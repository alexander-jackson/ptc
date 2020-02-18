use ast::{Identifier, Literal, Operator};

mod datatype;
mod generate;

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
    ListDisplay,
    FunctionCall {
        name: Box<Expression>,
        args: Option<Vec<Expression>>,
    },
    AttributeRef {
        primary: Box<Expression>,
        attribute: Identifier,
    },
    Subscription {
        primary: Box<Expression>,
        expr: Box<Expression>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Literal,
    },
}
