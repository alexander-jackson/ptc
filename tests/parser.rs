extern crate ptc;

fn get_ast(input: &str) -> Result<ptc::ast::Program, String> {
    let parser = ptc::parser::ProgramParser::new();
    let lexer = ptc::lexer::Lexer::new(input.char_indices());

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

macro_rules! parser_failure {
    ($($name:ident: $expr:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: &str = $expr;
                let ast = get_ast(input);
                assert!(ast.is_err());
            }
        )*
    }
}

parser_success! {
    parse_underscores_in_identifiers: "longer_name\n",
    parse_integers: "40\n",
    parse_expressions: "var = 1 + 1 - 1 * 1 / 1 % 1\n",
    parse_unary_expressions: "+var\n-var\n",
    parse_bracketed_expressions: "var = (1 - 1) * 1\n",
    parse_comparison_operators: "var = 0 < 1\nvar = 0 > 1\nvar = 0 <= 1\nvar = 0 >= 1\nvar = 0 == 1\nvar = 0 != 1\n",
    parse_augmented_assignments: "var += 1\nvar -= 1\nvar *= 1\nvar /= 1\nvar %= 1\n",
    parse_identifier_expression: "var = var_one + var_two\n",
    parse_function_call: "var = add(1, 2)\n",
    parse_logical_expressions: "a and b\na or b\nnot a\n",
    parse_pass_statement: "pass\n",
    parse_if_statement: "if 1:\n    pass\n",
    parse_if_else_statement: "if 1:\n    pass\nelse:\n    pass\n",
    parse_while_statement: "while 1:\n    pass\n",
    parse_nested_compound_statements: "while expression:\n    if other_expression:\n        pass\n",
    parse_function_declaration: "def useless(x, y):\n    pass\n",
    parse_return_statement: "def add(x, y):\n    return x + y\n",
    parse_empty_function_declaration: "def useless():\n    pass\n",
    parse_empty_function_call: "useless()\n",
}

parser_failure! {
    fail_compound_without_indentation: "if 1:\npass\n",
    fail_function_call_with_trailing_comma: "add(1, 2,)\n",
    fail_function_definition_with_trailing_comma: "def add(x, y,):\n    pass\n",
}
