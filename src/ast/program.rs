use ast::{Context, Generate, Infer, Suite, VariableType};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Suite,
}

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

impl Infer for Program {
    fn infer(&mut self, context: &mut Context) {
        for stmt in &mut self.statements {
            stmt.infer(context);
        }
    }
}
