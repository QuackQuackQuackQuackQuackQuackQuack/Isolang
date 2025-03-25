use core::fmt;
use std::collections::BTreeMap;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coord {
    r  : isize,
    ul : isize
}

impl Coord {

    pub const ZERO : Self = Self { r : 0, ul : 0 };

    pub fn adjacent(&self, axis : char, sign : char) -> Result<Self, ()> {
        return Ok(match (axis,sign) {
            ('-', '<') => Self { r : self.r - 1, ul : self.ul     },
            ('-', '>') => Self { r : self.r + 1, ul : self.ul     },
            ('\\','<') => Self { r : self.r,     ul : self.ul + 1 },
            ('\\','>') => Self { r : self.r,     ul : self.ul - 1 },
            ('/', '<') => Self { r : self.r - 1, ul : self.ul - 1 },
            ('/', '>') => Self { r : self.r + 1, ul : self.ul + 1 },
            _ => return Err(())
        });
    }

}

impl fmt::Debug for Coord {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(→{} ↖{})", self.r, self.ul)
    }
}


pub struct World<Cell> {
    head  : Coord,
    cells : BTreeMap<Coord, Cell>
}

impl<Cell : PartialEq + Clone + Copy + Default> World<Cell> {

    pub fn new() -> Self { Self {
        head  : Coord::ZERO,
        cells : BTreeMap::new()
    } }

    pub fn head(&self) -> Coord {
        self.head
    }

    pub fn get(&self, coord : Coord) -> Cell {
        self.cells.get(&coord).cloned().unwrap_or(Cell::default())
    }

    pub fn insert(&mut self, coord : Coord, cell : Cell) {
        if (cell == Cell::default()) {
            self.cells.remove(&coord);
        } else {
            self.cells.insert(coord, cell);
        }
    }

}
