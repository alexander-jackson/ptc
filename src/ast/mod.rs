pub mod expression;
pub mod identifier;
pub mod literal;
pub mod operator;
pub mod program;
pub mod statement;

use ast::statement::Statement;

pub type Suite = Vec<Statement>;

pub trait Generate {
    fn generate(&self) -> String;
}

impl Generate for Suite {
    fn generate(&self) -> String {
        self.iter().map(|s| s.generate()).collect::<Vec<String>>().join(" ")
    }
}
