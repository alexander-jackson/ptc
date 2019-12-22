use ast::Generate;
use ast::Suite;

use ast::expression::Expression;
use ast::identifier::Identifier;
use ast::operator::Operator;

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
    },
    WhileStatement {
        expr: Expression,
        suite: Suite,
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
                format!("{} = {};", ident.generate(), expr.generate(),)
            }
            Statement::AugmentedAssign { ident, op, expr } => format!(
                "{} {} {};",
                ident.generate(),
                op.generate(),
                expr.generate(),
            ),
            Statement::Expression { expr } => expr.generate(),
            Statement::Pass => String::from(""),
            Statement::IfStatement { expr, suite } => format!(
                "if ({}) {{ {} }}",
                expr.generate(),
                suite.generate(),
            ),
            Statement::WhileStatement { expr, suite } => format!(
                "while ({}) {{ {} }}",
                expr.generate(),
                suite.generate(),
            ),
            Statement::ReturnStatement { expr } => format!("return {};", expr.generate()),
            Statement::FunctionDecl { name, args, body } => {
                let arg_str: String = args
                    .iter()
                    .map(|a| format!("int {}", a.generate()))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!(
                    "int {}({}) {{ {} }}",
                    name.generate(),
                    arg_str,
                    body.generate(),
                )
            }
        }
    }
}
