use crate::*;

fn get_ast(input: &str) -> Result<ast::program::Program, String> {
    let parser = parser::ProgramParser::new();
    let lexer = lexer::Lexer::new(input.char_indices());

    parser.parse(lexer).map_err(|e| format!("{:?}", e))
}

macro_rules! parser_success {
    ($($name:ident: $expr:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: &str = $expr;
                let ast = get_ast(input);
                assert!(ast.is_ok());
            }
        )*
    }
}

parser_success! {
    allow_underscores_in_identifiers: "longer_name\n",
    parse_integers: "40\n",
    parse_expressions: "var = 1 + 1 - 1 * 1 / 1\n",
    parse_bracketed_expressions: "var = (1 - 1) * 1\n",
    parse_comparison_operators: r#"
var = 0 < 1
var = 0 > 1
var = 0 <= 1
var = 0 >= 1
var = 0 == 1
var = 0 != 1
"#,
    parse_augmented_assignments: r#"
var += 1
var -= 1
var *= 1
var /= 1
"#,
    parse_identifier_expression: "var = var_one + var_two\n",
    parse_function_call: "var = add(1, 2)\n",
    parse_logical_expressions: "a and b\na or b\nnot a\n",
    parse_pass_statement: "pass\n",
    parse_if_statement: r#"
if 1:
    pass
"#,
    parse_if_else_statement: r#"
if 1:
    pass
else:
    pass
"#,
    parse_while_statement: r#"
while 1:
    pass
"#,
    parse_nested_compound_statements: r#"
while expression:
    if other_expression:
        pass
"#,
    parse_function_declaration: r#"
def useless(x, y):
    pass
"#,
    parse_return_statement: r#"
def add(x, y):
    return x + y
"#,
}
