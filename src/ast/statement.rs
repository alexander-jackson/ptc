use ast::Context;
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
    fn generate(&self, context: &mut Context) -> String {
        match self {
            Statement::Assign { ident, expr } => {
                // Check whether the variable is undefined
                let identifier: String = ident.generate(context);
                let prefix: String = if context.contains(&identifier) {
                    String::from("")
                } else {
                    context.insert(&identifier);
                    String::from("int ")
                };

                format!("{}{} = {};", prefix, identifier, expr.generate(context))
            }
            Statement::AugmentedAssign { ident, op, expr } => format!(
                "{} {} {};",
                ident.generate(context),
                op.generate(context),
                expr.generate(context),
            ),
            Statement::Expression { expr } => format!("{};", expr.generate(context)),
            Statement::Pass => String::from(""),
            Statement::IfStatement {
                expr,
                suite,
                optional,
            } => {
                let expr_gen = expr.generate(context);

                context.add_scope();
                let suite_gen = suite.generate(context);
                context.remove_scope();

                let optional_gen = match optional.as_ref() {
                    Some(s) => {
                        context.add_scope();
                        let optional_gen = s.generate(context);
                        context.remove_scope();
                        format!(" else {{ {} }}", &optional_gen)
                    }
                    None => String::from(""),
                };

                format!("if ({}) {{ {} }}{}", expr_gen, suite_gen, optional_gen)
            }
            Statement::WhileStatement { expr, suite } => {
                let expr_gen = expr.generate(context);

                context.add_scope();
                let suite_gen = suite.generate(context);
                context.remove_scope();

                format!("while ({}) {{ {} }}", expr_gen, suite_gen)
            }
            Statement::ReturnStatement { expr } => format!("return {};", expr.generate(context)),
            Statement::FunctionDecl { name, args, body } => {
                let arg_str: Option<String> = args.as_ref().map(|s| {
                    s.iter()
                        .map(|a| format!("int {}", a.generate(context)))
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let arg_str = arg_str.unwrap_or_else(|| String::from(""));
                let name_gen = name.generate(context);

                context.add_scope();
                let body_gen = body.generate(context);
                context.remove_scope();

                format!("int {}({}) {{ {} }}", name_gen, arg_str, body_gen,)
            }
        }
    }
}
