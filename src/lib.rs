//! The backend library of functions for the `ptc` compiler.

#![allow(unused_parens)]

#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;

pub mod app;

pub mod ast;
pub mod lexer;

lalrpop_mod!(#[allow(clippy::all, clippy::pedantic)] pub parser);
