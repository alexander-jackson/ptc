use ast::Program;
use ast::{Context, Generate};

impl Generate for Program {
    fn generate(&self, context: &mut Context) -> String {
        let code = self.statements.generate(context);
        // Must run 2nd, otherwise no includes exist yet
        let includes = context.generate_includes();

        if includes.is_empty() {
            format!("{}\n", code)
        } else {
            format!("{}\n{}\n", includes, code)
        }
    }
}
