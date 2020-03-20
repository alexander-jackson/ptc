//! The backend library of functions for the `ptc` compiler.

#![allow(unused_parens)]

#[macro_use]
extern crate lalrpop_util;
extern crate regex;

pub mod app;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser);
