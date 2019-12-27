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
        optional: Option<Suite>,
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
        args: Option<Vec<Identifier>>,
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
            Statement::IfStatement {
                expr,
                suite,
                optional,
            } => {
                let mut output = String::new();
                output.push_str(&format!(
                    "if ({}) {{ {} }}",
                    expr.generate(),
                    suite.generate(),
                ));

                if optional.is_some() {
                    let optional_gen: String =
                        format!("else {{ {} }}", optional.as_ref().unwrap().generate());
                    output.push_str(&optional_gen);
                }

                output
            }
            Statement::WhileStatement { expr, suite } => {
                format!("while ({}) {{ {} }}", expr.generate(), suite.generate(),)
            }
            Statement::ReturnStatement { expr } => format!("return {};", expr.generate()),
            Statement::FunctionDecl { name, args, body } => {
                let arg_str: Option<String> = args.as_ref().map(|s| {
                    s.iter()
                        .map(|a| format!("int {}", a.generate()))
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let arg_str = arg_str.unwrap_or(String::from(""));

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
