use std::collections::HashMap;

use ast::VariableType;

#[derive(Debug, Eq, Hash, PartialEq)]
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

    pub fn push_scope(&mut self, indices: &[usize]) -> usize {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].push_scope(tail);
        }

        self.subscopes.push(Scope::new());
        self.subscopes.len() - 1
    }

    pub fn insert_variable(&mut self, indices: &[usize], variable: Variable, vtype: VariableType) {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].insert_variable(tail, variable, vtype);
        }

        self.variables.insert(variable, vtype);
    }

    pub fn get_type(&self, indices: &[usize], variable: &Variable) -> Option<&VariableType> {
        if let Some((head, tail)) = indices.split_first() {
            let vtype = self.subscopes[*head].get_type(tail, variable);

            match vtype {
                Some(v) => return Some(v),
                None => (),
            }
        }

        // Either
        //      We are the final scope
        //      The inner scopes returned nothing
        // Thus, check whether we contain <variable>
        self.variables.get(&variable)
    }

    pub fn display_active_scope(&self, indices: &[usize]) {
        if let Some((head, tail)) = indices.split_first() {
            return self.subscopes[*head].display_active_scope(tail);
        }

        dbg!(&self);
    }

    pub fn next_scope(&mut self, level: usize, indices: &mut Vec<usize>) {
        if let Some(i) = indices.get(level) {
            return self.subscopes[indices[*i]].next_scope(level + 1, indices);
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

    pub fn variable_defined(&mut self, indices: &[usize], variable: &str) -> bool {
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

        let mut obj: Variable = Variable::new(variable);

        if let Some(vtype) = self.variables.remove(&obj) {
            obj.defined = true;
            self.variables.insert(obj, vtype);
        }

        return false;
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

    pub fn push_scope(&mut self) {
        let index: usize = self.scope.push_scope(&self.active);
        self.active.push(index);
    }

    pub fn pop_scope(&mut self) {
        self.active.pop();
    }

    pub fn insert_variable(&mut self, variable: Variable, vtype: VariableType) {
        self.scope.insert_variable(&self.active, variable, vtype);
    }

    pub fn get_type(&mut self, variable: &Variable) -> Option<&VariableType> {
        self.scope.get_type(&self.active, variable)
    }

    pub fn reset_position(&mut self) {
        self.active.clear();
    }

    pub fn display_active_scope(&self) {
        self.scope.display_active_scope(&self.active);
    }

    pub fn next_scope(&mut self) {
        self.scope.next_scope(0, &mut self.active);
    }

    pub fn variable_defined(&mut self, variable: &str) -> bool {
        self.scope.variable_defined(&self.active, variable)
    }
}
