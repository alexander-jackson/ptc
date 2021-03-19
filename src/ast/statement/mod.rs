//! A Statement used in the Python code.
//!
//! Statements can be anything from control flow, loops and function declarations to expressions
//! and variable modifications. Statements can be simple or compound variants, meaning they can
//! either be basic on their own (such as Assign) or contain other statements (such as If, While).
//!
//! Simple statements comprise a single logical line, whereas Compound statements consist of a
//! 'header' and a `Suite`, usually starting a new indentation level after the header.
//!
//! Statements can propagate inference as well, through the bodies of compound statements in
//! expression statements.

use crate::ast::{Expression, Identifier, Operator, Suite};

mod generate;
mod infer;

/// An effective branch instruction, which checks whether a condition holds and executes some
/// statements if it does.
#[derive(Debug)]
pub struct Branch {
    /// The condition to check.
    pub condition: Expression,
    /// The code to execute if true.
    pub block: Suite,
}

impl Branch {
    /// Creates a new Branch struct containing an expression and a list of statements.
    pub fn new(condition: Expression, block: Suite) -> Branch {
        Branch { condition, block }
    }
}

/// A Statement used in the Python code.
#[derive(Debug)]
pub enum Statement {
    /// An assignment, such as `x = 0`
    Assign {
        /// The target of the assignment
        target: Expression,
        /// The value to assign
        expr: Expression,
    },
    /// An augmented assignment, such as `x += 1`
    AugmentedAssign {
        /// The target of the assignment
        target: Expression,
        /// The operator to use with the assignment
        op: Operator,
        /// The value to assign
        expr: Expression,
    },
    /// A basic expression on its own
    Expression {
        /// The expression being used
        expr: Expression,
    },
    /// The `pass` keyword
    Pass,
    /// The `del` keyword, along with its arguments
    Delete {
        /// The elements to delete
        targets: Vec<Identifier>,
    },
    /// The `if` statement
    If {
        /// The condition and statements
        initial: Branch,
        /// An optional collection of elif statements
        elif: Vec<Branch>,
        /// A Suite to execute if it is false, basically an else statement
        optional: Option<Suite>,
    },
    /// The `while` statement
    While {
        /// The condition and statements
        branch: Branch,
    },
    /// The `return` statement
    Return {
        /// An optional expression to return the value of
        expr: Option<Expression>,
    },
    /// The `global` statement
    Global {
        /// The identifier to declare globally
        ident: Identifier,
    },
    /// A function declaration with `def`
    FunctionDecl {
        /// The name of the function
        name: Identifier,
        /// The arguments to the function if they exist
        args: Option<Vec<Identifier>>,
        /// The body of the function
        body: Suite,
        /// An optional return typehint if specified
        ret: Option<String>,
    },
}
