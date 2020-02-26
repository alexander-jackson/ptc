use ast::Statement;
use ast::{Context, DataType, Expression, Identifier, Infer, VariableType};

impl Infer for Statement {
    fn infer(&mut self, context: &mut Context) {
        match self {
            Statement::Assign { target, expr } => {
                // If the target is a variable, we can infer the type of it
                if let Expression::Identifier { name } = target {
                    let identifier: String = name.get_identifier();

                    let inferred = match name {
                        Identifier::Name { .. } => expr.get_type(context),
                        Identifier::Typed { .. } => name.get_type(context),
                    };

                    context.insert_inferred_type(&identifier, inferred);
                }

                expr.infer(context);
            }
            Statement::Expression { expr } => expr.infer(context),
            Statement::IfStatement {
                suite, optional, ..
            } => {
                context.push_scope();
                suite.infer(context);
                context.pop_scope();

                if let Some(s) = optional {
                    context.push_scope();
                    s.infer(context);
                    context.pop_scope();
                }
            }
            Statement::WhileStatement { suite, .. } => {
                context.push_scope();
                suite.infer(context);
                context.pop_scope();
            }
            Statement::ReturnStatement { expr } => {
                // If the statement returns a value, get the type of it
                if let Some(e) = expr {
                    let datatype = e.get_type(context);
                    context.set_function_return_type(datatype);
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
                context.set_current_function(Some(function_name.clone()));

                if let Some(r) = ret {
                    let rtype = VariableType::from(r.clone());
                    context.set_function_return_type(rtype);
                }

                // If the function has any arguments, check whether they have typehints
                if let Some(arguments) = args {
                    for (index, ident) in arguments.iter().enumerate() {
                        if let Identifier::Typed { typehint, .. } = ident {
                            let vtype = VariableType::from(typehint.clone());

                            context.set_function_argument_type(
                                &function_name,
                                index,
                                vtype.clone(),
                            );
                        }
                    }

                    let ftypes = context.get_function_argument_types(&function_name);
                    let mut insertions: Vec<(String, VariableType)> = Vec::new();

                    // Load all the inferred types that we know so far
                    // These are either inferred or typehints
                    if let Some(types) = ftypes {
                        // Iterate the types and argument names
                        for (dtype, arg) in types.iter().zip(arguments.iter()) {
                            // If we have a type for this argument
                            if let Some(t) = dtype {
                                let identifier = arg.get_identifier();
                                insertions.push((identifier, t.clone()));
                            }
                        }
                    }

                    for (name, dtype) in insertions {
                        context.insert_inferred_type(&name, dtype);
                    }
                }

                body.infer(context);
                context.pop_scope();
                context.set_current_function(None);
            }
            _ => (),
        }
    }
}
