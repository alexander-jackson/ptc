mod datatype;
mod generate;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
    Typed { name: String, typehint: String },
}

impl Identifier {
    pub fn get_identifier(&self) -> String {
        match self {
            Identifier::Name { name } => name.clone(),
            Identifier::Typed { name, .. } => name.clone(),
        }
    }
}
