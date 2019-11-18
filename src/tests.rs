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
        .parse(lexer::Lexer::new(input))
        .unwrap();

    let expected_ast = Program {
        stmts: vec![
            Stmt::Statement(
                Identifier::Name { name: "name" },
                Operator::Assign,
                Expression::Literal { value:
                    Number::Integer { value: 4 },
                },
            ),
            Stmt::Statement(
                Identifier::Name { name: "base" },
                Operator::Assign,
                Expression::Literal { value:
                    Number::Integer { value: 3 },
                },
            ),
            Stmt::Statement(
                Identifier::Name { name: "newline" },
                Operator::Assign,
                Expression::Literal { value:
                    Number::Integer { value: 1 },
                },
            ),
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
        .parse(lexer::Lexer::new(input))
        .unwrap();

    let expected_ast = Program {
        stmts: vec![
            Stmt::Statement(
                Identifier::Name { name: "longer_name" },
                Operator::Assign,
                Expression::Literal { value:
                    Number::Integer { value: 3 },
                },
            ),
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
        .parse(lexer::Lexer::new(input))
        .unwrap();

    let expected_ast = Program {
        stmts: vec![
            Stmt::Statement(
                Identifier::Name { name: "name" },
                Operator::Assign,
                Expression::Literal { value:
                    Number::Integer { value: 40 },
                },
            )
        ],
    };

    assert_eq!(&ast, &expected_ast);
}

#[test]
fn parse_expressions_test() {
    let input: &str = r#"
    var = 1 / 1 * 1 + 1 - 1;
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input));

    assert!(ast.is_ok());
}

#[test]
fn parse_bracketed_expression_test() {
    let input: &str = r#"
    var = (1 / 1) * 1;
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input));

    assert!(ast.is_ok());
}
