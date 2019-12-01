use std::iter::FromIterator;

use crate::*;

fn get_lexer_tokens(input: &str) -> Vec<lexer::Tok> {
    let lexer = lexer::Lexer::new(input.char_indices());

    Vec::from_iter(lexer.map(|x| x.unwrap().1))
}

#[test]
fn lex_keywords_test() {
    let input: &str = r#"
pass or and not if while def return
"#;

    use lexer::Tok::*;

    let tokens = get_lexer_tokens(input);
    let expected = vec![
        Newline, Pass, LogicalOr, LogicalAnd, LogicalNot, If, While, Def, Return, Newline,
    ];

    assert_eq!(tokens, expected);
}
