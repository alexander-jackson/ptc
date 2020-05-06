//! Implements the `Generate` trait for `Identifier`.

use ast::Identifier;
use ast::{Context, Generate};

impl Generate for Identifier {
    /// Generates the string representation for the `Identifier`.
    ///
    /// As identifiers do not need to be changed, this just converts the internal representation to
    /// a heap allocated `String` for both variants.
    fn generate(&self, _context: &mut Context) -> String {
        match self {
            Identifier::Name { name } | Identifier::Typed { name, .. } => name.to_string(),
        }
    }
}
