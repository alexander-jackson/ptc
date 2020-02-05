use ast::Suite;

mod generate;
mod infer;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Suite,
}
