use std::collections::HashSet;

pub mod expression;
pub mod identifier;
pub mod literal;
pub mod operator;
pub mod program;
pub mod statement;

use ast::statement::Statement;

pub type Suite = Vec<Statement>;
pub type SymbolTable = HashSet<String>;

pub trait Generate {
    fn generate(&self, &mut SymbolTable) -> String;
}

impl Generate for Suite {
    fn generate(&self, symbol_table: &mut SymbolTable) -> String {
        self.iter()
            .map(|s| s.generate(symbol_table))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
