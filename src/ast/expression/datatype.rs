use ast::Expression;
use ast::{Context, DataType, VariableType};

impl DataType for Expression {
    fn get_type(&self, context: &mut Context) -> VariableType {
        match self {
            Expression::Literal { value } => value.get_type(context),
            Expression::Identifier { name } => name.get_type(context),
            _ => VariableType::Integer,
        }
    }
}
