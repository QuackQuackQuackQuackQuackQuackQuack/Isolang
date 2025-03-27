//! # Isolang
//! Parser & Interpreter
//! 
//! TODO: Crate docs


use std::fs::File;
use std::io::{ Read, BufReader };


mod parser;
pub use parser::*;

mod world;
pub use world::*;

mod runner;
pub use runner::*;


fn main() {
    // Open the file.
    let     file   = File::open("samples/simple.isolang").unwrap();
    // Get the bytes of the file.
    let     bytes  = BufReader::new(file).bytes();
    // Parse the bytes.
    let     script = ScriptParser::parse(bytes).unwrap();
    // Construct a `ScriptRunner` for the parsed script.
    let mut runner = ScriptRunner::<cell::U32Cell>::new(script);
    // Run each instruction until the end of the program is reached.
    while (runner.run_next()) { }
}
