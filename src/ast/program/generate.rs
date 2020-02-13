use ast::Program;
use ast::{Context, Generate};

impl Generate for Program {
    fn generate(&self, context: &mut Context) -> String {
        format!("{}\n", self.statements.generate(context))
    }
}
