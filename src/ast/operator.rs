use ast::Generate;
use ast::Context;

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
    fn generate(&self, _context: &mut Context) -> String {
        match self {
            Operator::LogicalOr => String::from("||"),
            Operator::LogicalAnd => String::from("&&"),
            Operator::LogicalNot => String::from("!"),
            Operator::Assign => String::from("="),
            Operator::Plus => String::from("+"),
            Operator::Minus => String::from("-"),
            Operator::Multiply => String::from("*"),
            Operator::Divide => String::from("/"),
            Operator::PlusEquals => String::from("+="),
            Operator::MinusEquals => String::from("-="),
            Operator::MultiplyEquals => String::from("*="),
            Operator::DivideEquals => String::from("/="),
            Operator::Less => String::from("<"),
            Operator::Greater => String::from(">"),
            Operator::LessOrEqual => String::from("<="),
            Operator::GreaterOrEqual => String::from(">="),
            Operator::Equal => String::from("=="),
            Operator::NotEqual => String::from("!="),
        }
    }
}
