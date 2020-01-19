use std::collections::HashSet;

pub mod expression;
pub mod identifier;
pub mod literal;
pub mod operator;
pub mod program;
pub mod statement;

use ast::statement::Statement;

pub type Suite = Vec<Statement>;

pub trait Generate {
    fn generate(&self, &mut Context) -> String;
}

impl Generate for Suite {
    fn generate(&self, symbol_table: &mut Context) -> String {
        self.iter()
            .map(|s| s.generate(symbol_table))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub struct Context {
    symbol_table: Vec<HashSet<String>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            symbol_table: vec![HashSet::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.symbol_table.push(HashSet::new());
    }

    pub fn pop_scope(&mut self) {
        self.symbol_table.pop();
    }

    pub fn contains(&self, variable: &str) -> bool {
        self.symbol_table.iter().any(|x| x.contains(variable))
    }

    pub fn insert(&mut self, variable: &str) {
        let index: usize = self.symbol_table.len() - 1;
        self.symbol_table[index].insert(variable.to_string());
    }
}
