use super::*;

#[test]
fn ignore_whitespace_test() {
    let input: &str = r#"
    name =    4;
        base = 3;
    "#;

    let ast = parser::ProgramParser::new()
        .parse(lexer::Lexer::new(input))
        .unwrap();

    let expected_ast = ast::Program {
        stmts: vec![
            ast::Stmt::Statement(
                ast::Identifier::Name { name: "name" },
                ast::Operator::Assign,
                ast::Number::Integer { value: 4 },
            ),
            ast::Stmt::Statement(
                ast::Identifier::Name { name: "base" },
                ast::Operator::Assign,
                ast::Number::Integer { value: 3 },
            ),
        ],
    };

    assert_eq!(&ast, &expected_ast);
}
