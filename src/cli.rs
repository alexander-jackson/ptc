use std::fs;
use std::error::Error;

use crate::ast;
use crate::parser;
use crate::lexer;

use crate::ast::Generate;

pub struct Args {
    /// File input using -f <filename> or --filename <filename>
    filename: Option<String>,
    /// Whether the AST should be displayed, specified by --ast
    abstract_tree: bool,
}

pub fn get_arguments() -> Result<Args, Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        filename: args.opt_value_from_str(["-f", "--filename"])?,
        abstract_tree: args.contains("--ast"),
    };

    Ok(args)
}

pub fn process_args(args: Args) -> Result<(), Box<dyn Error>> {
    let filename = args.filename
        .expect("Please supply a filename with [-f/--filename]");

    let program_code: String = fs::read_to_string(&filename).expect("Failed to read the file.");

    let ast = parse(&program_code).expect("Failed to parse the given program");

    if args.abstract_tree {
        dbg!(&ast);
    }

    println!("{}", ast.generate());

    Ok(())
}

fn parse(input: &str) -> Result<ast::program::Program, String> {
    match parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices())) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("{:?}", e)),
    }
}
