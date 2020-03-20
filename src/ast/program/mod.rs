use ast::Suite;

mod generate;
mod infer;

#[derive(Debug, PartialEq)]
pub struct Program {
    statements: Suite,
}

impl Program {
    pub fn new(statements: Suite) -> Program {
        Program { statements }
    }
}
