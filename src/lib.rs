#[macro_use]
extern crate lalrpop_util;

mod cli;
mod error_handling;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);

pub fn lib_main() {
    let result = cli::get_arguments().and_then(cli::process_args);
    error_handling::handle_errors(result);
}
