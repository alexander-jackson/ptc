use std::iter::FromIterator;

use crate::*;

fn get_lexer_tokens(input: &str) -> Vec<lexer::Tok> {
    let lexer = lexer::Lexer::new(input.char_indices());

    Vec::from_iter(lexer.map(|x| x.unwrap().1))
}

#[test]
fn lex_all_tokens_test() {
    let input: &str = r#"

"#;

    use lexer::Tok::*;

    let tokens = get_lexer_tokens(input);
    let expected = vec![Newline, Newline];

    assert_eq!(tokens, expected);
}
