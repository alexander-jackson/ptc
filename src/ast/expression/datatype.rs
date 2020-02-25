use ast::Expression;
use ast::{Context, DataType, Generate, Operator, VariableType};

impl DataType for Expression {
    fn get_type(&self, context: &mut Context) -> VariableType {
        match self {
            Expression::BinaryOperation { left, op, right } => {
                let ltype = left.get_type(context);
                let rtype = right.get_type(context);
                return op.resulting_type(ltype, rtype);
            }
            Expression::UnaryOperation { op, expr } => {
                match op {
                    Operator::Plus => expr.get_type(context),
                    Operator::Minus => expr.get_type(context),
                    Operator::LogicalNot => VariableType::Integer,
                    _ => VariableType::Unknown,
                }
            }
            Expression::ParenExpression { expr } => expr.get_type(context),
            Expression::Literal { value } => value.get_type(context),
            Expression::Identifier { name } => name.get_type(context),
            Expression::FunctionCall { name, .. } => {
                if let Expression::Identifier { name } = &**name {
                    let identifier = name.generate(context);
                    if let Some(t) = context.get_function_return_type(&identifier) {
                        return t.clone();
                    }
                }

                VariableType::Integer
            }
            Expression::Subscription { primary, .. } => {
                let primary_gen = primary.generate(context);

                if let Some(t) = context.get_type(&primary_gen) {
                    if let VariableType::List { elements } = t {
                        return (**elements).clone();
                    }
                }

                VariableType::Unknown
            }
            _ => VariableType::Integer,
        }
    }
}