//! The valid operators that `ptc` understands.

use ast::VariableType;

mod generate;

#[derive(Debug, PartialEq)]
/// An operator that can be used in an expression.
pub enum Operator {
    /// `or` in Python, `||` in C
    LogicalOr,
    /// `and` in Python, `&&` in C
    LogicalAnd,
    /// `not` in Python, `!` in C
    LogicalNot,
    /// `=` in Python, `=` in C
    Assign,
    /// `+` in Python, `+` in C
    Plus,
    /// `-` in Python, `-` in C
    Minus,
    /// `*` in Python, `*` in C
    Multiply,
    /// `/` or `//` in Python, `/` in C
    Divide,
    /// `%` in Python, `%` in C
    Modulo,
    /// `+=` in Python, `+=` in C
    PlusEquals,
    /// `-=` in Python, `-=` in C
    MinusEquals,
    /// `*=` in Python, `*=` in C
    MultiplyEquals,
    /// `/=` or `//=` in Python, `/=` in C
    DivideEquals,
    /// `%=` in Python, `%=` in C
    ModuloEquals,
    /// `<` in Python, `<` in C
    Less,
    /// `>` in Python, `>` in C
    Greater,
    /// `<=` in Python, `<=` in C
    LessOrEqual,
    /// `>=` in Python, `>=` in C
    GreaterOrEqual,
    /// `==` in Python, `==` in C
    Equal,
    /// `!=` in Python, `!=` in C
    NotEqual,
}

impl Operator {
    /// Calculates the resulting type for this operator and its operands.
    ///
    /// Given the types of the left and right expressions for this operand, decides what the output
    /// operand should be. If `Operand` is a `Divide`, it will always produce a
    /// `VariableType::Float`. If both `left` and `right` are equal, it will produce the same type
    /// as them. If either `left` or `right` are `VariableType::Float`, a `VariableType::Float` will
    /// be output. If none of these are true, it will default to a `VariableType::Integer`.
    pub fn resulting_type(&self, left: VariableType, right: VariableType) -> VariableType {
        // If the operator is a divide, we probably want a float out of it
        if let Operator::Divide = self {
            return VariableType::Float;
        }

        if (left == right) {
            return left;
        }

        if (left == VariableType::Float || right == VariableType::Float) {
            return VariableType::Float;
        }

        VariableType::Integer
    }
}
