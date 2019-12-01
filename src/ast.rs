pub type Suite = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Suite,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        ident: Identifier,
        expr: Expression
    },
    AugmentedAssign {
        ident: Identifier,
        op: Operator,
        expr: Expression,
    },
    Expression {
        expr: Expression
    },
    Pass,
    IfStatement {
        expr: Expression,
        stmt: Suite,
    },
    WhileStatement {
        expr: Expression,
        stmt: Suite,
    },
    ReturnStatement {
        expr: Expression,
    },
    FunctionDecl {
        name: Identifier,
        args: Vec<Identifier>,
        body: Suite,
    },
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name { name: String },
}

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

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOperation {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
    UnaryOperation {
        op: Operator,
        expr: Box<Expression>,
    },
    ParenExpression {
        expr: Box<Expression>,
    },
    Identifier {
        name: Identifier,
    },
    Literal {
        value: Literal,
    },
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer { value: u32 },
}
