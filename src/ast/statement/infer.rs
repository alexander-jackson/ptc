//! Implements the `Infer` trait for `Statement`.

use crate::ast::{Context, DataType, Expression, Identifier, Infer, Statement, VariableType};

impl Infer for Statement {
    fn infer(&mut self, context: &mut Context) {
        match self {
            Statement::Assign { target, expr } => {
                // If the target is a variable, we can infer the type of it
                if let Expression::Identifier { name } = target {
                    let identifier = name.get_identifier();

                    // If we have a type hint, use that, otherwise infer using the expression
                    let inferred = match name {
                        Identifier::Name { .. } => expr.get_type(context),
                        Identifier::Typed { .. } => name.get_type(context),
                    };

                    if let Some(t) = inferred {
                        context.insert_inferred_type(&identifier, t);
                    }
                }

                // Propagate the calls into the expression
                expr.infer(context);
            }
            Statement::Expression { expr } => expr.infer(context),
            Statement::If {
                initial,
                elif,
                optional,
                ..
            } => {
                // Push a scope into the Context and infer on the `if` statement contents
                context.push_scope();
                initial.block.infer(context);
                context.pop_scope();

                for e in elif {
                    context.push_scope();
                    e.block.infer(context);
                    context.pop_scope();
                }

                if let Some(s) = optional {
                    // Push a scope into the Context and infer on the `else` statement contents
                    context.push_scope();
                    s.infer(context);
                    context.pop_scope();
                }
            }
            Statement::While { branch } => {
                // Push a scope into the Context and infer on the `while` statement contents
                context.push_scope();
                branch.block.infer(context);
                context.pop_scope();
            }
            Statement::Return { expr: Some(expr) } => {
                // If the statement returns a value, get the type of it
                if let Some(t) = expr.get_type(context) {
                    context.set_function_return_type(t);
                }
            }
            Statement::FunctionDecl {
                name,
                args,
                body,
                ret,
            } => {
                context.push_scope();
                let function_name = name.get_identifier();
                context.set_current_function(function_name.to_owned());

                // If the return type is type hinted, insert this information
                if let Some(r) = ret {
                    let rtype = VariableType::from(r.as_str());

                    // If it returns a list, add this include
                    if let VariableType::List { .. } = rtype {
                        context.add_include("list.h");
                    }

                    context.set_function_return_type(rtype);
                }

                // If the function has any arguments, check whether they have typehints
                if let Some(arguments) = args {
                    for (index, ident) in arguments.iter().enumerate() {
                        // Insert all the argument names into the `Context`
                        context.set_function_argument_name(
                            &function_name,
                            index,
                            &ident.get_identifier(),
                        );

                        // If the function argument has a type hint, insert this
                        if let Identifier::Typed { typehint, .. } = ident {
                            let vtype = VariableType::from(typehint.as_str());

                            if let VariableType::List { .. } = vtype {
                                context.add_include("list.h");
                            }

                            context.set_function_argument_type(
                                &function_name,
                                index,
                                vtype.clone(),
                            );
                        }
                    }

                    // If we know some types already from previous usage, load these
                    let ftypes = context.get_function_argument_types(&function_name);

                    let insertions: Option<Vec<_>> = ftypes.map(|types| {
                        types
                            .iter()
                            .zip(arguments.iter())
                            .filter_map(|(dtype, arg)| {
                                dtype.as_ref().map(|t| (arg.get_identifier(), t.clone()))
                            })
                            .collect()
                    });

                    if let Some(insertions) = insertions {
                        for (name, dtype) in insertions {
                            context.insert_inferred_type(&name, dtype);
                        }
                    }
                }

                // Infer on the body and leave the function scope
                body.infer(context);
                context.pop_scope();
                context.set_current_function(None);
            }
            _ => (),
        }
    }
}
