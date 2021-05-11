//! Contains the Context structure for storing information about the program.
//!
//! Aims to behave similarly to the LLVM idea of context, where it stores variable types, function
//! arguments, function names, function return types and other information learnt from inference
//! that can then be used in the calls to `generate`.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::ast::{SymbolTable, VariableType};

/// A structure for storing information learnt about the program provided.
///
/// Stores a `SymbolTable` object, which allows for variables to have information stored about
/// them, such as whether they have been initialised or not and what type they have.
///
/// Stores the current function name if we are inside one, the return types of functions, their
/// argument names and types, and any external `#include`s that are needed.
#[derive(Debug, Default)]
pub struct Context {
    /// Stores the currently defined variables and deals with scoping rules
    symbol_table: SymbolTable,
    /// The current function definition we are in
    current_function: Option<String>,
    /// The return types of functions that we know
    function_return_types: HashMap<String, Option<VariableType>>,
    /// The argument types of each parameter to a function
    function_argument_types: HashMap<String, Vec<Option<VariableType>>>,
    /// The argument names of each parameter to a function
    function_argument_names: HashMap<String, Vec<String>>,
    /// The files that should be `#include`d to the source file
    includes: HashSet<String>,
    /// The files that should be `#include`d to the header file
    header_includes: HashSet<String>,
}

impl Context {
    /// Creates a new Context.
    ///
    /// This assumes that nothing has been learnt about the program yet.
    pub fn new() -> Context {
        Context::default()
    }

    /// Push a new scope into the `SymbolTable`.
    pub fn push_scope(&mut self) {
        self.symbol_table.push_scope();
    }

    /// Pop a scope from the `SymbolTable`.
    pub fn pop_scope(&mut self) {
        self.symbol_table.pop_scope();
    }

    /// Insert the inferred type for a variable into the `SymbolTable`.
    pub fn insert_inferred_type(&mut self, variable: &str, inferred: VariableType) {
        self.symbol_table.insert_variable(variable, inferred);
    }

    /// Insert the inferred type for a variable into the `SymbolTable` at a shallower level than
    /// the current scope.
    pub fn insert_shallow_inferred_type(&mut self, variable: &str, inferred: &VariableType) {
        // If we already have a "stronger" type inferred, ignore this one
        if let Some(VariableType::List { elements: Some(_) }) = self.get_type(variable) {
            return;
        }

        self.symbol_table
            .insert_shallow_variable(variable, &inferred);
    }

    /// Get the [`VariableType`] for a variable if it exists.
    pub fn get_type(&self, variable: &str) -> Option<&VariableType> {
        self.symbol_table.get_type(variable)
    }

    /// Check whether a variable has been defined in the [`SymbolTable`] currently.
    pub fn variable_defined(&self, variable: &str) -> bool {
        self.symbol_table.variable_defined(variable)
    }

    /// Reset the position of the [`SymbolTable`].
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
    pub fn set_current_function(&mut self, function_name: impl Into<Option<String>>) {
        self.current_function = function_name.into();

        if let Some(f) = &self.current_function {
            self.function_return_types.insert(f.to_string(), None);
        }
    }

    /// Set the return type for the current function.
    pub fn set_function_return_type(&mut self, datatype: VariableType) {
        if let Some(f) = &self.current_function {
            let current = self.get_function_return_type(f);

            // If we don't already know the function return type
            // This causes us to ignore any inference after the first
            // For def func() -> TYPE this makes sense as we should always use TYPE
            if current.is_none() {
                self.function_return_types
                    .insert(f.to_string(), Some(datatype));
            }
        }
    }

    /// Check whether we know the return type for a function call.
    pub fn get_function_return_type(&self, function_name: &str) -> Option<&VariableType> {
        self.function_return_types
            .get(function_name)
            .and_then(Option::as_ref)
    }

    /// Set the argument type of a given function based on the index it occurred at in the function
    /// call.
    pub fn set_function_argument_type(
        &mut self,
        function_name: &str,
        pos: usize,
        datatype: VariableType,
    ) {
        let entry = self
            .function_argument_types
            .entry(String::from(function_name))
            .or_default();

        entry.resize_with(pos + 1, Default::default);
        entry[pos] = Some(datatype);
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
        let entry = self
            .function_argument_names
            .entry(String::from(function_name))
            .or_default();

        entry.resize_with(pos + 1, Default::default);
        entry[pos] = argument_name.to_string();
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
        let mut header_lines: Vec<String> = Vec::new();

        // Sort the #include files for consistency
        let mut sorted = self.header_includes.iter().collect::<Vec<&String>>();
        sorted.sort();

        // Add all the #include files to the header file
        for include in sorted {
            header_lines.push(format!(r#"#include "{}""#, include));
        }

        // Iterate the names and return types of all the functions we saw defined
        for (name, return_type) in &self.function_return_types {
            // Get the argument types and names if possible
            let (types, names) = (
                self.get_function_argument_types(name),
                self.get_function_argument_names(name),
            );

            let arguments = types
                .zip(names)
                .map(|(types, names)| {
                    types
                        .iter()
                        .map(|t| {
                            t.as_ref()
                                .map_or_else(|| VariableType::Void.to_string(), ToString::to_string)
                        })
                        .zip(names.iter())
                        .map(|(t, n)| format!("{} {}", t, n))
                        .join(", ")
                })
                .unwrap_or_default();

            // Check whether we have a return type, assuming void otherwise
            let rtype = return_type
                .as_ref()
                .map_or_else(|| VariableType::Void.to_string(), ToString::to_string);

            // Format this function and add it to the file
            let prototype = format!("{} {}({});", rtype, name, arguments);
            header_lines.push(prototype);
        }

        // Join all the lines with newlines
        header_lines.join("\n")
    }

    /// Adds the name of a file that should be included in the output source.
    pub fn add_include(&mut self, include: &str) {
        self.includes.insert(include.to_string());
    }

    /// Adds the name of a file that should be included in the output header.
    pub fn add_header_include(&mut self, include: &str) {
        self.header_includes.insert(include.to_string());
    }

    /// Generates the include statements for the current file.
    pub fn generate_includes(&self) -> String {
        // Sorts the includes for consistency
        let mut sorted = self.includes.iter().collect::<Vec<&String>>();
        sorted.sort();

        sorted
            .iter()
            .map(|i| format!(r#"#include "{}""#, i))
            .join("\n")
    }

    /// Generates the global list initialiser function.
    ///
    /// This is used so that globally defined lists in the Python source can be properly
    /// initialised and used in the output C code. Lists in C cannot be initialised globally as the
    /// function to initialise them is not `const`.
    pub fn generate_global_list_initialiser(&self) -> Option<String> {
        // Get all the globally defined lists
        let global_lists = self.symbol_table.get_global_lists();

        // If none were defined, we do not need this function
        if global_lists.is_empty() {
            return None;
        }

        // Gets the appropriate version of list_*_new for a variable type
        let get_list_new = |v: &VariableType| match v {
            VariableType::List { elements } => match elements {
                Some(t) => match **t {
                    VariableType::Integer => "list_int_new()",
                    VariableType::Float => "list_float_new()",
                    _ => unreachable!(),
                },
                None => "list_unknown_new()",
            },
            _ => unreachable!(),
        };

        // For each list type, generate its relevant function
        let initialisers = global_lists
            .iter()
            .map(|(name, vtype)| format!("{} = {};", name, get_list_new(vtype)))
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
