//! Implements the `Infer` trait for `Expression`.

use crate::ast::Expression;
use crate::ast::{Context, DataType, Generate, Infer, VariableType};

impl Infer for Expression {
    /// Allows type inference to propagate correctly down the AST.
    fn infer(&mut self, context: &mut Context) {
        match self {
            Expression::BinaryOperation { left, right, .. } => {
                left.infer(context);
                right.infer(context);
            }
            Expression::UnaryOperation { expr, .. }
            | Expression::ParenExpression { expr, .. }
            | Expression::Subscription { expr, .. } => {
                expr.infer(context);
            }
            Expression::FunctionCall { name, args } => {
                let fname = name.generate(context);

                // Check whether we are calling append on a variable
                if let Expression::AttributeRef { primary, attribute } = &**name {
                    if attribute.get_identifier() == "append" {
                        if let Some(a) = args {
                            if let Some(element_type) = a[0].get_type(context) {
                                let t = VariableType::List {
                                    elements: Some(Box::new(element_type)),
                                };
                                let p = primary.generate(context);
                                context.insert_shallow_inferred_type(&p, &t);
                            }
                        }
                    }
                }

                // Check for usage of the `len` function
                if let Expression::Identifier { name } = &**name {
                    if name.get_identifier() == "len" {
                        if let Some(a) = args {
                            // `len(name)` allows us to infer that `name` is a list
                            let t = VariableType::List { elements: None };
                            let p = a[0].generate(context);
                            // As the list may not be in the current scope, insert where it is
                            context.insert_shallow_inferred_type(&p, &t);
                        }
                    }
                }

                // Iterate the arguments and see if we know the types for them
                // This allows for `func(1, 2)` to infer that `func` takes 2 integers
                if let Some(a) = args {
                    for (i, e) in a.iter().enumerate() {
                        if let Some(e_type) = e.get_type(context) {
                            context.set_function_argument_type(&fname, i, e_type);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
