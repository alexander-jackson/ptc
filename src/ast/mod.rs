pub mod program;
pub mod statement;
pub mod expression;
pub mod operator;
pub mod identifier;
pub mod literal;

use ast::statement::Statement;

pub type Suite = Vec<Statement>;

pub trait Generate {
    fn generate(&self) -> String;
}