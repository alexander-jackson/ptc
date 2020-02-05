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
