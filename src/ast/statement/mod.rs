use ast::{Expression, Identifier, Operator, Suite};

mod generate;
mod infer;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        ident: Identifier,
        expr: Expression,
    },
    AugmentedAssign {
        ident: Identifier,
        op: Operator,
        expr: Expression,
    },
    Expression {
        expr: Expression,
    },
    Pass,
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
        expr: Expression,
    },
    GlobalStatement {
        ident: Identifier,
    },
    FunctionDecl {
        name: Identifier,
        args: Option<Vec<Identifier>>,
        body: Suite,
    },
}
