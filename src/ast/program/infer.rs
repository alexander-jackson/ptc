use crate::ast::{Context, Infer, Program};

impl Infer for Program {
    fn infer(&mut self, context: &mut Context) {
        for stmt in &mut self.statements {
            stmt.infer(context);
        }
    }
}
