use std::fs::File;


mod world;
pub use world::*;

mod runner;
pub use runner::*;


fn main() {
    let runner = ScriptRunner::<cell::U32Cell>::new(File::open("samples/simple.isolang").unwrap());
}
