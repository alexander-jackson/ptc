use ast::Identifier;
use ast::{Context, Generate};

impl Generate for Identifier {
    fn generate(&self, _context: &mut Context) -> String {
        match self {
            Identifier::Name { name, .. } => name.to_string(),
        }
    }
}
