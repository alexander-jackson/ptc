use std::collections::HashMap;
use std::collections::HashSet;

pub mod expression;
pub mod identifier;
pub mod literal;
pub mod operator;
pub mod program;
pub mod statement;

pub use self::{
    expression::Expression, identifier::Identifier, literal::Literal, operator::Operator,
    program::Program, statement::Statement,
};

pub type Suite = Vec<Statement>;

pub trait Generate {
    fn generate(&self, &mut Context) -> String;
}

impl Generate for Suite {
    fn generate(&self, context: &mut Context) -> String {
        self.iter()
            .map(|s| s.generate(context))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VariableType {
    Integer,
}

pub trait Infer {
    fn infer(&mut self, &mut Context);
}

impl Infer for Suite {
    fn infer(&mut self, context: &mut Context) {
        for stmt in self {
            stmt.infer(context);
        }
    }
}

pub trait Type {
    fn get_type(&self) -> VariableType;
}

#[derive(Debug)]
pub struct Context {
    symbol_table: Vec<HashMap<String, Option<VariableType>>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            symbol_table: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.symbol_table.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.symbol_table.pop();
    }

    pub fn contains(&self, variable: &str) -> bool {
        self.symbol_table.iter().any(|x| x.contains_key(variable))
    }

    pub fn insert(&mut self, variable: &str) {
        let index: usize = self.symbol_table.len() - 1;
        self.symbol_table[index].insert(variable.to_string(), None);
    }

    pub fn insert_inferred_type(&mut self, variable: &str, inferred: VariableType) {
        let index: usize = self.symbol_table.len() - 1;
        self.symbol_table[index].insert(variable.to_string(), Some(inferred));
    }

    pub fn get_type(&self, variable: &str) -> Option<VariableType> {
        for m in self.symbol_table.iter().rev() {
            if let Some(v) = m.get(variable) {
                return *v;
            }
        }

        None
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}
