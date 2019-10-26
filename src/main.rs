use std::fs;
use std::env;

#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);

fn compile(input: &str) -> Result<ast::Program, String> {
    match parser::ProgramParser::new().parse(lexer::Lexer::new(input)) {
        Ok(s) => Ok(ast::Program::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn main() {
    let mut source = String::new();
    match env::args().nth(1) {
        Some(filename) => {
            source = fs::read_to_string(&filename)
                .expect("Failed to open the file and read it.");
        }
        None => { (); }
    }

    if source.is_empty() {
        println!("Empty file");
        return;
    }

    let ast = compile(&source).expect("OH NO");
    dbg!(&ast);
}
