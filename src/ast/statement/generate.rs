use ast::Statement;
use ast::{Context, Generate};

impl Generate for Statement {
    fn generate(&self, context: &mut Context) -> String {
        match self {
            Statement::Assign { ident, expr } => {
                // Generate the string for the identifier
                let identifier: String = ident.generate(context);

                // If the variable is defined, we need no prefix
                let prefix = if context.variable_defined(&identifier) {
                    String::new()
                } else if let Some(t) = context.get_type(&identifier) {
                    // If we know its type, we can add the type in the code
                    let str_type = String::from(t.clone());
                    context.define_variable(&identifier);
                    str_type
                } else {
                    // The variable wasn't defined and we don't know the type
                    String::from("error ")
                };

                if prefix.is_empty() {
                    format!("{}{} = {};", prefix, identifier, expr.generate(context))
                } else {
                    format!("{} {} = {};", prefix, identifier, expr.generate(context))
                }
            }
            Statement::AugmentedAssign { ident, op, expr } => {
                let ident_gen = ident.generate(context);
                let op_gen = op.generate(context);
                let expr_gen = expr.generate(context);
                format!("{} {} {};", ident_gen, op_gen, expr_gen)
            }
            Statement::Expression { expr } => format!("{};", expr.generate(context)),
            Statement::Pass => String::from(""),
            Statement::IfStatement {
                expr,
                suite,
                optional,
            } => {
                let expr_gen = expr.generate(context);

                context.next_scope();
                let suite_gen = suite.generate(context);
                context.next_scope();

                // If there is an else statement, make sure we generate it too
                let optional_gen = match optional.as_ref() {
                    Some(s) => {
                        context.next_scope();
                        let optional_gen = s.generate(context);
                        context.next_scope();
                        format!(" else {{ {} }}", &optional_gen)
                    }
                    None => String::from(""),
                };

                format!("if ({}) {{ {} }}{}", expr_gen, suite_gen, optional_gen)
            }
            Statement::WhileStatement { expr, suite } => {
                let expr_gen = expr.generate(context);

                context.next_scope();
                let suite_gen = suite.generate(context);
                context.next_scope();

                format!("while ({}) {{ {} }}", expr_gen, suite_gen)
            }
            Statement::ReturnStatement { expr } => {
                // Deal with the Option<Expression> by generating if it exists
                let ret = expr
                    .as_ref()
                    .map_or_else(|| String::from(""), |e| e.generate(context));

                if ret.is_empty() {
                    return String::from("return;");
                }

                format!("return {};", ret)
            }
            Statement::GlobalStatement { .. } => String::from(""),
            Statement::FunctionDecl { name, args, body } => {
                let name_gen = name.generate(context);

                // If we know the datatype, add it here
                let datatype = match context.get_function_return_type(&name_gen) {
                    Some(v) => String::from(v.clone()),
                    None => String::from(""),
                };

                let arg_str: Option<String> = args.as_ref().map(|s| {
                    s.iter()
                        .map(|a| format!("int {}", a.generate(context)))
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let arg_str = arg_str.unwrap_or_else(|| String::from(""));

                context.next_scope();
                let body_gen = body.generate(context);
                context.next_scope();

                format!("{} {}({}) {{ {} }}", datatype, name_gen, arg_str, body_gen,)
            }
        }
    }
}
