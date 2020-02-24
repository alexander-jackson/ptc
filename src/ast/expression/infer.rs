use ast::Expression;
use ast::{Context, DataType, Generate, Infer};

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
