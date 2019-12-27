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
    generate_integer: "1\n", "1;\n",
    generate_identifier: "name\n", "name;\n",
    generate_function_call: "add(1, 2)\n", "add(1, 2);\n",
    generate_assign_statement: "x = 0\n", "x = 0;\n",
    generate_augmented_assign_statement: "x += 1\n", "x += 1;\n",
    generate_expression_statement: "3 + 4\n", "3 + 4;\n",
    generate_pass_statement: "pass\n", "\n",
    generate_if_statement: "if 1:\n    pass\n", "if (1) {  }\n",
    generate_if_else_statement: "if 1:\n    pass\nelse:\n    pass\n", "if (1) {  } else {  }\n",
    generate_while_statement: "while 1:\n    pass\n", "while (1) {  }\n",
    generate_return_statement: "return x\n", "return x;\n",
    generate_function_declaration_statement: "def useless():\n    pass\n", "int useless() {  }\n",
}