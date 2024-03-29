//! Top level functions for the binary to use.
//!
//! The app contains functions related to parsing the arguments given to the program, dealing with
//! those arguments and processing the files and data given to it using the core of the compiler.

use std::env;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::ast::{Context, Generate, Infer};
use crate::{ast, lexer, parser};

/// The arguments that can be passed to the program.
pub struct Args {
    /// Whether the AST should be displayed, specified by `--ast`
    abstract_tree: bool,
    /// Whether we should display tokens, specified by `--tokens`
    tokens: bool,
    /// Whether we should display generated code, specified by `--display`
    display: bool,
    /// Whether the help message should be displayed, specified by `-h`, `--help` or no arguments
    help: bool,
    /// All paths that the compiler should be run on
    paths: Vec<OsString>,
}

/// Display the help message for the compiler, typically upon running the program with no
/// arguments or supplying the `-h` or `--help` flag.
fn display_help_message() {
    println!(
        r#"
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
"#
    );
}

/// Parse the arguments into an `Args` struct that can then be used to determine what the user
/// wants the program to do.
///
/// # Errors
///
/// If any of the free arguments are not UTF-8 encoded or any of the flags are left, this will
/// return an error variant.
pub fn get_arguments() -> Result<Args, Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    Ok(Args {
        abstract_tree: args.contains("--ast"),
        tokens: args.contains("--tokens"),
        display: args.contains("--display"),
        help: args.contains(["-h", "--help"]),
        paths: args.finish(),
    })
}

/// Process the arguments given to the program from `get_arguments()`. Process each path in turn as
/// normal.
///
/// # Errors
///
/// Errors can be caused if any of the paths being processed cause an error. These propagate up
/// from `process_path`
pub fn process_args(args: Args) -> Result<(), Box<dyn Error>> {
    // If the user provided -h, --help or left no paths to process
    if args.help || args.paths.is_empty() {
        display_help_message();
        return Ok(());
    }

    // Iterate through all paths given and process them
    for path in &args.paths {
        process_path(&path, &args)?;
    }

    Ok(())
}

/// Process a single path to a file. Generates code if required, as well as displaying abstract
/// syntax trees, tokens and the generated output if needed.
fn process_path(path: &OsStr, args: &Args) -> Result<(), Box<dyn Error>> {
    // Read contents of file and get the basename of the filepath
    let code: String = fs::read_to_string(&path)?;
    let basename = get_output_filename(&path);

    // If they asked for the tokens, just dump the tokens out
    if args.tokens {
        display_tokens(&code);
    }

    // Parse the file to get the AST, optionally displaying
    let mut ast = get_abstract_syntax_tree(&code, args.abstract_tree)?;

    // Infer as many types as we can in a single pass and return to the start
    let mut context = Context::default();
    ast.infer(&mut context);
    context.reset_position();

    // When given a filename with non-UTF8 chars this could fail
    if let Some(basename) = basename {
        // Add the header as an include
        context.add_include(&format!("{}.h", basename));
        // Generate the resulting C
        let generated = ast.generate(&mut context);

        // Format the header file
        let header = context.generate_header_file();
        let header_contents = add_if_guards(&basename, &header);

        if args.display {
            print!("Header:\n{}\n\nSource:\n{}", &header_contents, &generated);
        } else {
            let source_filename = format!("{}.c", basename);
            let header_filename = format!("{}.h", basename);

            write_and_format_output_file(&source_filename, &generated)?;
            write_and_format_output_file(&header_filename, &header_contents)?;
        }
    } else {
        eprintln!("Failed to get the output filename.");
    }

    Ok(())
}

/// Display the tokens given by the lexer for the given input. Displays to stdout unless it is an
/// error token, at which point it will be displayed to stderr and the lexer will stop.
fn display_tokens(program_code: &str) {
    let lexer = lexer::Lexer::new(program_code.char_indices());

    for t in lexer {
        match t.map(|x| x.1) {
            Ok(c) => println!("Token: {:#?}", c),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

/// Get the output filename given the filename of the file we are currently processing. Gets the
/// stem of the file and its basename.
fn get_output_filename(filename: &OsStr) -> Option<String> {
    let path_struct = Path::new(&filename);
    let stem = path_struct.file_stem()?;
    let basename = stem.to_str()?;

    Some(basename.to_string())
}

/// Check whether `clang-format` is installed on this system. Searches the path variable for a
/// version of the program and returns true if it is found.
fn clang_format_exists() -> bool {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if Path::new(&path.join("clang-format")).exists() {
                return true;
            }
        }
    }

    false
}

/// Write the output code to a file and optionally format it using `clang-format`.
fn write_and_format_output_file(filename: &str, code: &str) -> Result<(), Box<dyn Error>> {
    fs::write(&filename, &code)?;

    // Format the code using clang-format if possible
    if clang_format_exists() {
        let mut command = Command::new("clang-format");
        command.arg("-i");
        command.arg(&filename);

        if Path::new(".clang-format").exists() {
            command.arg("--style=file");
        }

        command.spawn()?;
    }

    Ok(())
}

/// Parse program code specified by the argument and return the abstract syntax tree representation
/// if it passes, otherwise a string representation of the error that caused the issue.
///
/// # Errors
///
/// Errors can be caused on a malformed program input, at which point the parser will return an
/// error message which will then be formatted and propagated here.
pub fn parse(input: &str) -> Result<ast::Program, String> {
    let lex_input = input.char_indices();
    let lexer = lexer::Lexer::new(lex_input);
    let parser = parser::ProgramParser::new();
    let result = parser.parse(lexer);

    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(format_parser_error(e)),
    }
}

/// Format a parser error from the parser into a more human readable version.
fn format_parser_error(
    error: lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>,
) -> String {
    match error {
        lalrpop_util::ParseError::User { error } => format!("{}", error),
        lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
            let expected_tokens: Vec<String> =
                expected.iter().map(|s| s.replace("\"", "")).collect();
            let expected_tokens = format!("[{}]", expected_tokens.join(", "));

            format!(
                "Found: {:?} on line {}, Expected: {}",
                token.1, token.2, expected_tokens
            )
        }
        _ => String::from("Unexpected error occurred"),
    }
}

/// Get the abstract syntax tree for some program code, displaying it if specified by the `--ast`
/// flag on the command line.
fn get_abstract_syntax_tree(code: &str, display: bool) -> Result<ast::Program, Box<dyn Error>> {
    let ast = parse(&code)?;

    if display {
        println!("Abstract syntax tree:");
        println!("{:#?}", &ast);
    }

    Ok(ast)
}

/// Adds `#ifndef` guards to the header given the basename for the file.
///
/// Given the basename for a file, for example `test`, and a header file containing `void main();`,
/// this will wrap the header using `#ifndef` to ensure it can only be included once.
fn add_if_guards(basename: &str, header: &str) -> String {
    let uppercase = basename.to_uppercase();

    format!(
        "#ifndef {}_H\n#define {}_H\n\n{}\n\n#endif /* {}_H */\n",
        uppercase, uppercase, header, uppercase,
    )
}
