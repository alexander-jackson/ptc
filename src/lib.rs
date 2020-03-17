#![allow(unused_parens)]
#![warn(missing_docs)]

#[macro_use]
extern crate lalrpop_util;
extern crate regex;

pub mod app;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);

pub fn lib_main() {
    let result = app::get_arguments().and_then(app::process_args);

    match result {
        Ok(_) => (),
        Err(e) => eprintln!("Error occurred: {}", e),
    };
}
