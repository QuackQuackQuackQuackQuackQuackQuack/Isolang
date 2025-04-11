//! # Isolang
//! Parser & Interpreter
//!
//! TODO: Crate docs

#![feature(assert_matches)]

use std::fs::File;
use std::io::{ Read, BufReader };


mod cli;
pub use cli::*;

mod parser;
pub use parser::*;

mod world;
pub use world::*;

mod runner;
pub use runner::*;


fn main() -> Result<(), ParseError> {
    match (Cli::parse().cmd) {

        CliCommand::Run {
            cell_mode,
            #[cfg(debug_assertions)]
            world_margin,
            source_file
        } => {
            let file   = File::open(source_file)?;
            let bytes  = BufReader::new(file).bytes();
            let script = ScriptParser::parse(bytes)?;
            // println!("{:#?}", script);
            match (cell_mode) {
                CellMode::U8 => {
                    let mut runner = ScriptRunner::<cell::U8Cell>::new(script);
                    #[cfg(debug_assertions)]
                    runner.world_mut().set_display_margin(world_margin);
                    while (runner.run_next()) { }
                },
                CellMode::U32 => {
                    let mut runner = ScriptRunner::<cell::U32Cell>::new(script);
                    #[cfg(debug_assertions)]
                    runner.world_mut().set_display_margin(world_margin);
                    while (runner.run_next()) { }
                }
            }
            println!();
            Ok(())
        }

    }
}
