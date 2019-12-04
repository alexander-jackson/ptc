use ast::Suite;
use ast::Generate;

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

impl Generate for Statement {
    fn generate(&self) -> String {
        match self {
            Statement::Assign { ident, expr } => {
                format!(
                    "{} = {};",
                    ident.generate(),
                    expr.generate(),
                )
            },
            Statement::AugmentedAssign { ident, op, expr } => {
                format!(
                    "{} {} {};",
                    ident.generate(),
                    op.generate(),
                    expr.generate(),
                )
            },
            Statement::Expression { expr } => expr.generate(),
            Statement::Pass => String::from(""),
            Statement::IfStatement { expr, stmt } => {
                format!(
r#"if ({}) {{
    {}
}}"#,
                    expr.generate(),
                    stmt[0].generate(),
                )
            },
            Statement::FunctionDecl { name, args, body } => {
                let arg_str: String = args.iter()
                    .map(|a| format!("int {}", a.generate()))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!(
r#"int {}({}) {{
    {}
}}"#,
                    name.generate(),
                    arg_str,
                    body[0].generate(),
                )
            },
            _ => String::from(""),
        }
    }
}
