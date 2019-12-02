use ast::Suite;
use ast::expression::Expression;
use ast::operator::Operator;
use ast::identifier::Identifier;

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
