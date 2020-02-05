mod datatype;
mod generate;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}
