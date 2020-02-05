use ast::{Context, Generate, Type, VariableType};

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name {
        name: String,
        var_type: Option<VariableType>,
    },
}

impl Generate for Identifier {
    fn generate(&self, _context: &mut Context) -> String {
        match self {
            Identifier::Name { name, .. } => name.to_string(),
        }
    }
}

impl Type for Identifier {
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
