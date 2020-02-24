use ast::VariableType;

mod generate;

#[derive(Debug, PartialEq)]
pub enum Operator {
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    ModuloEquals,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

impl Operator {
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
