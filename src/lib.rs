#![feature(box_syntax, box_patterns)]

#[macro_use] extern crate serde_derive;
extern crate clap;

extern crate prettytable;
extern crate reqwest;

pub mod command;
pub mod util;
