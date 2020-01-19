use ast::Context;
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
        args: Option<Vec<Expression>>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Literal,
    },
}

impl Generate for Expression {
    fn generate(&self, context: &mut Context) -> String {
        match self {
            Expression::BinaryOperation { left, op, right } => format!(
                "{} {} {}",
                left.generate(context),
                op.generate(context),
                right.generate(context)
            ),
            Expression::UnaryOperation { op, expr } => {
                format!("{}{}", op.generate(context), expr.generate(context))
            }
            Expression::ParenExpression { expr } => format!("({})", expr.generate(context)),
            Expression::FunctionCall { name, args } => {
                let arg_str: Option<String> = args.as_ref().map(|s| {
                    s.iter()
                        .map(|a| a.generate(context))
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let arg_str = arg_str.unwrap_or_else(|| String::from(""));

                format!("{}({})", name.generate(context), arg_str)
            }
            Expression::Identifier { name } => name.generate(context),
            Expression::Literal { value } => value.generate(context),
        }
    }
}
