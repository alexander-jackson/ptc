use ast::Generate;

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
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

impl Generate for Operator {
    fn generate(&self) -> String {
        match self {
            Operator::Plus => String::from("+"),
            Operator::Minus => String::from("-"),
            _ => String::from(""),
        }
    }
}
