use std::collections::HashMap;

use ast::VariableType;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Variable {
    name: String,
    defined: bool,
}

impl Variable {
    pub fn new(s: &str) -> Variable {
        Variable {
            name: String::from(s),
            defined: false,
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    variables: HashMap<Variable, VariableType>,
    subscopes: Vec<Scope>,
    explored: bool,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
            subscopes: Vec::new(),
            explored: false,
        }
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
    pub fn insert_variable(&mut self, indices: &[usize], variable: Variable, vtype: VariableType) {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].insert_variable(tail, variable, vtype);
        }

        self.variables.insert(variable, vtype);
    }

    /// Insert a variable with a related type into the symbol table at a shallower level than the
    /// current scope.
    pub fn insert_shallow_variable(
        &mut self,
        indices: &[usize],
        variable: &Variable,
        vtype: &VariableType,
    ) -> bool {
        if let Some((head, tail)) = indices.split_first() {
            let found = self.subscopes[*head].insert_shallow_variable(tail, variable, vtype);

            if found {
                return true;
            }
        }

        // Check whether the variable is in this scope
        for key in self.variables.keys() {
            if variable.name == key.name {
                self.variables.insert(variable.clone(), vtype.clone());
                return true;
            }
        }

        false
    }

    /// Get the type of a variable in the symbol table if we know it.
    pub fn get_type(&self, indices: &[usize], variable: &Variable) -> Option<&VariableType> {
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
        for (key, value) in self.variables.iter() {
            if key.name == variable.name {
                return Some(value);
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

        match self.subscopes.iter().position(|s| !s.explored) {
            Some(p) => indices.push(p),
            None => {
                indices.pop();
                self.explored = true;
            }
        }
    }

    /// Check whether a variable has been defined yet in the generated code. This will only return
    /// true if a call to define_variable() has been made in a previous scope or the current one.
    pub fn variable_defined(&self, indices: &[usize], variable: &str) -> bool {
        if let Some((head, tail)) = indices.split_first() {
            // Check whether it is defined in a scope closer to our current position
            let defined = self.subscopes[*head].variable_defined(tail, variable);

            if defined {
                return true;
            }
        }

        // Check if it is defined here
        for key in self.variables.keys() {
            if key.name == variable && key.defined {
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

        // We are in the currently active scope
        let mut v = Variable::new(variable);

        if let Some(vtype) = self.variables.remove(&v) {
            v.defined = true;
            self.variables.insert(v, vtype);
        }
    }

    pub fn get_global_lists(&self) -> Vec<(String, VariableType)> {
        let mut lists: Vec<(String, VariableType)> = Vec::new();

        // Get all the lists in this scope
        for (key, value) in self.variables.iter() {
            if let VariableType::List { .. } = value {
                lists.push((key.name.clone(), value.clone()));
            }
        }

        lists
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scope: Scope,
    active: Vec<usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            scope: Scope::new(),
            active: Vec::new(),
        }
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

    /// Insert an inferred variable type into the SymbolTable.
    pub fn insert_variable(&mut self, variable: Variable, vtype: VariableType) {
        self.scope.insert_variable(&self.active, variable, vtype);
    }

    /// Insert an inferred variable type into the SymbolTable at a shallower level than the current
    /// scope.
    pub fn insert_shallow_variable(&mut self, variable: Variable, vtype: VariableType) {
        self.scope
            .insert_shallow_variable(&self.active, &variable, &vtype);
    }

    /// Get the type of a variable if we have inferred it previously.
    pub fn get_type(&self, variable: &Variable) -> Option<&VariableType> {
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

    pub fn get_global_lists(&self) -> Vec<(String, VariableType)> {
        self.scope.get_global_lists()
    }

    pub fn in_global_scope(&self) -> bool {
        self.active.is_empty()
    }
}
