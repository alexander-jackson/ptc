//! The Abstract Syntax Tree produced by the compiler.
//!
//! The AST represents the program syntax at a more abstract level, removing some of the finer
//! details. The parser produces an AST when it is executed, and the AST understands things like
//! type inference, expression types and code generation.
//!
//! Each of the AST nodes are stored in their own module. Each one may implement a different subset
//! of the traits found in this module. They are all likely to use the `Context` struct, which
//! allows for inference to be carried across from the `infer` call to the `generate` call.

use std::collections::HashMap;
use std::collections::HashSet;

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

/// Suites are generally used as a name for any number of continuous statements in the Python
/// grammar
pub type Suite = Vec<Statement>;

/// Allows AST nodes to generate transpiled code for themselves.
///
/// Denotes that an AST node can produce equivalent C code given the currently known context for
/// the program.
pub trait Generate {
    /// Generates a string representation of the current AST node in C.
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

/// Variable types that `ptc` currently supports.
#[derive(Clone, Debug, PartialEq)]
pub enum VariableType {
    /// A type we don't know about
    Unknown,
    /// The integer type
    Integer,
    /// The float type
    Float,
    /// The void type, only used for function returns
    Void,
    /// A list of homogeneous types
    List {
        /// The type of each element in the list
        elements: Box<VariableType>,
    },
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

/// Propagates type inference down the AST and allows for types to be inferred on the current
/// object if information is known.
///
/// Nodes such as function declarations can infer the types of their arguments when they are
/// reached and then propagate the inference into the body of the definition.
pub trait Infer {
    /// Performs the inferrence, taking the currently known context of the program.
    fn infer(&mut self, &mut Context);
}

impl Infer for Suite {
    fn infer(&mut self, context: &mut Context) {
        for stmt in self {
            stmt.infer(context);
        }
    }
}

/// Denotes that an AST node has a type.
///
/// Nodes such as expressions have a type which might be able to be worked out from the operations
/// performed within it. In statements such as assignments or returns, it is useful to be able to
/// get the type of the argument to perform inference.
///
/// # Examples
///
/// ```
/// use ptc::ast::{Context, DataType, Literal, VariableType};
///
/// let node = Literal::Float { value: 0.5 };
/// let mut context = Context::new();
/// assert_eq!(node.get_type(&mut context), VariableType::Float);
/// ```
pub trait DataType {
    /// Gets the type of the current node, using the known context if needed.
    fn get_type(&self, &mut Context) -> VariableType;
}

/// A structure for storing information learnt about the program provided.
///
/// Stores a SymbolTable object, which allows for variables to have information stored about them,
/// such as whether they have been initialised or not and what type they have.
///
/// Stores the current function name if we are inside one, the return types of functions, their
/// argument names and types, and any external `#include`s that are needed.
#[derive(Debug)]
pub struct Context {
    /// Stores the currently defined variables and deals with scoping rules
    symbol_table: SymbolTable,
    /// The current function definition we are in
    current_function: Option<String>,
    /// The return types of functions that we know
    function_return_types: HashMap<String, VariableType>,
    /// The argument types of each parameter to a function
    function_argument_types: HashMap<String, Vec<Option<VariableType>>>,
    /// The argument names of each parameter to a function
    function_argument_names: HashMap<String, Vec<String>>,
    /// The files that should be `#include`d to the source file
    includes: HashSet<String>,
}

impl Context {
    /// Creates a new Context.
    ///
    /// This assumes that nothing has been learnt about the program yet.
    pub fn new() -> Context {
        Context {
            symbol_table: SymbolTable::new(),
            current_function: None,
            function_return_types: HashMap::new(),
            function_argument_types: HashMap::new(),
            function_argument_names: HashMap::new(),
            includes: HashSet::new(),
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

    /// Insert the inferred type for a variable into the SymbolTable at a shallower level than the
    /// current scope.
    pub fn insert_shallow_inferred_type(&mut self, variable: &str, inferred: VariableType) {
        self.symbol_table
            .insert_shallow_variable(Variable::new(variable), inferred);
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

    /// Get the argument names of a given function after we have seen the function.
    pub fn get_function_argument_names(&self, function_name: &str) -> Option<&Vec<String>> {
        self.function_argument_names.get(function_name)
    }

    /// Set the argument name of a given function based on the index it occurred at in the function
    /// definition.
    pub fn set_function_argument_name(
        &mut self,
        function_name: &str,
        pos: usize,
        argument_name: &str,
    ) {
        match self.function_argument_names.get_mut(function_name) {
            Some(v) => {
                // Make sure we resize the vector so it definitely has this index
                v.resize(pos + 1, String::new());
                v[pos] = argument_name.to_string();
            }
            None => {
                // Create a vector with enough space
                let mut v: Vec<String> = Vec::new();
                v.resize(pos + 1, String::new());
                v[pos] = argument_name.to_string();
                self.function_argument_names
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

    /// Generates the header file for the current source file.
    pub fn generate_header_file(&self) -> String {
        let mut prototypes: Vec<String> = Vec::new();

        // function_return_types contains all functions we saw
        for (name, return_type) in self.function_return_types.iter() {
            let (types, names) = (
                self.get_function_argument_types(name),
                self.get_function_argument_names(name),
            );

            let arguments = match (types, names) {
                (Some(types), Some(names)) => {
                    // We have both types and names
                    types
                        .iter()
                        .zip(names.iter())
                        .map(|(t, n)| match t {
                            Some(t) => format!("{} {}", String::from(t.clone()), n),
                            None => format!("{} {}", String::from(VariableType::Void), n),
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                }
                _ => String::new(),
            };

            let rtype = match return_type {
                VariableType::Unknown => String::from(VariableType::Void),
                _ => String::from(return_type.clone()),
            };

            let prototype = format!("{} {}({});", rtype, name, arguments);
            prototypes.push(prototype);
        }

        prototypes.join("\n")
    }

    /// Adds the name of a file that should be included in the output source.
    pub fn add_include(&mut self, include: &str) {
        self.includes.insert(include.to_string());
    }

    /// Generates the include statements for the current file.
    pub fn generate_includes(&self) -> String {
        self.includes
            .iter()
            .map(|i| format!(r#"#include "{}""#, i))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Generates the global list initialiser function.
    ///
    /// This is used so that globally defined lists in the Python source can be properly
    /// initialised and used in the output C code. Lists in C cannot be initialised globally as the
    /// function to initialise them is not `const`.
    pub fn generate_global_list_initialiser(&self) -> Option<String> {
        let global_lists = self.symbol_table.get_global_lists();

        if global_lists.is_empty() {
            return None;
        }

        // For each list type, generate its relevant function
        let initialisers = global_lists
            .iter()
            .map(|(name, vtype)| {
                format!(
                    "{} = {};",
                    name,
                    match vtype {
                        VariableType::List { elements } => {
                            match **elements {
                                VariableType::Integer => "list_int_new()",
                                VariableType::Float => "list_float_new()",
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                )
            })
            .collect::<Vec<String>>()
            .join(" ");

        // Format the function for output
        let return_type = "void";
        let name = "initialise_global_lists";
        Some(format!("{} {}() {{ {} }}", return_type, name, initialisers))
    }

    /// Checks whether we are currently in the global scope of the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use ptc::ast::Context;
    ///
    /// let mut context = Context::new();
    /// assert!(context.in_global_scope());
    ///
    /// context.push_scope();
    /// assert!(!context.in_global_scope());
    ///
    /// context.pop_scope();
    /// assert!(context.in_global_scope());
    /// ```
    pub fn in_global_scope(&self) -> bool {
        self.symbol_table.in_global_scope()
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}
