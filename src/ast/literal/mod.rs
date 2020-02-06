mod datatype;
mod generate;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
    Float { value: f32 },
}
