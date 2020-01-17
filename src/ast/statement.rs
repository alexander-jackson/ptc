use ast::Generate;
use ast::Suite;
use ast::SymbolTable;

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
    fn generate(&self, symbol_table: &mut SymbolTable) -> String {
        match self {
            Statement::Assign { ident, expr } => {
                // Check whether the variable is undefined
                let identifier: String = ident.generate(symbol_table);
                let prefix: String = if symbol_table.contains(&identifier) {
                    String::from("")
                } else {
                    symbol_table.insert(identifier.clone());
                    String::from("int ")
                };

                format!("{}{} = {};", prefix, identifier, expr.generate(symbol_table),)
            }
            Statement::AugmentedAssign { ident, op, expr } => format!(
                "{} {} {};",
                ident.generate(symbol_table),
                op.generate(symbol_table),
                expr.generate(symbol_table),
            ),
            Statement::Expression { expr } => format!("{};", expr.generate(symbol_table)),
            Statement::Pass => String::from(""),
            Statement::IfStatement {
                expr,
                suite,
                optional,
            } => {
                let mut output = String::new();
                output.push_str(&format!(
                    "if ({}) {{ {} }}",
                    expr.generate(symbol_table),
                    suite.generate(symbol_table),
                ));

                if optional.is_some() {
                    let optional_gen: String =
                        format!(" else {{ {} }}", optional.as_ref().unwrap().generate(symbol_table));
                    output.push_str(&optional_gen);
                }

                output
            }
            Statement::WhileStatement { expr, suite } => {
                format!("while ({}) {{ {} }}", expr.generate(symbol_table), suite.generate(symbol_table),)
            }
            Statement::ReturnStatement { expr } => format!("return {};", expr.generate(symbol_table)),
            Statement::FunctionDecl { name, args, body } => {
                let arg_str: Option<String> = args.as_ref().map(|s| {
                    s.iter()
                        .map(|a| format!("int {}", a.generate(symbol_table)))
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let arg_str = arg_str.unwrap_or_else(|| String::from(""));

                format!(
                    "int {}({}) {{ {} }}",
                    name.generate(symbol_table),
                    arg_str,
                    body.generate(symbol_table),
                )
            }
        }
    }
}
