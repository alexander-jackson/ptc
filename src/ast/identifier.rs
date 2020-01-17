use ast::Generate;
use ast::SymbolTable;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
}

impl Generate for Identifier {
    fn generate(&self, _t: &mut SymbolTable) -> String {
        match self {
            Identifier::Name { name } => name.to_string(),
        }
    }
}
