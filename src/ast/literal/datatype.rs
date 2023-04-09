use crate::ast::Literal;
use crate::ast::{Context, DataType, VariableType};

impl DataType for Literal {
    fn get_type(&self, _: &mut Context) -> Option<VariableType> {
        match self {
            Literal::Integer { .. } => Some(VariableType::Integer),
            Literal::Float { .. } => Some(VariableType::Float),
        }
    }
}
