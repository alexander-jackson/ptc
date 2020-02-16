use std::collections::HashMap;

use regex::Regex;

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

#[derive(Clone, Debug, PartialEq)]
pub enum VariableType {
    Unknown,
    Integer,
    Float,
    Void,
    List { elements: Box<VariableType> },
}

impl From<VariableType> for String {
    fn from(v: VariableType) -> String {
        match v {
            VariableType::Unknown => String::from("error"),
            VariableType::Integer => String::from("int"),
            VariableType::Float => String::from("float"),
            VariableType::Void => String::from("void"),
            VariableType::List { elements } => format!("{}*", String::from(*elements)),
        }
    }
}

impl From<String> for VariableType {
    fn from(s: String) -> VariableType {
        if s == "int" {
            return VariableType::Integer;
        } else if s == "float" {
            return VariableType::Float;
        }

        let re = Regex::new(r"^List\[(.*)\]$").unwrap();

        if let Some(caps) = re.captures(&s) {
            let inner = caps.get(1).unwrap().as_str();

            return VariableType::List {
                elements: Box::new(VariableType::from(String::from(inner))),
            };
        }

        VariableType::Unknown
    }
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

pub trait DataType {
    fn get_type(&self, &mut Context) -> VariableType;
}

#[derive(Debug)]
pub struct Context {
    symbol_table: SymbolTable,
    current_function: Option<String>,
    function_return_types: HashMap<String, VariableType>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            symbol_table: SymbolTable::new(),
            current_function: None,
            function_return_types: HashMap::new(),
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

    pub fn get_type(&self, variable: &str) -> Option<&VariableType> {
        self.symbol_table.get_type(&Variable::new(variable))
    }

    pub fn variable_defined(&self, variable: &str) -> bool {
        self.symbol_table.variable_defined(variable)
    }

    pub fn reset_position(&mut self) {
        self.symbol_table.reset_position();
    }

    pub fn next_scope(&mut self) {
        self.symbol_table.next_scope();
    }

    pub fn display_active_scope(&self) {
        self.symbol_table.display_active_scope();
    }

    pub fn define_variable(&mut self, variable: &str) {
        self.symbol_table.define_variable(variable);
    }

    pub fn set_current_function(&mut self, function_name: Option<String>) {
        self.current_function = function_name;
        self.set_function_return_type(VariableType::Void);
    }

    pub fn set_function_return_type(&mut self, datatype: VariableType) {
        if let Some(f) = &self.current_function {
            self.function_return_types.insert(f.to_string(), datatype);
        }
    }

    pub fn get_function_return_type(&self, function_name: &str) -> Option<&VariableType> {
        self.function_return_types.get(function_name)
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}
