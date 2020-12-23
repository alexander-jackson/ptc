//! Tracks variable names, states and types.
//!
//! Deals with the storage of variables, whether they have been defined and their types. Manages
//! the scope of variables, and allows for types to be inferred across the syntax tree without the
//! need to move around.

use std::collections::HashMap;

use crate::ast::VariableType;

/// Stores information about a variable in the symbol table.
#[derive(Debug)]
pub struct VariableInformation {
    /// Whether the variable has defined in output code yet
    defined: bool,
    /// What type the variable has
    vtype: VariableType,
}

impl VariableInformation {
    /// Creates a new `VariableInformation` with a given datatype and assumes it is undefined.
    pub fn with_type(vtype: VariableType) -> VariableInformation {
        VariableInformation {
            defined: false,
            vtype,
        }
    }
}

/// Stores a single scope layer with the variables defined within it.
#[derive(Debug, Default)]
pub struct Scope {
    /// The variables defined in this scope
    variables: HashMap<String, VariableInformation>,
    /// The scopes created within this one
    subscopes: Vec<Scope>,
    /// Whether this scope has been processed in the code generation yet
    explored: bool,
}

impl Scope {
    /// Creates a new Scope.
    ///
    /// Initialises a new scope with an empty `HashMap` for Variables, no known subscopes and marks
    /// it as unexplored thusfar.
    pub fn new() -> Scope {
        Default::default()
    }

    /// Push a new scope into the symbol table, returning the new index that it was inserted into.
    pub fn push_scope(&mut self, indices: &[usize]) -> usize {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].push_scope(tail);
        }

        self.subscopes.push(Scope::new());
        self.subscopes.len() - 1
    }

    /// Insert a variable with a related type into the symbol table.
    pub fn insert_variable(&mut self, indices: &[usize], variable: &str, vtype: VariableType) {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].insert_variable(tail, variable, vtype);
        }

        let info = VariableInformation::with_type(vtype);
        self.variables.insert(variable.to_string(), info);
    }

    /// Insert a variable with a related type into the symbol table at a shallower level than the
    /// current scope.
    pub fn insert_shallow_variable(
        &mut self,
        indices: &[usize],
        variable: &str,
        vtype: &VariableType,
    ) -> bool {
        if let Some((head, tail)) = indices.split_first() {
            let found = self.subscopes[*head].insert_shallow_variable(tail, variable, vtype);

            if found {
                return true;
            }
        }

        // Check whether the variable is in this scope
        if let Some(info) = self.variables.get_mut(variable) {
            info.vtype = vtype.clone();
            return true;
        }

        false
    }

    /// Get the type of a variable in the symbol table if we know it.
    pub fn get_type(&self, indices: &[usize], variable: &str) -> Option<&VariableType> {
        if let Some((head, tail)) = indices.split_first() {
            let vtype = self.subscopes[*head].get_type(tail, variable);

            if let Some(v) = vtype {
                return Some(v);
            }
        }

        // Either
        //      We are the final scope
        //      The inner scopes returned nothing
        // Thus, check whether we contain <variable>
        for (name, info) in &self.variables {
            if name == variable {
                return Some(&info.vtype);
            }
        }

        None
    }

    /// Move us into the next scope in the depth first traversal of the scope tree.
    pub fn next_scope(&mut self, level: usize, indices: &mut Vec<usize>) {
        if let Some(i) = indices.get(level) {
            return self.subscopes[*i].next_scope(level + 1, indices);
        }

        // We have now reached the deepest point so far
        //
        // If there is an available subscope, move into it
        // Otherwise, backtrack
        if let Some(p) = self.subscopes.iter().position(|s| !s.explored) {
            indices.push(p);
        } else {
            indices.pop();
            self.explored = true;
        }
    }

    /// Check whether a variable has been defined yet in the generated code. This will only return
    /// true if a call to `define_variable`() has been made in a previous scope or the current one.
    pub fn variable_defined(&self, indices: &[usize], variable: &str) -> bool {
        if let Some((head, tail)) = indices.split_first() {
            // Check whether it is defined in a scope closer to our current position
            let defined = self.subscopes[*head].variable_defined(tail, variable);

            if defined {
                return true;
            }
        }

        // Check if it is defined here
        for (name, info) in &self.variables {
            if name == variable && info.defined {
                return true;
            }
        }

        false
    }

    /// Define a variable in the currently active scope if it exists within it.
    pub fn define_variable(&mut self, indices: &[usize], variable: &str) {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].define_variable(tail, variable);
        }

        if let Some(info) = self.variables.get_mut(variable) {
            info.defined = true;
        }
    }

    /// Gets the lists that are defined in this scope.
    pub fn get_defined_lists(&self) -> Vec<(String, VariableType)> {
        let mut lists: Vec<(String, VariableType)> = Vec::new();

        // Get all the lists in this scope
        for (name, info) in &self.variables {
            if let VariableType::List { .. } = info.vtype {
                lists.push((name.clone(), info.vtype.clone()));
            }
        }

        lists
    }
}

/// Defines the `SymbolTable` used in the Context.
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// The global scope, which all scopes descend from
    scope: Scope,
    /// The index path to the scope we are currently in
    active: Vec<usize>,
}

impl SymbolTable {
    /// Creates a new `SymbolTable` with an empty global scope.
    pub fn new() -> SymbolTable {
        Default::default()
    }

    /// Push a new scope into the currently active one. This implies we are going into a new level
    /// of nesting, such as a function definition, if statement or while statement.
    pub fn push_scope(&mut self) {
        let index: usize = self.scope.push_scope(&self.active);
        self.active.push(index);
    }

    /// Pop a scope from our active ones. Implies we are finished with that scope and have done
    /// everything we need to in it.
    pub fn pop_scope(&mut self) {
        self.active.pop();
    }

    /// Insert an inferred variable type into the `SymbolTable`.
    pub fn insert_variable(&mut self, variable: &str, vtype: VariableType) {
        self.scope.insert_variable(&self.active, variable, vtype);
    }

    /// Insert an inferred variable type into the `SymbolTable` at a shallower level than the
    /// current scope.
    pub fn insert_shallow_variable(&mut self, variable: &str, vtype: &VariableType) {
        self.scope
            .insert_shallow_variable(&self.active, variable, &vtype);
    }

    /// Get the type of a variable if we have inferred it previously.
    pub fn get_type(&self, variable: &str) -> Option<&VariableType> {
        self.scope.get_type(&self.active, variable)
    }

    /// Reset the active position of the table after inference has been performed.
    pub fn reset_position(&mut self) {
        self.active.clear();
    }

    /// Move us into the next scope in the depth first traversal of the tree.
    pub fn next_scope(&mut self) {
        self.scope.next_scope(0, &mut self.active);
    }

    /// Check whether a variable has already been defined in the output code.
    pub fn variable_defined(&self, variable: &str) -> bool {
        self.scope.variable_defined(&self.active, variable)
    }

    /// State that we have now defined a variable in the output code.
    pub fn define_variable(&mut self, variable: &str) {
        self.scope.define_variable(&self.active, variable);
    }

    /// Gets the names of all global lists and their `VariableType`s.
    pub fn get_global_lists(&self) -> Vec<(String, VariableType)> {
        self.scope.get_defined_lists()
    }

    /// Checks whether we are in the global scope.
    pub fn in_global_scope(&self) -> bool {
        self.active.is_empty()
    }
}
