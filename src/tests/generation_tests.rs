use crate::*;

use ast::Generate;

fn get_output(input: &str) -> String {
    let parser = parser::ProgramParser::new();
    let lexer = lexer::Lexer::new(input.char_indices());

    parser.parse(lexer).map_err(|e| format!("{:?}", e)).unwrap().generate()
}


macro_rules! generate {
    ($($name:ident: $input:expr, $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: &str = $input;
                let output: &str = &get_output(input);
                let expected: &str = $expected;
                assert_eq!(output, expected);
            }
        )*
    }
}

generate! {
    generate_expression: "3 + 4\n", "3 + 4\n",
}
