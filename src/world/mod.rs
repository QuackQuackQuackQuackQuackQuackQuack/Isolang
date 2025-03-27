//! Worlds contain the cell grid and world head.


use std::collections::BTreeMap;


mod coord;
pub use coord::Coord;

mod adj;
pub use adj::Adj;

mod dir;
pub use dir::Dir;


pub mod cell;
pub use cell::Cell;


/// A container for the cell grid and world head.
pub struct World<C : Cell> {

    /// The current position of the world head.
    head  : Coord,

    /// The cells in the world.
    cells : BTreeMap<Coord, C>

}

impl<C : Cell> Default for World<C> {
    fn default() -> Self { Self {
        head  : Coord::ZERO,
        cells : BTreeMap::new()
    } }
}

impl<C : Cell> World<C> {

    /// Get the current world head.
    pub fn head(&self) -> Coord {
        self.head
    }
    /// Get a mutable reference to the world head.
    pub fn head_mut(&mut self) -> &mut Coord {
        &mut self.head
    }

    /// Get a cell in the world by coordinate.
    /// 
    /// *Note: If the cell does not exist, [`C::default()*](Default::default) is returned.
    pub fn get(&self, coord : Coord) -> C {
        self.cells.get(&coord).cloned().unwrap_or(C::default())
    }
    /// Overwrites a cell in the world.
    pub fn insert(&mut self, coord : Coord, cell : C) {
        // TODO: Stdin/stdout
        if (cell == C::default()) {
            self.cells.remove(&coord);
        } else {
            self.cells.insert(coord, cell);
        }
    }

}
