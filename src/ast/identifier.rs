use ast::Context;
use ast::Generate;
use ast::VariableType;

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
            Identifier::Name { name, var_type: _ } => name.to_string(),
        }
    }
}
