use ast::{Context, Generate, Type, VariableType};

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}

impl Generate for Literal {
    fn generate(&self, _t: &mut Context) -> String {
        match self {
            Literal::Integer { value } => value.to_string(),
        }
    }
}

impl Type for Literal {
    fn get_type(&self, _: &mut Context) -> VariableType {
        match self {
            Literal::Integer { .. } => VariableType::Integer,
        }
    }
}
