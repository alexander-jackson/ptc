use ast::Generate;

use ast::operator::Operator;
use ast::identifier::Identifier;
use ast::literal::Literal;

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
        args: Vec<Expression>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Literal,
    },
}

impl Generate for Expression {
    fn generate(&self) -> String {
        match self {
            Expression::Identifier { name } => name.generate(),
            Expression::Literal { value } => value.generate(),
            Expression::BinaryOperation { left, op, right } => {
                format!(
                    "{} {} {}",
                    left.generate(),
                    op.generate(),
                    right.generate(),
                )
            },
            _ => String::from(""),
        }
    }
}
