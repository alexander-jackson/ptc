use ast::{Context, DataType, VariableType};
use ast::Literal;

impl DataType for Literal {
    fn get_type(&self, _: &mut Context) -> VariableType {
        match self {
            Literal::Integer { .. } => VariableType::Integer,
        }
    }
}
