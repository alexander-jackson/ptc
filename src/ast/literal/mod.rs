mod generate;
mod r#type;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}

