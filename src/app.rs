use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::ast;
use crate::lexer;
use crate::parser;

use crate::ast::Context;
use crate::ast::Generate;
use crate::ast::Infer;

pub struct Args {
    /// Whether the AST should be displayed, specified by --ast
    abstract_tree: bool,
    /// Whether we should display tokens, specified by --tokens
    tokens: bool,
    /// Whether we should display generated code, specified by --display
    display: bool,
    /// Whether the help message should be displayed
    help: bool,
    /// All paths that the compiler should be run on
    paths: Vec<String>,
}

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

pub fn get_arguments() -> Result<Args, Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    Ok(Args {
        abstract_tree: args.contains("--ast"),
        tokens: args.contains("--tokens"),
        display: args.contains("--display"),
        help: args.contains(["-h", "--help"]),
        paths: args.free()?,
    })
}

pub fn process_args(args: Args) -> Result<(), Box<dyn Error>> {
    if args.help || args.paths.is_empty() {
        display_help_message();
        return Ok(());
    }

    for path in &args.paths {
        process_path(&path, &args)?;
    }

    Ok(())
}

fn process_path(path: &str, args: &Args) -> Result<(), Box<dyn Error>> {
    let code: String = fs::read_to_string(&path)?;

    if args.tokens {
        display_tokens(&code);
    }

    let mut ast = get_abstract_syntax_tree(&code, args.abstract_tree);
    let mut context = Context::new();
    ast.infer(&mut context);
    let generated = ast.generate(&mut context);
    let output = get_output_filename(&path);

    if args.display {
        println!("{}", &generated);
    } else {
        match output {
            Some(s) => write_and_format_output_file(&s, &generated)?,
            None => eprintln!("Failed to get the output filename."),
        }
    }

    Ok(())
}

fn display_tokens(program_code: &str) {
    let lexer = lexer::Lexer::new(program_code.char_indices());

    for t in lexer {
        match t.map(|x| x.1) {
            Ok(c) => println!("Token: {:#?}", c),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

fn get_output_filename(filename: &str) -> Option<String> {
    let path_struct = Path::new(&filename);
    let stem = path_struct.file_stem()?;
    let basename = stem.to_str()?;

    Some(format!("{}.c", basename))
}

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

fn write_and_format_output_file(filename: &str, code: &str) -> Result<(), Box<dyn Error>> {
    fs::write(&filename, &code)?;

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

fn parse(input: &str) -> Result<ast::program::Program, String> {
    let lex_input = input.char_indices();
    let lexer = lexer::Lexer::new(lex_input);
    let parser = parser::ProgramParser::new();
    let result = parser.parse(lexer);

    match result {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn get_abstract_syntax_tree(code: &str, display: bool) -> ast::program::Program {
    let ast = parse(&code).expect("Failed to parse the given program");

    if display {
        println!("Abstract syntax tree:");
        println!("{:#?}", &ast);
    }

    ast
}
