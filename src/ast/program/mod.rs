//! The top level of the AST.
//!
//! The Program struct simply contains a Vec<Statement> that define the entire syntax of the
//! program `ptc` was given. It is used as a top-level abstraction, and simply allows for the
//! `infer` and `generate` calls to be propagated throughout the program.

use ast::Suite;

mod generate;
mod infer;

/// The statements contained in the whole program.
///
/// The top level of the AST, storing everything inside it.
#[derive(Debug, PartialEq)]
pub struct Program {
    /// The statements in the program
    statements: Suite,
}

impl Program {
    /// Constructs a new Program from the statements that make it up
    pub fn new(statements: Suite) -> Program {
        Program { statements }
    }
}
