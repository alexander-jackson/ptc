use ast::Context;
use ast::Generate;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
}

impl Generate for Identifier {
    fn generate(&self, _t: &mut Context) -> String {
        match self {
            Identifier::Name { name } => name.to_string(),
        }
    }
}
