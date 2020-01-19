use ast::Generate;
use ast::Suite;
use ast::Context;

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
