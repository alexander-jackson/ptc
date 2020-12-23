//! Implements the `DataType` trait for `Identifier`.
//!
//! This allows identifiers to have their types guessed using the known context of the program.

use crate::ast::{Context, DataType, Identifier, VariableType};

impl DataType for Identifier {
    /// Gets the datatype for the identifier if known.
    fn get_type(&self, context: &mut Context) -> Option<VariableType> {
        match self {
            Identifier::Name { name, .. } => {
                // See if we know what type it is already
                context.get_type(&name).cloned()
            }
            Identifier::Typed { typehint, .. } => Some(VariableType::from(typehint.as_str())),
        }
    }
}
