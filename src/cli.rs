//! Command line interface parsers and types.


use std::path::PathBuf;
pub use clap::Parser;
use clap::{Subcommand, ValueEnum };


#[allow(missing_docs)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd : CliCommand
}


#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum CliCommand {

    /// Run an Isolang script.
    Run {

        /// The type of cells in the world.
        #[clap(short = 'c', long, default_value = "u32")]
        cell_mode   : CellMode,

        /// The source script file to run.
        source_file : PathBuf

    }

}

/// The type of cells in the world.
#[derive(ValueEnum, Clone)]
pub enum CellMode {
    /// 8-bit unsigned integers.
    U8,
    /// 32-bit unsigned integers.
    U32
}
