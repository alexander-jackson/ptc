//! An Expression used in a Python program.
//!
//! Expressions are things such as `3+4`, `func(1)` or `list.append(x)`. Expressions occur more
//! than expected, with a lot of `Statement`s simply being `Expression`s.
//!
//! Expressions have a datatype that can usually be worked out. They can also be used to propagate
//! type inference calls, such as for duck typing or finding what parameters are passed to a
//! function.

use ast::{Identifier, Literal, Operator};

mod datatype;
mod generate;
mod infer;

/// An Expression in a Python program.
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// A binary operation, such as `3+4`.
    BinaryOperation {
        /// The left-hand side of the expression
        left: Box<Expression>,
        /// The operator to apply
        op: Operator,
        /// The right-hand side of the expression
        right: Box<Expression>,
    },
    /// A unary operation, such as `-3` or `not 1`.
    UnaryOperation {
        /// The operator to apply
        op: Operator,
        /// The expression to apply it to
        expr: Box<Expression>,
    },
    /// A parenthesised expression, such as `(-3)`.
    ParenExpression {
        /// The expression inside the parenthesis
        expr: Box<Expression>,
    },
    /// A basic list initialisation, always `[]`.
    ListDisplay,
    /// A call to a function, such as `func(1)` or `list.append(x)`.
    FunctionCall {
        /// The name of the function being called
        name: Box<Expression>,
        /// The arguments to the function
        args: Option<Vec<Expression>>,
    },
    /// A reference to an attribute, such as `list.append`
    AttributeRef {
        /// Everything before the `.`
        primary: Box<Expression>,
        /// The attribute we are referring to
        attribute: Identifier,
    },
    /// An indexing of a variable, such as `list[0]`
    Subscription {
        /// The expression we are indexing
        primary: Box<Expression>,
        /// The expression inside the `[]`
        expr: Box<Expression>,
    },
    /// A lone identifier, such as `x`
    Identifier {
        /// The identifier that makes up the expression
        name: Identifier,
    },
    /// A lone literal, such as `1` or `0.5`
    Literal {
        /// The literal that makes up the expression
        value: Literal,
    },
}
