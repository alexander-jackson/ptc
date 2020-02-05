use ast::Statement;
use ast::{Context, DataType, Generate, Infer};

impl Infer for Statement {
    fn infer(&mut self, context: &mut Context) {
        match self {
            Statement::Assign { ident, expr } => {
                let inferred = expr.get_type(context);
                println!("Inferred type for '{:?}': {:?}", expr, inferred);
                let identifier: String = ident.generate(context);
                context.insert_inferred_type(&identifier, inferred);
            }
            Statement::IfStatement { suite, .. } => {
                context.push_scope();
                suite.infer(context);
                context.pop_scope();
            }
            Statement::FunctionDecl { body, .. } => {
                context.push_scope();
                body.infer(context);
                context.pop_scope();
            }
            _ => (),
        }
    }
}
