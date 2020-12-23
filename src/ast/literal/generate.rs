use crate::ast::{Context, Generate, Literal};

impl Generate for Literal {
    fn generate(&self, _t: &mut Context) -> String {
        match self {
            Literal::Integer { value } => value.to_string(),
            Literal::Float { value } => value.to_string(),
        }
    }
}
