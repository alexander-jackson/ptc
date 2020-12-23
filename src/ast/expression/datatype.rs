//! Implements the `DataType` trait for `Expression`.

use crate::ast::{Context, DataType, Expression, Generate, Operator, VariableType};

impl DataType for Expression {
    /// Allows attempted calculations of `VariableType`s for an `Expression`.
    fn get_type(&self, context: &mut Context) -> Option<VariableType> {
        match self {
            Expression::BinaryOperation { left, op, right } => {
                let ltype = left.get_type(context);
                let rtype = right.get_type(context);
                op.resulting_type(ltype, rtype)
            }
            Expression::UnaryOperation { op, expr } => match op {
                Operator::Plus | Operator::Minus => expr.get_type(context),
                Operator::LogicalNot => Some(VariableType::Integer),
                _ => unreachable!(),
            },
            Expression::ParenExpression { expr } => expr.get_type(context),
            Expression::ListDisplay => Some(VariableType::List { elements: None }),
            Expression::Literal { value } => value.get_type(context),
            Expression::Identifier { name } => name.get_type(context),
            Expression::FunctionCall { name, .. } => {
                if let Expression::Identifier { name } = &**name {
                    let identifier = name.generate(context);

                    // If we know the return type of the function, insert it here
                    if let Some(t) = context.get_function_return_type(&identifier) {
                        return Some(t.clone());
                    }

                    // `len` is built-in and should return an integer
                    if identifier == "len" {
                        return Some(VariableType::Integer);
                    }
                }

                // If it's not an identifier, we are out of luck
                None
            }
            Expression::Subscription { primary, .. } => {
                let primary_gen = primary.generate(context);

                // If we are doing `list[..]`, we can use the element type of `list`
                if let Some(t) = context.get_type(&primary_gen) {
                    if let VariableType::List { elements } = t {
                        if let Some(elements) = elements {
                            return Some(*elements.clone());
                        }
                    }
                }

                None
            }
            _ => None,
        }
    }
}
