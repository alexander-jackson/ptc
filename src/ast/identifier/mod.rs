//! An Identifier which can be optionally typed.
//!
//! Identifiers such as `x` and `i: int` can be used to store variables. Declarations in Python can
//! optionally declare a typehint as of PEP 484 and these can be used to make type inference much
//! easier.
//!
//! Identifiers can come in two forms. When used normally, they will always be the `Name` variant,
//! but when declared they can be the `Typed` variant if the typehint syntax is used.
//!
//! Typehints can be used to assume the type of a variable, in `ptc` we always assume that a
//! variable has the type of its typehint. Thus, the following code would be considered malformed,
//! even if it is legal Python.
//!
//! ```python
//! x: int = 0
//! x = []
//! ```
//!
//! Here, `ptc` would not currently throw an error, but would generate:
//!
//! ```c
//! int x = 0;
//! x = list_int_new();
//! ```
//!
//! The C compiler would catch this for us. Despite it being legal Python code, dynamic typing is
//! currently not supported.
//!
//! Identifier names are usually stored in the `Context` for the program, which allows us to
//! remember their type and whether they have been defined yet.

mod datatype;
mod generate;

/// An Identifier in the program, usually a string of characters that stores a value.
#[derive(Debug, PartialEq)]
pub enum Identifier {
    /// A basic Identifier
    Name {
        /// The internal name of the Identifier
        name: String,
    },
    /// An Identifier used in a variable declaration with a typehint
    Typed {
        /// The internal name of the Identifier
        name: String,
        /// The type that we should assume it has
        typehint: String,
    },
}

impl Identifier {
    /// Gets the name of the Identifier by cloning it
    pub fn get_identifier(&self) -> String {
        match self {
            Identifier::Name { name } | Identifier::Typed { name, .. } => name.clone(),
        }
    }
}
