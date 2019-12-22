use std::error::Error;
use std::fs;
use std::path::Path;

use crate::ast;
use crate::lexer;
use crate::parser;

use crate::ast::Generate;

pub struct Args {
    /// File input using -f <filename> or --filename <filename>
    filename: Option<String>,
    /// Whether the AST should be displayed, specified by --ast
    abstract_tree: bool,
    /// Whether we should display tokens, specified by --tokens
    tokens: bool,
}

pub fn get_arguments() -> Result<Args, Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        filename: args.opt_value_from_str(["-f", "--filename"])?,
        abstract_tree: args.contains("--ast"),
        tokens: args.contains("--tokens"),
    };

    Ok(args)
}

pub fn process_args(args: Args) -> Result<(), Box<dyn Error>> {
    let filename = args
        .filename
        .expect("Please supply a filename with [-f/--filename]");

    let output_filename = get_output_filename(&filename)?;

    let program_code: String = fs::read_to_string(&filename).expect("Failed to read the file.");

    if args.tokens {
        return display_tokens(&program_code);
    }

    let ast = parse(&program_code).expect("Failed to parse the given program");

    if args.abstract_tree {
        dbg!(&ast);
    }

    let generated_code = ast.generate();
    // This is safe as we have already checked whether the file exists
    fs::write(&output_filename, &generated_code).unwrap();

    println!("{}", &generated_code);

    Ok(())
}

fn parse(input: &str) -> Result<ast::program::Program, String> {
    match parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices())) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn get_output_filename(filename: &str) -> Result<String, String> {
    let path_struct = Path::new(&filename);
    let stem = path_struct.file_stem().unwrap();
    let basename = stem.to_str().unwrap();

    let output_filename = format!("{}.c", basename);

    if Path::new(&output_filename).exists() {
        // Path already exists so we cannot use this one
        return Err(format!("File {} already exists.", output_filename));
    }

    Ok(output_filename)
}

fn display_tokens(program_code: &str) -> Result<(), Box<dyn Error>> {
    let mut lexer = lexer::Lexer::new(program_code.char_indices());

    while let Some(t) = lexer.next() {
        let curr = t.unwrap().1;
        println!("Token: {:#?}", curr);
    }

    Ok(())
}
