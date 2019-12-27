use std::iter::FromIterator;

extern crate ptc;

fn get_lexer_tokens(input: &str) -> Vec<ptc::lexer::Tok> {
    let lexer = ptc::lexer::Lexer::new(input.char_indices());

    Vec::from_iter(lexer.map(|x| x.unwrap().1))
}

#[test]
fn lex_all_tokens_test() {
    let input: &str = "\n\n";

    use ptc::lexer::Tok::*;

    let tokens = get_lexer_tokens(input);
    let expected = vec![Newline, Newline];

    assert_eq!(tokens, expected);
}
