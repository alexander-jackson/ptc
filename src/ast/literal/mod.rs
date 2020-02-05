mod generate;
mod datatype;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}

