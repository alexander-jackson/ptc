use std::iter::FromIterator;

extern crate ptc;

use ptc::lexer::Tok::*;
use ptc::lexer::*;

macro_rules! lex {
    ($($name:ident: $input:expr, $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: &str = $input;
                let tokens = get_lexer_tokens(input);
                assert_eq!(tokens, $expected);
            }
        )*
    }
}

fn get_lexer_tokens(input: &str) -> Vec<Tok> {
    let lexer = Lexer::new(input.char_indices());

    Vec::from_iter(lexer.map(|x| x.unwrap().1))
}

lex! {
    newline_token: "\n\n", vec![Newline, Newline],
    identifier: "name", vec![Identifier { name: String::from("name") }],
    identifier_with_number: "func2", vec![Identifier { name: String::from("func2") }],
    identifier_with_underscore: "common_divisor", vec![Identifier { name: String::from("common_divisor") }],
    identifier_with_leading_underscore: "__init__", vec![Identifier { name: String::from("__init__") }],
    identifier_with_leading_number: "2func", vec![Integer { value: 2 }, Identifier { name: String::from("func") }],
    operators: "+-*/%", vec![Plus, Minus, Multiply, Divide, Modulo],
    literals: "1\n1.0", vec![Integer { value: 1 }, Newline, Float { value: 1.0 }],
    augmented_operators: "+=-=*=/=%=", vec![PlusEquals, MinusEquals, MultiplyEquals, DivideEquals, ModuloEquals],
    punctuation: "()[]:;,", vec![LPar, RPar, LSquare, RSquare, Colon, Semicolon, Comma],
    keywords: "if else while pass def and or not", vec![If, Else, While, Pass, Def, LogicalAnd, LogicalOr, LogicalNot],
    simple_indentation: "if condition:\n\tpass\n", vec![If, Identifier { name: String::from("condition") }, Colon, Newline, Indent, Pass, Newline, Unindent],
    nested_indentation: "if condition:\n\tif other:\n\t\tpass\n", vec![If, Identifier { name: String::from("condition") }, Colon, Newline, Indent, If, Identifier { name: String::from("other") }, Colon, Newline, Indent, Pass, Newline, Unindent, Unindent],
}

#[test]
#[should_panic]
fn mixed_indentation() {
    let input: &str = "if:\n\tpass\n    else:\n    pass";
    let _ = get_lexer_tokens(input);
}
