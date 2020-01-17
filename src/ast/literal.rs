use ast::Generate;
use ast::SymbolTable;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}

impl Generate for Literal {
    fn generate(&self, _t: &mut SymbolTable) -> String {
        match self {
            Literal::Integer { value } => value.to_string(),
        }
    }
}
