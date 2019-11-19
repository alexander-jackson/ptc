use super::*;

use ast::*;

#[test]
fn ignore_whitespace_test() {
    let input: &str = r#"
    name =    4;
        base = 3;

    newline = 1;
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input.char_indices()))
        .unwrap();

    let expected_ast = Program {
        statements: vec![
            Statement::Assign {
                ident: Identifier::Name {
                    name: String::from("name"),
                },
                expr: Expression::Literal {
                    value: Number::Integer { value: 4 },
                },
            },
            Statement::Assign {
                ident: Identifier::Name {
                    name: String::from("base"),
                },
                expr: Expression::Literal {
                    value: Number::Integer { value: 3 },
                },
            },
            Statement::Assign {
                ident: Identifier::Name {
                    name: String::from("newline"),
                },
                expr: Expression::Literal {
                    value: Number::Integer { value: 1 },
                },
            },
        ],
    };

    assert_eq!(&ast, &expected_ast);
}

#[test]
fn allow_underscores_in_identifiers_test() {
    let input: &str = r#"
    longer_name = 3;
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input.char_indices()))
        .unwrap();

    let expected_ast = Program {
        statements: vec![
            Statement::Assign {
                ident: Identifier::Name {
                    name: String::from("longer_name"),
                },
                expr: Expression::Literal {
                    value: Number::Integer { value: 3 },
                },
            }
        ],
    };

    assert_eq!(&ast, &expected_ast);
}

#[test]
fn parse_integers_test() {
    let input: &str = r#"
    name = 40;
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input.char_indices()))
        .unwrap();

    let expected_ast = Program {
        statements: vec![
            Statement::Assign {
                ident: Identifier::Name {
                    name: String::from("name"),
                },
                expr: Expression::Literal {
                    value: Number::Integer { value: 40 },
                },
            }
        ],
    };

    assert_eq!(&ast, &expected_ast);
}

#[test]
fn parse_expressions_test() {
    let input: &str = r#"
    var = 1 / 1 * 1 + 1 - 1;
    "#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_bracketed_expression_test() {
    let input: &str = r#"
    var = (1 / 1) * 1;
    "#;

    let ast = parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices()));

    assert!(ast.is_ok());
}

#[test]
fn parse_identifier_expression_test() {
    let input: &str = r#"
    var = var_one + 2;
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
