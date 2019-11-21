use super::*;

#[test]
fn ignore_whitespace_test() {
    let input: &str = r#"

        
     
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn allow_underscores_in_identifiers_test() {
    let input: &str = r#"
    longer_name
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_integers_test() {
    let input: &str = r#"
    40
    123
    23525
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input.char_indices()));

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
fn parse_identifier_expression_test() {
    let input: &str = r#"
    var = var_one + var_two
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
