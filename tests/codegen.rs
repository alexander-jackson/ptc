extern crate ptc;

use ptc::ast::{Context, Generate, Infer};

fn get_output(input: &str) -> String {
    let parser = ptc::parser::ProgramParser::new();
    let lexer = ptc::lexer::Lexer::new(input.char_indices());
    let mut ast = parser.parse(lexer).unwrap();

    let mut context = Context::new();
    ast.infer(&mut context);

    context.reset_position();
    ast.generate(&mut context)
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
    integer: "1\n", "1;\n",
    float: "0.1\n", "0.1;\n",
    identifier: "name\n", "name;\n",
    function_call: "add(1, 2)\n", "add(1, 2);\n",
    integer_assignment: "x = 0\n", "int x = 0;\n",
    float_assignment: "x = 0.1\n", "float x = 0.1;\n",
    integer_augmented_assignment: "x += 1\n", "x += 1;\n",
    float_augmented_assignment: "x += 0.1\n", "x += 0.1;\n",
    expression_statement: "3 + 4\n", "3 + 4;\n",
    pass_statement: "pass\n", "\n",
    if_statement: "if 1:\n    pass\n", "if (1) {  }\n",
    if_else_statement: "if 1:\n    pass\nelse:\n    pass\n", "if (1) {  } else {  }\n",
    while_statement: "while 1:\n    pass\n", "while (1) {  }\n",
    return_statement: "return x\n", "return x;\n",
    function_declaration_statement: "def useless():\n    pass\n", "int useless() {  }\n",
    type_inference: "if 1:\n    x = 0\nelse:\n    x = 0.1\n", "if (1) { int x = 0; } else { float x = 0.1; }\n",
    layered_type_inference: "if 1:\n\tx = 0\nx = 0.0\n", "if (1) { int x = 0; }\nfloat x = 0;\n",
    preinitialised_layered_type_inference: "x = 0.1\nif 1:\n\tx = 0\n", "float x = 0.1;\nif (1) { x = 0; }\n",
}
