use ast::Statement;
use ast::{Context, DataType, Generate, Identifier, Infer};

impl Infer for Statement {
    fn infer(&mut self, context: &mut Context) {
        match self {
            Statement::Assign { ident, expr } => {
                // If the variable has a typehint, use that, otherwise infer
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
            Statement::ReturnStatement { expr } => {
                // If the statement returns a value, get the type of it
                if let Some(e) = expr {
                    let datatype = e.get_type(context);
                    context.set_function_return_type(datatype);
                }
            }
            Statement::FunctionDecl { name, body, .. } => {
                context.push_scope();
                let function_name = name.generate(context);
                context.set_current_function(Some(function_name));
                body.infer(context);
                context.pop_scope();
                context.set_current_function(None);
            }
            _ => (),
        }
    }
}
