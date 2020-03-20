//! A Literal value, either an integer or a float.
//!
//! Literals such as `1` or `0.5` can be used in Expressions. They can either be `Integer` literals
//! or `Float` literals depending on whether they have a decimal point. Floats such as `0.` are
//! considered malformed, all Float values must have a value before and after the point.
//!
//! Literals are useful for inference as they have a concrete type. This allows us to use them as
//! definite values when continuing inference.

mod datatype;
mod generate;

/// A Literal value.
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// An Integer value
    Integer {
        /// The value the integer has, between 0 and 4,294,967,295
        value: u32,
    },
    /// A floating point value
    Float {
        /// The value the float has
        value: f32,
    },
}
