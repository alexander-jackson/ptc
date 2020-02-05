use ast::Program;
use ast::{Context, Infer};

impl Infer for Program {
    fn infer(&mut self, context: &mut Context) {
        for stmt in &mut self.statements {
            stmt.infer(context);
        }
    }
}
