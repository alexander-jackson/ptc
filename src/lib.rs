#[macro_use]
extern crate lalrpop_util;

mod cli;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);

pub fn lib_main() {
    let result = cli::get_arguments().and_then(cli::process_args);

    match result {
        Ok(_) => (),
        Err(e) => eprintln!("Error occurred: {}", e),
    };
}
