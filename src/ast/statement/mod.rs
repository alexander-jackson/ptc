use ast::{Expression, Identifier, Operator, Suite};

mod generate;
mod infer;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        target: Expression,
        expr: Expression,
    },
    AugmentedAssign {
        target: Expression,
        op: Operator,
        expr: Expression,
    },
    Expression {
        expr: Expression,
    },
    Pass,
    DeleteStatement {
        targets: Vec<Identifier>,
    },
    IfStatement {
        expr: Expression,
        suite: Suite,
        optional: Option<Suite>,
    },
    WhileStatement {
        expr: Expression,
        suite: Suite,
    },
    ReturnStatement {
        expr: Option<Expression>,
    },
    GlobalStatement {
        ident: Identifier,
    },
    FunctionDecl {
        name: Identifier,
        args: Option<Vec<Identifier>>,
        body: Suite,
        ret: Option<String>,
    },
}
