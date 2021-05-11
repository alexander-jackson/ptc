//! Implements the `Generate` trait for `Statement`.

use itertools::Itertools;

use crate::ast::{Context, Expression, Generate, Statement, VariableType};

impl Generate for Statement {
    fn generate(&self, context: &mut Context) -> String {
        match self {
            Statement::Assign { target, expr } => {
                // If the LHS is a pure identifier
                if let Expression::Identifier { name } = target {
                    let identifier = name.get_identifier();
                    let dtype = context.get_type(&identifier);

                    // Generate a list display if that's what the RHS is
                    let expr_gen = match check_list_display(&expr, dtype) {
                        None => expr.generate(context),
                        Some(g) => {
                            context.add_include("list.h");
                            // Check whether this is a global list
                            if context.in_global_scope() {
                                if let Some(t) = context.get_type(&identifier) {
                                    return format!("{} {};", t, &identifier);
                                }
                            }

                            g
                        }
                    };

                    // Check whether it is defined
                    if context.variable_defined(&identifier) {
                        return format!("{} = {};", identifier, expr_gen);
                    }

                    // Otherwise, we should define it if we have a type for it
                    if let Some(t) = context.get_type(&identifier) {
                        let generated = format!("{} {} = {};", t, identifier, expr_gen);
                        context.define_variable(&identifier);
                        return generated;
                    }

                    return format!("unknown {} = {};", identifier, expr_gen);
                }

                let expr_gen = expr.generate(context);

                // Check for a subscription on the left-hand side
                if let Expression::Subscription { primary, expr } = target {
                    let p_ident = primary.generate(context);
                    let index = expr.generate(context);
                    return format!("{}->data[{}] = {};", p_ident, index, expr_gen);
                }

                unreachable!(
                    "Found an instance of Statement::Assign that didn't generate correctly."
                );
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

                unreachable!(
                    "Found an instance of Statement::AugmentedAssign that didn't generate correctly."
                );
            }
            Statement::Expression { expr } => format!("{};", expr.generate(context)),
            Statement::Delete { targets } => targets
                .iter()
                .filter_map(|t| {
                    let ident = t.get_identifier();

                    // If the identifier is a list with known element types
                    if let Some(VariableType::List { elements }) = context.get_type(&ident) {
                        elements
                            .as_ref()
                            .map(|e| format!("list_{}_free({});", e.to_string(), ident))
                    } else {
                        None
                    }
                })
                .join(" "),
            Statement::If {
                initial,
                elif,
                optional,
            } => {
                let expr_gen = initial.condition.generate(context);

                context.next_scope();
                let suite_gen = initial.block.generate(context);
                context.next_scope();

                let if_gen = format!("if ({}) {{ {} }}", expr_gen, suite_gen);

                let elif_gen = elif
                    .iter()
                    .map(|(branch)| {
                        format!(
                            " else if ({}) {{ {} }}",
                            branch.condition.generate(context),
                            branch.block.generate(context)
                        )
                    })
                    .join(" ");

                let optional_gen = optional
                    .as_ref()
                    .map(|s| {
                        context.next_scope();
                        let optional_gen = s.generate(context);
                        context.next_scope();
                        format!(" else {{ {} }}", &optional_gen)
                    })
                    .unwrap_or_default();

                // If there is an else statement, make sure we generate it too
                format!("{}{}{}", if_gen, elif_gen, optional_gen)
            }
            Statement::While { branch } => {
                let expr_gen = branch.condition.generate(context);

                context.next_scope();
                let suite_gen = branch.block.generate(context);
                context.next_scope();

                format!("while ({}) {{ {} }}", expr_gen, suite_gen)
            }
            Statement::Return { expr } => expr.as_ref().map_or_else(
                || String::from("return;"),
                |e| format!("return {};", e.generate(context)),
            ),
            Statement::Global { .. } | Statement::Pass => String::new(),
            Statement::FunctionDecl {
                name, args, body, ..
            } => {
                let name_gen = name.get_identifier();
                let ret_type = context.get_function_return_type(&name_gen);

                // If we know the datatype, add it here
                let datatype = ret_type
                    .map(ToString::to_string)
                    .unwrap_or_else(|| ToString::to_string(&VariableType::Void));

                // Get around the borrow checker quickly
                let mut list_h = matches!(ret_type, Some(VariableType::List { .. }));

                // Get the arg string if we have one
                let arg_str = args
                    .as_ref()
                    .map(|args| {
                        let mut arguments: Vec<String> = Vec::new();

                        // Get the inferred argument types
                        if let Some(v) = context.get_function_argument_types(&name_gen) {
                            for (t, a) in v.iter().zip(args.iter()) {
                                let str_type = match t {
                                    Some(vtype) => vtype.to_string(),
                                    None => String::from("unknown"),
                                };

                                if let Some(VariableType::List { .. }) = t {
                                    list_h = true;
                                }

                                arguments.push(format!("{} {}", str_type, a.get_identifier()));
                            }
                        } else {
                            for a in args.iter() {
                                arguments.push(format!("unknown {}", a.get_identifier()));
                            }
                        }

                        arguments.join(", ")
                    })
                    .unwrap_or_default();

                if list_h {
                    context.add_header_include("list.h");
                }

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
        if let Some(VariableType::List { elements }) = dtype {
            let expr_str = format!(
                "list_{}_new()",
                elements
                    .as_ref()
                    .map_or_else(|| String::from("unknown"), ToString::to_string)
            );

            return Some(expr_str);
        }
    }

    None
}
