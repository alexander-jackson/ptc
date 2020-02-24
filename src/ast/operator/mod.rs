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
        if (left == right) {
            return left;
        }

        if (left == VariableType::Float || right == VariableType::Float) {
            return VariableType::Float;
        }

        VariableType::Integer
    }
}
