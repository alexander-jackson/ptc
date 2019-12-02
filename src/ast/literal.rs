use ast::Generate;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}

impl Generate for Literal {
    fn generate(&self) -> String {
        match self {
            Literal::Integer { value } => value.to_string(),
        }
    }
}
