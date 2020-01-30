use ast::{Context, Generate, VariableType};

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
