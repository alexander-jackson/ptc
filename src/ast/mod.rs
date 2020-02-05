pub mod expression;
pub mod identifier;
pub mod literal;
pub mod operator;
pub mod program;
pub mod statement;

mod symboltable;

pub use self::{
    expression::Expression, identifier::Identifier, literal::Literal, operator::Operator,
    program::Program, statement::Statement,
};

use self::symboltable::SymbolTable;
use self::symboltable::Variable;

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
    Unknown,
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
    fn get_type(&self, &mut Context) -> VariableType;
}

#[derive(Debug)]
pub struct Context {
    symbol_table: SymbolTable,
}

impl Context {
    pub fn new() -> Context {
        Context {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.symbol_table.push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.symbol_table.pop_scope();
    }

    pub fn contains(&mut self, variable: &str) -> bool {
        self.symbol_table.variable_defined(variable)
    }

    pub fn insert(&mut self, variable: &str) {
        self.symbol_table
            .insert_variable(Variable::new(variable), VariableType::Unknown);
    }

    pub fn insert_inferred_type(&mut self, variable: &str, inferred: VariableType) {
        self.symbol_table
            .insert_variable(Variable::new(variable), inferred);
    }

    pub fn get_type(&mut self, variable: &str) -> Option<&VariableType> {
        self.symbol_table.get_type(&Variable::new(variable))
    }

    pub fn variable_defined(&mut self, variable: &str) -> bool {
        self.symbol_table.variable_defined(variable)
    }

    pub fn reset_position(&mut self) {
        self.symbol_table.reset_position();
    }

    pub fn next_scope(&mut self) {
        self.symbol_table.next_scope();
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}
