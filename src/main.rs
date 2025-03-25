mod world;
pub use world::*;

mod runner;
pub use runner::*;

use std::fs::File;
use std::io::Read;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct U32Cell(u32);

impl Default for U32Cell {
    #[inline(always)]
    fn default() -> Self { Self(1) }
}


fn main() {
    let script = File::open("samples/simple.isolang").unwrap();
    for ch in script.bytes() {
        let ch = ch.unwrap() as char;
    }
    let world = World::<U32Cell>::new();
}
