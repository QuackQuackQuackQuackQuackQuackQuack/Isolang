use std::collections::BTreeMap;


mod coord;
pub use coord::Coord;

mod axis;
pub use axis::Axis;

mod adj;
pub use adj::Adj;

mod dir;
pub use dir::Dir;


pub mod cell;
pub use cell::Cell;


pub struct World<C : Cell> {
    head  : Coord,
    cells : BTreeMap<Coord, C>
}

impl<C : Cell> World<C> {

    pub fn new() -> Self { Self {
        head  : Coord::ZERO,
        cells : BTreeMap::new()
    } }

    pub fn head(&self) -> Coord {
        self.head
    }
    pub fn head_mut(&mut self) -> &mut Coord {
        &mut self.head
    }

    pub fn get(&self, coord : Coord) -> C {
        self.cells.get(&coord).cloned().unwrap_or(C::default())
    }
    pub fn insert(&mut self, coord : Coord, cell : C) {
        // TODO: Stdin/stdout
        if (cell == C::default()) {
            self.cells.remove(&coord);
        } else {
            self.cells.insert(coord, cell);
        }
    }

}
