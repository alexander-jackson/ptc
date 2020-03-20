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

use ast::{Expression, Identifier, Operator, Suite};

mod generate;
mod infer;

/// A Statement used in the Python code.
#[derive(Debug, PartialEq)]
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
    DeleteStatement {
        /// The elements to delete
        targets: Vec<Identifier>,
    },
    /// The `if` statement
    IfStatement {
        /// The expression to check
        expr: Expression,
        /// The statements to execute if it is true
        suite: Suite,
        /// An Suite to execute if it is false, basically an else statement
        optional: Option<Suite>,
    },
    /// The `while` statement
    WhileStatement {
        /// The expression to check
        expr: Expression,
        /// The statements to execute while it is true
        suite: Suite,
    },
    /// The `return` statement
    ReturnStatement {
        /// An optional expression to return the value of
        expr: Option<Expression>,
    },
    /// The `global` statement
    GlobalStatement {
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
