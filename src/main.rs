mod world;
pub use world::{ Coord, World };


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct U32Cell(u32);

impl Default for U32Cell {
    #[inline(always)]
    fn default() -> Self { Self(1) }
}


fn main() {
    let world = World::<U32Cell>::new();
}
