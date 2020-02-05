use ast::Program;
use ast::{Context, Generate};

impl Generate for Program {
    fn generate(&self, context: &mut Context) -> String {
        // Generate all the statements and collect them
        let mut code: Vec<String> = Vec::new();

        for stmt in &self.statements {
            code.push(stmt.generate(context));
        }

        // Add a blank line at the end of the file
        code.push(String::from(""));

        code.join("\n")
    }
}
