use crate::*;

#[test]
fn allow_underscores_in_identifiers_test() {
    let input: &str = r#"
longer_name
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_integers_test() {
    let input: &str = r#"
40
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_expressions_test() {
    let input: &str = r#"
var = 1 / 1 * 1 + 1 - 1
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_bracketed_expression_test() {
    let input: &str = r#"
var = (1 / 1) * 1
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_comparison_operators_test() {
    let input: &str = r#"
var = 0 < 1
var = 0 > 1
var = 0 <= 1
var = 0 >= 1
var = 0 == 1
var = 0 != 1
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_augmented_assignment_test() {
    let input: &str = r#"
var += 1
var -= 1
var *= 1
var /= 1
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_identifier_expression_test() {
    let input: &str = r#"
var = var_one + var_two
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_function_call_expression_test() {
    let input: &str = r#"
var = add(1, 2)
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_logical_and_expression_test() {
    let input: &str = r#"
variable and other_variable
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_logical_or_expression_test() {
    let input: &str = r#"
variable or other_variable
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_logical_not_expression_test() {
    let input: &str = r#"
not other_variable
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_pass_statement_test() {
    let input: &str = r#"
pass
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_if_statement_test() {
    let input: &str = r#"
if 1:
    pass
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_if_else_statement_test() {
    let input: &str = r#"
if 1:
    pass
else:
    pass
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_while_statement_test() {
    let input: &str = r#"
while 1:
    pass
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_nested_compound_statements_test() {
    let input: &str = r#"
while expression:
    if other_expression:
        pass
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_function_declaration_test() {
    let input: &str = r#"
def useless(x, y):
    pass
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_return_statement_test() {
    let input: &str = r#"
def add(x, y):
    return x + y
"#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}
