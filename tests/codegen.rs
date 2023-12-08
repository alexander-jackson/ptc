extern crate ptc;

use ptc::ast::{Context, Generate, Infer};

fn get_output(input: &str) -> String {
    let parser = ptc::parser::ProgramParser::new();
    let lexer = ptc::lexer::Lexer::new(input.char_indices());
    let mut ast = parser.parse(lexer).unwrap();

    let mut context = Context::default();
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
    unary_operation_type_inference: "x = 0.1\ny = -x\n", "float x = 0.1; float y = -x;\n",
    paren_expression_type_inference: "x = (0.1)\n", "float x = (0.1);\n",
    variable_return_inference: "def variable_return_inference():\n    x = 0\n    return x\n", "int variable_return_inference() { int x = 0; return x; }\n",
    float_variable_return_inference: "def float_variable_return():\n    x = 0.1\n    return x\n", "float float_variable_return() { float x = 0.1; return x; }\n",
    global_statements: "x = 1\n\ndef float_variable_return():\n    global x\n    x = 2\n    return x\n", "int x = 1; int float_variable_return() {  x = 2; return x; }\n",
    empty_list_creation: "def main():\n    x: List[int] = []\n", "#include \"list.h\"\nvoid main() { list_int* x = list_int_new(); }\n",
    list_append: "def main():\n    x: List[int] = []\n    x.append(1)\n", "#include \"list.h\"\nvoid main() { list_int* x = list_int_new(); list_int_append(x, 1); }\n",
    function_return_type_propagation: "def create_list():\n    x: List[int] = []\n    return x\n\ndef main():\n    x = create_list()\n    x.append(1)\n", "#include \"list.h\"\nlist_int* create_list() { list_int* x = list_int_new(); return x; } void main() { list_int* x = create_list(); list_int_append(x, 1); }\n",
    array_access: "def int():\n    x: List[int] = []\n    y: int = x[0]\n", "#include \"list.h\"\nvoid int() { list_int* x = list_int_new(); int y = x->data[0]; }\n",
    array_access_type_inference: "def int():\n    x: List[int] = []\n    y = x[0]\n\ndef float():\n    x: List[float] = []\n    y = x[0]\n", "#include \"list.h\"\nvoid int() { list_int* x = list_int_new(); int y = x->data[0]; } void float() { list_float* x = list_float_new(); float y = x->data[0]; }\n",
    list_display_type_inference: "def main():\n    x: List[float] = []\n", "#include \"list.h\"\nvoid main() { list_float* x = list_float_new(); }\n",
    list_index_type_inference: "def main():\n    x: List[float] = []\n    x.append(0.1)\n", "#include \"list.h\"\nvoid main() { list_float* x = list_float_new(); list_float_append(x, 0.1); }\n",
    function_argument_type_inference: "def process(data, index):\n    pass\n\ndef main():\n    integers: List[int] = []\n    i = 0.1\n    process(integers, i)\n", "#include \"list.h\"\nvoid process(list_int* data, float index) {  } void main() { list_int* integers = list_int_new(); float i = 0.1; process(integers, i); }\n",
    list_subscription_assign: "def main():\n    x: List[float] = []\n    x[0] = 0\n", "#include \"list.h\"\nvoid main() { list_float* x = list_float_new(); x->data[0] = 0; }\n",
    list_subscription_augmented_assign: "def main():\n    x: List[float] = []\n    x[0] += 0\n", "#include \"list.h\"\nvoid main() { list_float* x = list_float_new(); x->data[0] += 0; }\n",
    built_in_len_function: "def main():\n    x: List[int] = []\n    len(x)\n", "#include \"list.h\"\nvoid main() { list_int* x = list_int_new(); x->size; }\n",
    binary_expression_type_inference: "a = 0.1\nb = 0.1\nc = a + b\n\nx = 0\ny = 0.1\nz = x + y\n\nd = 0\ne = 0\nf = d + e\n", "float a = 0.1; float b = 0.1; float c = a + b; int x = 0; float y = 0.1; float z = x + y; int d = 0; int e = 0; int f = d + e;\n",
    division_expression_type_inference: "x = 1 / 2\n", "float x = 1 / 2;\n",
    ignore_del_on_non_lists: "a = 0.1\ndel a\n", "float a = 0.1; \n",
    del_becomes_free_on_lists: "def main():\n    x: List[int] = []\n    del x\n", "#include \"list.h\"\nvoid main() { list_int* x = list_int_new(); list_int_free(x); }\n",
    multiple_deletes_become_frees: "def main():\n    a: List[int] = []\n    b: List[int] = []\n    del a, b\n", "#include \"list.h\"\nvoid main() { list_int* a = list_int_new(); list_int* b = list_int_new(); list_int_free(a); list_int_free(b); }\n",
    always_use_function_return_typehint: "def add(x, y) -> List[int]:\n    return 0\n", "#include \"list.h\"\nlist_int* add(unknown x, unknown y) { return 0; }\n",
    use_typehint_even_without_return: "def add(x, y) -> int:\n    pass\n", "int add(unknown x, unknown y) {  }\n",
    allow_specifying_void_return: "def add() -> None:\n    pass\n", "void add() {  }\n",
    disallow_overriding_void_return: "def add(x, y) -> None:\n    return y\n", "void add(unknown x, unknown y) { return y; }\n",
    allow_typehints_in_function_declarations: "def add(x: int, y: int) -> None:\n    pass\n", "void add(int x, int y) {  }\n",
    allow_mixed_typehints_in_function_declarations: "def add(a: int, b: float, c: List[int], d: List[float]) -> None:\n    pass\n", "#include \"list.h\"\nvoid add(int a, float b, list_int* c, list_float* d) {  }\n",
    use_typehints_in_function_body: "def add_integers(x: int, y: int):\n    return x + y\n\ndef add_mixed(x: int, y: float):\n    return x + y\n", "int add_integers(int x, int y) { return x + y; } float add_mixed(int x, float y) { return x + y; }\n",
    previous_type_inference_used_in_function_body: "add_integers(1, 1.0)\n\ndef add_integers(x, y):\n    return x + y\n", "add_integers(1, 1); float add_integers(int x, float y) { return x + y; }\n",
    previous_inferred_type_overwritten: "add_integers(1, 0.5)\n\ndef add_integers(x, y: int):\n    return x + y\n", "add_integers(1, 0.5); int add_integers(int x, int y) { return x + y; }\n",
    unknown_types_ignored_for_typehint: "value_pi = pi()\nadd_pi(0.5, value_pi)\n\ndef add_pi(x: float, y: float):\n    return x + y\n\n", "unknown value_pi = pi(); add_pi(0.5, value_pi); float add_pi(float x, float y) { return x + y; }\n",
    duck_typing_for_integer_lists: "def main():\n    x = []\n    x.append(1)\n", "#include \"list.h\"\nvoid main() { list_int* x = list_int_new(); list_int_append(x, 1); }\n",
    duck_typing_for_float_expressions: "def main():\n    x = []\n    x.append(b / c)\n", "#include \"list.h\"\nvoid main() { list_float* x = list_float_new(); list_float_append(x, b / c); }\n",
    shallow_duck_typing_for_literals: "a = []\n\ndef main():\n    initialise_global_lists()\n    a.append(0)\n", "#include \"list.h\"\nlist_int* a; void main() { initialise_global_lists(); list_int_append(a, 0); }\nvoid initialise_global_lists() { a = list_int_new(); }\n",
    shallow_duck_typing_for_variables: "a = []\n\ndef main():\n    initialise_global_lists()\n    b: int = 0\n    a.append(b)\n", "#include \"list.h\"\nlist_int* a; void main() { initialise_global_lists(); int b = 0; list_int_append(a, b); }\nvoid initialise_global_lists() { a = list_int_new(); }\n",
    shallow_duck_typing_for_expressions: "a = []\n\ndef main():\n    initialise_global_lists()\n    b = 10\n    c = 3\n    a.append(b / c)\n", "#include \"list.h\"\nlist_float* a; void main() { initialise_global_lists(); int b = 10; int c = 3; list_float_append(a, b / c); }\nvoid initialise_global_lists() { a = list_float_new(); }\n",
    only_global_lists_are_initialised: "a = []\n\ndef main():\n    b = []\n    initialise_global_lists()\n    a.append(0)\n    b.append(0)\n", "#include \"list.h\"\nlist_int* a; void main() { list_int* b = list_int_new(); initialise_global_lists(); list_int_append(a, 0); list_int_append(b, 0); }\nvoid initialise_global_lists() { a = list_int_new(); }\n",
    duck_typing_len_infers_list: "def main():\n    a = []\n    b = len(a)\n", "#include \"list.h\"\nvoid main() { list_unknown* a = list_unknown_new(); int b = a->size; }\n",
    elif_statement: "def main():\n    x = 0;\n    bound = 0;\n\n    if x < 0:\n        bound = -1\n    elif x == 0:\n        bound = 0\n    else:\n        bound = 1\n", "void main() { int x = 0; int bound = 0; if (x < 0) { bound = -1; } else if (x == 0) { bound = 0; } else { bound = 1; } }\n",
    multiple_elif_statements: "def main():\n    x: int = 3\n    y: int = 1\n    bound: int = 0\n\n    if x < 0 and y < 0:\n        bound = -2\n    elif x < 0 and y > 0:\n        bound = -1\n    elif x > 0 and y < 0:\n        bound = 1\n    else:\n        bound = 2\n", "void main() { int x = 3; int y = 1; int bound = 0; if (x < 0 && y < 0) { bound = -2; } else if (x < 0 && y > 0) { bound = -1; }  else if (x > 0 && y < 0) { bound = 1; } else { bound = 2; } }\n",
}
