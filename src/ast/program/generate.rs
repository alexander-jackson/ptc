use crate::ast::{Context, Generate, Program, VariableType};

impl Generate for Program {
    fn generate(&self, context: &mut Context) -> String {
        let code = format!("{}\n", self.statements.generate(context));
        // Must run 2nd, otherwise no includes exist yet
        // Generate the global list initialiser
        let gli = context
            .generate_global_list_initialiser()
            .map(|initialiser| {
                context.set_current_function(String::from("initialise_global_lists"));
                context.set_function_return_type(VariableType::Void);
                format!("{}\n", initialiser)
            })
            .unwrap_or_default();

        let includes = context.generate_includes();

        if includes.is_empty() {
            format!("{}{}", code, gli)
        } else {
            format!("{}\n{}{}", includes, code, gli)
        }
    }
}
