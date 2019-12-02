use std::fs;

use ast::Generate;

#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);

struct Args {
    filename: Option<String>,
}

fn parse(input: &str) -> Result<ast::program::Program, String> {
    match parser::ProgramParser::new().parse(lexer::Lexer::new(input.char_indices())) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        filename: args.opt_value_from_str("--filename")?,
    };

    let filename = match args.filename {
        Some(f) => f,
        None => panic!("Please enter a filename using the --filename argument."),
    };

    let program_code: String = fs::read_to_string(&filename).expect("Failed to read the file.");

    let ast = parse(&program_code).expect("Failed to parse the given program");
    dbg!(&ast);

    println!("Program: {}", ast.generate());

    Ok(())
}

#[cfg(test)]
mod tests;
