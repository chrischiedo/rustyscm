use std::io;

pub mod env;
pub mod eval;
mod lexer;
mod op_utils;
pub mod parser;

use crate::env::standard_env;
use crate::eval::eval;

use anyhow::{Context, Result};

pub fn repl() {
    let mut global_env = standard_env();

    loop {
        println!("schemer>");

        let expr = read_input().unwrap();

        match eval(expr.as_ref(), &mut global_env) {
            Ok(val) => println!(" ==> {}", val),
            Err(error) => eprintln!("==> Error: {}", error),
        };
    }
}

fn read_input() -> Result<String> {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .context("Failed to read line")?;

    Ok(raw_input)
}
