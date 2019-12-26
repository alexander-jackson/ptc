use ast::Generate;

use ast::identifier::Identifier;
use ast::literal::Literal;
use ast::operator::Operator;

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
            Expression::BinaryOperation { left, op, right } => {
                format!("{} {} {}", left.generate(), op.generate(), right.generate())
            }
            Expression::UnaryOperation { op, expr } => {
                format!("{}{}", op.generate(), expr.generate())
            }
            Expression::ParenExpression { expr } => format!("({})", expr.generate()),
            Expression::FunctionCall { name, args } => {
                let arg_str: String = args
                    .iter()
                    .map(|a| a.generate())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", name.generate(), arg_str)
            }
            Expression::Identifier { name } => name.generate(),
            Expression::Literal { value } => value.generate(),
        }
    }
}
