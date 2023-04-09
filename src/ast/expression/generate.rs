//! Implements the `Generate` trait for `Expression`.

use crate::ast::Expression;
use crate::ast::{Context, DataType, Generate, VariableType};

impl Generate for Expression {
    /// Produces the C code for a given `Expression`.
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
            Expression::ListDisplay => unreachable!("This is handled at the Statement level."),
            Expression::FunctionCall { name, args } => {
                // Generate the arguments if they exist
                let arg_str = match args {
                    Some(s) => s
                        .iter()
                        .map(|a| a.generate(context))
                        .collect::<Vec<String>>()
                        .join(", "),
                    None => String::new(),
                };

                // Check if this is the `len` function
                if let Some(s) = check_builtin(name, &arg_str) {
                    return s;
                }

                // Check for <list>.append(<args>)
                if let Expression::AttributeRef { primary, attribute } = &**name {
                    if let "append" = attribute.generate(context).as_ref() {
                        if let Some(VariableType::List { elements }) = primary.get_type(context) {
                            if let Some(elements) = elements {
                                return format!(
                                    "list_{}_append({}, {})",
                                    String::from(&*elements),
                                    primary.generate(context),
                                    arg_str
                                );
                            }
                        }
                    }
                }

                // This is a normal function call, so just format it
                format!("{}({})", name.generate(context), arg_str)
            }
            Expression::AttributeRef { .. } => String::from(""),
            Expression::Subscription { primary, expr } => {
                let primary_gen = primary.generate(context);
                let expr_gen = expr.generate(context);

                if let Some(t) = context.get_type(&primary_gen) {
                    if let VariableType::List { .. } = t {
                        return format!("{}->data[{}]", primary_gen, expr_gen);
                    }
                }

                String::from("")
            }
            Expression::Identifier { name } => name.generate(context),
            Expression::Literal { value } => value.generate(context),
        }
    }
}

/// Checks if the name of the function is a built-in and handles it accordingly.
fn check_builtin(name: &Expression, args: &str) -> Option<String> {
    if let Expression::Identifier { name } = name {
        if let "len" = name.get_identifier().as_ref() {
            return Some(format!("{}->size", args));
        }
    }

    None
}
