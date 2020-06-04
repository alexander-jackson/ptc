//! An abstract syntax tree produced by the parser.
//!
//! The AST represents the program syntax at a more abstract level, removing some of the finer
//! details. The parser produces an AST when it is executed, and the AST understands things like
//! type inference, expression types and code generation.
//!
//! Each of the AST nodes are stored in their own module. Each one may implement a different subset
//! of the traits found in this module. They are all likely to use the `Context` struct, which
//! allows for inference to be carried across from the `infer` call to the `generate` call.

mod context;
mod expression;
mod identifier;
mod literal;
mod operator;
mod program;
mod statement;
mod symboltable;
mod variable_type;

pub use self::{
    context::Context, expression::Expression, identifier::Identifier, literal::Literal,
    operator::Operator, program::Program, statement::Branch, statement::Statement,
    symboltable::SymbolTable, variable_type::VariableType,
};

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
    /// Generates the code for a given Suite.
    ///
    /// Simply iterates through the statements in a Suite and generates each of them, concatenating
    /// them with a space.
    fn generate(&self, context: &mut Context) -> String {
        self.iter()
            .map(|s| s.generate(context))
            .collect::<Vec<String>>()
            .join(" ")
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
    /// Infer on each of the statements in a Suite.
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
/// assert_eq!(node.get_type(&mut context), Some(VariableType::Float));
/// ```
pub trait DataType {
    /// Gets the type of the current node, using the known context if needed.
    fn get_type(&self, &mut Context) -> Option<VariableType>;
}
