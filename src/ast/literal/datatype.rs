use ast::Literal;
use ast::{Context, DataType, VariableType};

impl DataType for Literal {
    fn get_type(&self, _: &mut Context) -> VariableType {
        match self {
            Literal::Integer { .. } => VariableType::Integer,
            Literal::Float { .. } => VariableType::Float,
        }
    }
}
