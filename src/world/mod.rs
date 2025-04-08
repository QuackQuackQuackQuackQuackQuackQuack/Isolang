//! Worlds contain the cell grid and world head.


use core::ops::{ Deref, DerefMut };
use core::hint;
use std::collections::BTreeMap;

mod coord;
pub use coord::Coord;

mod adj;
pub use adj::Adj;

mod dir;
pub use dir::Dir;


pub mod cell;
pub use cell::{ Cell, CellStdinReadError };


/// A container for the cell grid and world head.
pub struct World<C : Cell> {

    /// The current position of the world head.
    head        : Coord,

    /// The cells in the world.
    cells       : BTreeMap<Coord, C>,

    /// Standard in data.
    stdin       : C::StdinReader

}

impl<C : Cell> Default for World<C> {
    fn default() -> Self { Self {
        head        : Coord::ZERO,
        cells       : BTreeMap::new(),
        stdin       : C::create_stdin_reader()
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
    pub fn get(&mut self, coord : Coord) -> C {
        if (coord == Coord::ZERO) {
            if let Some(Dir::R) = coord.half_side() {
                loop {
                    if let Some(Ok(ch)) = self.stdin.next() {
                        return ch;
                    }
                    hint::spin_loop();
                }
            } else {
                C::ONE
            }
        } else {
            self.cells.get(&coord).cloned().unwrap_or(C::ONE)
        }
    }

    /// Get a mutable reference to a cell in the world by coordinate.
    pub fn get_mut(&mut self, coord : Coord) -> CellMut<C> {
        CellMut {
            cell  : self.get(coord),
            world : self,
            coord
        }
    }

    /// Overwrites a cell in the world.
    pub fn insert(&mut self, coord : Coord, cell : C) {
        if (coord == Coord::ZERO) {
            if let Some(Dir::L) = coord.half_side() {
                print!("{}", cell);
            }
        } else if (cell == C::ONE) {
            self.cells.remove(&coord);
        } else {
            self.cells.insert(coord, cell);
        }
    }

}


/// Mutable access to a cell in a [`World`].
///
/// *Note: Changes to the [`World`] are applied when this is dropped.*
pub struct CellMut<'l, C : Cell> {

    /// The current value of the cell, not yet written to the [`World`].
    cell  : C,

    /// The [`World`] to apply to.
    world : &'l mut World<C>,

    /// The coordinate of the cell in question.
    coord : Coord

}

impl<'l, C : Cell> Deref for CellMut<'l, C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}

impl<'l, C : Cell> DerefMut for CellMut<'l, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cell
    }
}

impl<'l, C : Cell> Drop for CellMut<'l, C> {
    fn drop(&mut self) {
        self.world.insert(self.coord, self.cell);
    }
}

// TODO test
