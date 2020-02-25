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
            VariableType::Unknown => String::from("unknown"),
            VariableType::Integer => String::from("int"),
            VariableType::Float => String::from("float"),
            VariableType::Void => String::from("void"),
            VariableType::List { elements } => format!("list_{}*", String::from(*elements)),
        }
    }
}

impl From<String> for VariableType {
    fn from(s: String) -> VariableType {
        if s == "int" {
            return VariableType::Integer;
        } else if s == "float" {
            return VariableType::Float;
        } else if s == "None" {
            // Used for function return typehints
            return VariableType::Void;
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
    function_argument_types: HashMap<String, Vec<Option<VariableType>>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            symbol_table: SymbolTable::new(),
            current_function: None,
            function_return_types: HashMap::new(),
            function_argument_types: HashMap::new(),
        }
    }

    /// Push a new scope into the SymbolTable.
    pub fn push_scope(&mut self) {
        self.symbol_table.push_scope();
    }

    /// Pop a scope from the SymbolTable.
    pub fn pop_scope(&mut self) {
        self.symbol_table.pop_scope();
    }

    /// Insert the inferred type for a variable into the SymbolTable.
    pub fn insert_inferred_type(&mut self, variable: &str, inferred: VariableType) {
        self.symbol_table
            .insert_variable(Variable::new(variable), inferred);
    }

    /// Get the VariableType for a variable if it exists.
    pub fn get_type(&self, variable: &str) -> Option<&VariableType> {
        self.symbol_table.get_type(&Variable::new(variable))
    }

    /// Check whether a variable has been defined in the SymbolTable currently.
    pub fn variable_defined(&self, variable: &str) -> bool {
        self.symbol_table.variable_defined(variable)
    }

    /// Reset the position of the SymbolTable.
    pub fn reset_position(&mut self) {
        self.symbol_table.reset_position();
    }

    /// Move us into the next scope in the depth first traversal of the scopes.
    pub fn next_scope(&mut self) {
        self.symbol_table.next_scope();
    }

    /// Mark a variable as defined in the symbol table.
    pub fn define_variable(&mut self, variable: &str) {
        self.symbol_table.define_variable(variable);
    }

    /// Set the current function that we are parsing and generating code for.
    pub fn set_current_function(&mut self, function_name: Option<String>) {
        self.current_function = function_name;
        self.set_function_return_type(VariableType::Unknown);
    }

    /// Set the return type for the current function.
    pub fn set_function_return_type(&mut self, datatype: VariableType) {
        if let Some(f) = &self.current_function {
            let current = self.get_function_return_type(f);

            if current.is_none() {
                self.function_return_types.insert(f.to_string(), datatype);
            } else if let Some(VariableType::Unknown) = current {
                self.function_return_types.insert(f.to_string(), datatype);
            }
        }
    }

    /// Check whether we know the return type for a function call.
    pub fn get_function_return_type(&self, function_name: &str) -> Option<&VariableType> {
        self.function_return_types.get(function_name)
    }

    /// Set the argument type of a given function based on the index it occurred at in the function
    /// call.
    pub fn set_function_argument_type(
        &mut self,
        function_name: &str,
        pos: usize,
        datatype: VariableType,
    ) {
        match self.function_argument_types.get_mut(function_name) {
            Some(v) => {
                // Make sure we resize the vector so it definitely has this index
                v.resize(pos + 1, None);
                v[pos] = Some(datatype);
            }
            None => {
                // Create a vector with enough space
                let mut v: Vec<Option<VariableType>> = Vec::new();
                v.resize(pos + 1, None);
                v[pos] = Some(datatype);
                self.function_argument_types
                    .insert(String::from(function_name), v);
            }
        }
    }

    /// Get the argument types of a given function after we have inferred them previously.
    pub fn get_function_argument_types(
        &self,
        function_name: &str,
    ) -> Option<&Vec<Option<VariableType>>> {
        self.function_argument_types.get(function_name)
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}
