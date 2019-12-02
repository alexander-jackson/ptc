use ast::Suite;
use ast::Generate;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Suite,
}

impl Generate for Program {
    fn generate(&self) -> String {
        // Generate all the statements and collect them
        let mut code: Vec<String> = Vec::new();

        for stmt in &self.statements {
            code.push(stmt.generate());
        }

        code.join("\n")
    }
}
