use ast::Expression;
use ast::{Context, DataType, Generate, VariableType};

impl DataType for Expression {
    fn get_type(&self, context: &mut Context) -> VariableType {
        match self {
            Expression::Literal { value } => value.get_type(context),
            Expression::Identifier { name } => name.get_type(context),
            Expression::FunctionCall { name, .. } => {
                if let Expression::Identifier { name } = &**name {
                    let identifier = name.generate(context);
                    if let Some(t) = context.get_function_return_type(&identifier) {
                        return t.clone();
                    }
                }

                VariableType::Integer
            }
            _ => VariableType::Integer,
        }
    }
}
