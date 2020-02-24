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
    empty_return_statement: "return\n", "return;\n",
    function_declaration_statement: "def useless():\n    pass\n", "void useless() {  }\n",
    type_inference: "if 1:\n    x = 0\nelse:\n    x = 0.1\n", "if (1) { int x = 0; } else { float x = 0.1; }\n",
    layered_type_inference: "if 1:\n\tx = 0\nx = 0.0\n", "if (1) { int x = 0; } float x = 0;\n",
    preinitialised_layered_type_inference: "x = 0.1\nif 1:\n\tx = 0\n", "float x = 0.1; if (1) { x = 0; }\n",
    basic_type_hints: "x: float = 0\n", "float x = 0;\n",
    integer_return_inference: "def one():\n    return 1\n", "int one() { return 1; }\n",
    float_return_inference: "def float_half():\n    return 0.5\n", "float float_half() { return 0.5; }\n",
    variable_return_inference: "def variable_return_inference():\n    x = 0\n    return x\n", "int variable_return_inference() { int x = 0; return x; }\n",
    float_variable_return_inference: "def float_variable_return():\n    x = 0.1\n    return x\n", "float float_variable_return() { float x = 0.1; return x; }\n",
    global_statements: "x = 1\n\ndef float_variable_return():\n    global x\n    x = 2\n    return x\n", "int x = 1; int float_variable_return() {  x = 2; return x; }\n",
    empty_list_creation: "def main():\n    x: List[int] = []\n", "void main() { list_int* x = list_int_new(); }\n",
    list_append: "def main():\n    x: List[int] = []\n    x.append(1)\n", "void main() { list_int* x = list_int_new(); list_int_append(x, 1); }\n",
    function_return_type_propagation: "def create_list():\n    x: List[int] = []\n    return x\n\ndef main():\n    x = create_list()\n    x.append(1)\n", "list_int* create_list() { list_int* x = list_int_new(); return x; } void main() { list_int* x = create_list(); list_int_append(x, 1); }\n",
    array_access: "def int():\n    x: List[int] = []\n    y: int = x[0]\n", "void int() { list_int* x = list_int_new(); int y = x->data[0]; }\n",
    array_access_type_inference: "def int():\n    x: List[int] = []\n    y = x[0]\n\ndef float():\n    x: List[float] = []\n    y = x[0]\n", "void int() { list_int* x = list_int_new(); int y = x->data[0]; } void float() { list_float* x = list_float_new(); float y = x->data[0]; }\n",
    list_display_type_inference: "x: List[float] = []\n", "list_float* x = list_float_new();\n",
    list_index_type_inference: "x: List[float] = []\nx.append(0.1)\n", "list_float* x = list_float_new(); list_float_append(x, 0.1);\n",
    function_argument_type_inference: "def process(data, index):\n    pass\n\ndef main():\n    integers: List[int] = []\n    i = 0.1\n    process(integers, i)\n", "void process(list_int* data, float index) {  } void main() { list_int* integers = list_int_new(); float i = 0.1; process(integers, i); }\n",
    list_subscription_assign: "integers: List[int] = []\nintegers[0] = 0\n", "list_int* integers = list_int_new(); integers->data[0] = 0;\n",
    list_subscription_augmented_assign: "integers: List[int] = []\nintegers[0] += 0\n", "list_int* integers = list_int_new(); integers->data[0] += 0;\n",
    built_in_len_function: "integers: List[int] = []\nlen(integers)\n", "list_int* integers = list_int_new(); integers->size;\n",
}
