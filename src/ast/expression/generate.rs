use ast::Expression;
use ast::{Context, DataType, Generate, VariableType};

impl Generate for Expression {
    fn generate(&self, context: &mut Context) -> String {
        match self {
            Expression::BinaryOperation { left, op, right } => format!(
                "{} {} {}",
                left.generate(context),
                op.generate(context),
                right.generate(context)
            ),
            Expression::UnaryOperation { op, expr } => {
                format!("{}{}", op.generate(context), expr.generate(context))
            }
            Expression::ParenExpression { expr } => format!("({})", expr.generate(context)),
            Expression::ListDisplay => String::from("list_int_new()"),
            Expression::FunctionCall { name, args } => {
                let arg_str: Option<String> = args.as_ref().map(|s| {
                    s.iter()
                        .map(|a| a.generate(context))
                        .collect::<Vec<String>>()
                        .join(", ")
                });

                let arg_str = arg_str.unwrap_or_else(|| String::from(""));

                // Check for <list>.append(<args>)
                if let Expression::AttributeRef { primary, attribute } = &**name {
                    if let VariableType::List { elements } = primary.get_type(context) {
                        if let "append" = attribute.generate(context).as_ref() {
                            return format!(
                                "list_{}_append({}, {})",
                                String::from(*elements),
                                primary.generate(context),
                                arg_str
                            );
                        }
                    }
                }

                format!("{}({})", name.generate(context), arg_str)
            }
            Expression::AttributeRef { .. } => String::from(""),
            Expression::Subscription { primary, expr } => {
                let primary_gen = primary.generate(context);
                let expr_gen = expr.generate(context);

                if let Some(t) = context.get_type(&primary_gen) {
                    if let VariableType::List { .. } = t {
                        return format!("list_int_get({}, {})", primary_gen, expr_gen);
                    }
                }

                String::from("")
            }
            Expression::Identifier { name } => name.generate(context),
            Expression::Literal { value } => value.generate(context),
        }
    }
}
