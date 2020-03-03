use ast::Statement;
use ast::{Context, Expression, Generate, VariableType};

impl Generate for Statement {
    fn generate(&self, context: &mut Context) -> String {
        match self {
            Statement::Assign { target, expr } => {
                // If the LHS is a pure identifier
                if let Expression::Identifier { name } = target {
                    let identifier: String = name.get_identifier();
                    let dtype = context.get_type(&identifier);
                    let expr_gen = match check_list_display(&expr, dtype) {
                        None => expr.generate(context),
                        Some(g) => {
                            context.add_include("list.h");
                            g
                        }
                    };

                    // Check whether it is defined
                    if context.variable_defined(&identifier) {
                        return format!("{} = {};", identifier, expr_gen);
                    }

                    // Otherwise, we should define it if we have a type for it
                    if let Some(t) = context.get_type(&identifier) {
                        let str_type = String::from(t.clone());
                        context.define_variable(&identifier);
                        return format!("{} {} = {};", str_type, identifier, expr_gen);
                    }

                    return format!("error {} = {};", identifier, expr_gen);
                }

                let expr_gen = expr.generate(context);

                // Check for a subscription
                if let Expression::Subscription { primary, expr } = target {
                    let p_ident = primary.generate(context);
                    let index = expr.generate(context);
                    return format!("{}->data[{}] = {};", p_ident, index, expr_gen);
                }

                String::from("unimplemented")
            }
            Statement::AugmentedAssign { target, op, expr } => {
                let op_gen = op.generate(context);
                let expr_gen = expr.generate(context);

                // Check if this is an identifier so we can early return
                if let Expression::Identifier { name } = target {
                    let ident = name.get_identifier();
                    return format!("{} {} {};", ident, op_gen, expr_gen);
                }

                // Check whether this is a subscription on the LHS
                if let Expression::Subscription { primary, expr } = target {
                    let primary_gen = primary.generate(context);
                    let index = expr.generate(context);
                    return format!("{}->data[{}] {} {};", primary_gen, index, op_gen, expr_gen);
                }

                String::from("unimplemented")
            }
            Statement::Expression { expr } => format!("{};", expr.generate(context)),
            Statement::Pass => String::from(""),
            Statement::DeleteStatement { targets } => {
                let mut strs: Vec<String> = Vec::new();

                for ident in targets {
                    let identifier = ident.get_identifier();

                    if let Some(VariableType::List { .. }) = context.get_type(&identifier) {
                        strs.push(format!("free({});", identifier));
                    }
                }

                strs.join(" ")
            }
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
            Statement::FunctionDecl {
                name, args, body, ..
            } => {
                let name_gen = name.generate(context);

                // If we know the datatype, add it here
                let datatype = match context.get_function_return_type(&name_gen) {
                    Some(v) => match v {
                        VariableType::Unknown => String::from("void"),
                        _ => String::from(v.clone()),
                    },
                    None => String::from(""),
                };

                let arg_str = match args {
                    Some(args) => {
                        let mut arguments: Vec<String> = Vec::new();
                        let f_args = context.get_function_argument_types(&name_gen);

                        if let Some(v) = f_args {
                            for (t, a) in v.iter().zip(args.iter()) {
                                let str_type = match t {
                                    Some(vtype) => String::from(vtype.clone()),
                                    None => String::from("unknown"),
                                };

                                arguments.push(format!("{} {}", str_type, a.get_identifier()));
                            }
                        } else {
                            for a in args.iter() {
                                arguments.push(format!("unknown {}", a.generate(context)));
                            }
                        }

                        arguments.join(", ")
                    }
                    None => String::from(""),
                };

                context.next_scope();
                let body_gen = body.generate(context);
                context.next_scope();

                format!("{} {}({}) {{ {} }}", datatype, name_gen, arg_str, body_gen,)
            }
        }
    }
}

fn check_list_display(expr: &Expression, dtype: Option<&VariableType>) -> Option<String> {
    if let Expression::ListDisplay = expr {
        if let Some(t) = dtype {
            if let VariableType::List { elements } = t {
                return Some(format!("list_{}_new()", String::from(*elements.clone())));
            }
        }
    }

    None
}
