use ast::Statement;
use ast::{Context, DataType, Generate, Identifier, Infer};

impl Infer for Statement {
    fn infer(&mut self, context: &mut Context) {
        match self {
            Statement::Assign { ident, expr } => {
                let inferred = match ident {
                    Identifier::Typed { .. } => ident.get_type(context),
                    _ => expr.get_type(context),
                };

                let identifier: String = ident.generate(context);
                context.insert_inferred_type(&identifier, inferred);
            }
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
            Statement::FunctionDecl { body, .. } => {
                context.push_scope();
                body.infer(context);
                context.pop_scope();
            }
            _ => (),
        }
    }
}
