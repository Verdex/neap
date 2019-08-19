
mod data;
mod input;
mod lexer;
mod parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("need file name");
    }
    let mut file = File::open(&args[1])?;
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;
    println!("Hello, world!");
    println!("{:#?}", args);
    println!("{}", input_string);
    lexer::lex(&input_string);
    Ok(())
}
