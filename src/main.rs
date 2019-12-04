#[macro_use]
extern crate lalrpop_util;

mod cli;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::get_arguments()?;

    cli::process_args(args)
}

#[cfg(test)]
mod tests;
