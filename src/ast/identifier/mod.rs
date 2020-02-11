mod datatype;
mod generate;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
    Typed { name: String, typehint: String },
}
