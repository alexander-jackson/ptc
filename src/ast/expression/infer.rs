use ast::Expression;
use ast::{Context, DataType, Generate, Infer, VariableType};

impl Infer for Expression {
    fn infer(&mut self, context: &mut Context) {
        match self {
            Expression::BinaryOperation { left, right, .. } => {
                left.infer(context);
                right.infer(context);
            }
            Expression::UnaryOperation { expr, .. } => {
                expr.infer(context);
            }
            Expression::ParenExpression { expr, .. } => {
                expr.infer(context);
            }
            Expression::FunctionCall { name, args } => {
                let fname = name.generate(context);

                // Check whether we are calling append on a variable
                if let Expression::AttributeRef { primary, attribute } = &**name {
                    if attribute.get_identifier() == "append" {
                        if let Some(a) = args {
                            let t = VariableType::List {
                                elements: Box::new(a[0].get_type(context)),
                            };
                            let p = primary.generate(context);
                            context.insert_inferred_type(&p, t);
                        }
                    }
                }

                if let Some(a) = args {
                    for (i, e) in a.iter().enumerate() {
                        let e_type = e.get_type(context);
                        context.set_function_argument_type(&fname, i, e_type);
                    }
                }
            }
            Expression::Subscription { expr, .. } => {
                expr.infer(context);
            }
            _ => (),
        }
    }
}
