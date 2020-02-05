use ast::{Context, Type, VariableType};
use ast::Literal;

impl Type for Literal {
    fn get_type(&self, _: &mut Context) -> VariableType {
        match self {
            Literal::Integer { .. } => VariableType::Integer,
        }
    }
}
