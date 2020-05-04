Python to C Compiler (ptc)
--------------------------
ptc is a source-to-source compiler for converting Python to C code. It aims to
allow converted code to be dropped into a project as if it had been
handwritten. It is written using the [Rust](https://www.rust-lang.org/)
programming language.

## Installation

The compiler can be installed by cloning the respository and using `cargo`:

```bash
git clone git@github.com:alexander-jackson/ptc.git
cd ptc/
cargo install --path .
```

Alternatively, you can build and run the compiler using the `cargo run --
{ARGS}` command.

## Usage

The compiler can be used as follows:

```bash
alexander@cinnamon ~/D/U/P/r/ptc> ptc --help

ptc (Python to C Compiler)
Transpiles code from Python to C

USAGE:
    ptc [FLAGS] [OPTIONS] [PATH(S)]

FLAGS:
    --ast               Displays the abstract syntax tree after parsing
    --tokens            Displays the tokens output by the lexer for the given input
    --display           Displays the output code to the screen instead of writing to a file
    -h, --help          Prints this help information

ARGS:
    <PATH(S)>           Paths of Python files to transpile
```

## Tests

Tests live in the `tests/` directory. Each file tests a different component of
the compiler. You can run all the tests using the `cargo test` command. This
will also run the doctests that are written for some of the methods.

Fuzzing can be performed by using the `cargo fuzz run ptc` command to stress
test the lexer for any unexpected `panic!()`s.

## Documentation

The documentation for the compiler is written using `rustdoc`. You can build
and open the documentation using `cargo doc --no-deps --document-private-items
--open`. This will build the documentation for only the compiler, including the
non-public marked sections, and open the resulting documentation in a browser.
