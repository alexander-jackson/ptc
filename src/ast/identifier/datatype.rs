use ast::Identifier;
use ast::{Context, DataType, VariableType};

impl DataType for Identifier {
    fn get_type(&self, context: &mut Context) -> VariableType {
        match self {
            Identifier::Name { name, .. } => {
                if let Some(t) = context.get_type(&name) {
                    return *t;
                }

                VariableType::Integer
            }
        }
    }
}
