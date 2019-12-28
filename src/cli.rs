use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::ast;
use crate::lexer;
use crate::parser;

use crate::ast::Generate;

pub struct Args {
    /// File input using -f <filename> or --filename <filename>
    filename: Result<String, pico_args::Error>,
    /// Whether the AST should be displayed, specified by --ast
    abstract_tree: bool,
    /// Whether we should display tokens, specified by --tokens
    tokens: bool,
    /// Whether the help message should be displayed
    help: bool,
}

fn display_help_message() {
    println!(
        r#"
ptc (Python to C Compiler)
Transpiles code from Python to C

USAGE:
    ptc [FLAGS] [OPTIONS]

FLAGS:
    --ast               Displays the abstract syntax tree after parsing
    --tokens            Displays the tokens output by the lexer for the given input
    -h, --help          Prints this help information

OPTIONS:
    -f, --filename <path>           Prints age debug info for the given username
"#
    );
}

pub fn get_arguments() -> Args {
    let mut args = pico_args::Arguments::from_env();

    Args {
        filename: args.value_from_str(["-f", "--filename"]),
        abstract_tree: args.contains("--ast"),
        tokens: args.contains("--tokens"),
        help: args.contains(["-h", "--help"]),
    }
}

pub fn process_args(args: Args) -> Result<(), Box<dyn Error>> {
    if args.help {
        display_help_message();
        return Ok(());
    }

    let filename = args.filename?;
    let code: String = fs::read_to_string(&filename)?;

    if args.tokens {
        display_tokens(&code)?;
    }

    let ast = get_abstract_syntax_tree(&code, args.abstract_tree);
    let generated = ast.generate();
    let output = get_output_filename(&filename);

    write_generated_output(&output, &generated)?;

    Ok(())
}

fn display_tokens(program_code: &str) -> Result<(), Box<dyn Error>> {
    let lexer = lexer::Lexer::new(program_code.char_indices());

    for t in lexer {
        let curr = t.unwrap().1;
        println!("Token: {:#?}", curr);
    }

    Ok(())
}

fn get_output_filename(filename: &str) -> Option<String> {
    let path_struct = Path::new(&filename);
    let stem = path_struct.file_stem().unwrap();
    let basename = stem.to_str().unwrap();

    let output = format!("{}.c", basename);

    if !Path::new(&output).exists() {
        Some(output)
    } else {
        None
    }
}

fn write_generated_output(output: &Option<String>, generated: &str) -> Result<(), Box<dyn Error>> {
    if output.is_some() {
        write_and_format_output_file(output.as_ref().unwrap(), &generated)?;
    } else {
        eprintln!("The output file already exists, so the code will be displayed to the screen: ");
        println!("{}", &generated);
    }

    Ok(())
}

fn check_clang_format_exists() -> bool {
    Command::new("clang-format")
        .arg("--version")
        .spawn()
        .is_ok()
}

fn write_and_format_output_file(filename: &str, code: &str) -> Result<(), Box<dyn Error>> {
    fs::write(&filename, &code)?;

    if check_clang_format_exists() {
        Command::new("clang-format")
            .arg("-i")
            .arg(&filename)
            .spawn()?;
    } else {
        println!("clang-format does not exist");
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
