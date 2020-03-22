use ast::Identifier;
use ast::{Context, DataType, VariableType};

impl DataType for Identifier {
    fn get_type(&self, context: &mut Context) -> VariableType {
        match self {
            Identifier::Name { name, .. } => {
                // See if we have found a type for the variable already
                if let Some(t) = context.get_type(&name) {
                    return t.clone();
                }

                // We don't know what type the variable is
                VariableType::Unknown
            }
            Identifier::Typed { typehint, .. } => VariableType::from(typehint.as_str()),
        }
    }
}
