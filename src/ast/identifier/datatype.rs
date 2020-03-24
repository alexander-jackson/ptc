use ast::Identifier;
use ast::{Context, DataType, VariableType};

impl DataType for Identifier {
    fn get_type(&self, context: &mut Context) -> Option<VariableType> {
        match self {
            Identifier::Name { name, .. } => {
                // See if we have found a type for the variable already
                if let Some(t) = context.get_type(&name) {
                    return Some(t.clone());
                }

                // We don't know what type the variable is
                None
            }
            Identifier::Typed { typehint, .. } => Some(VariableType::from(typehint.as_str())),
        }
    }
}
