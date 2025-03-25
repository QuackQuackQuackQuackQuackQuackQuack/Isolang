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
            ('-','<') -> Self { r : self.r - 1, ul : self.ul },
	    ('-','>') -> Self { r : self.r + 1, ul : self.ul },
	    ('\','<') -> Self { r : self.r, ul : self.ul + 1 },
	    ('\','>') -> Self { r : self.r, ul : self.ul - 1 },
	    ('/','<') -> Self { r : self.r - 1, ul : self.ul - 1 },
 	    ('/','>') -> Self { r : self.r + 1, ul : self.ul + 1 },
	    _ -> return Err(())
        });
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(→{} ↖{})", self.r, self.ul)
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct Cell(u32);

impl Default for Cell {
    #[inline(always)]
    fn default() -> Self { Self(1) }
}


pub struct World {
    head  : Coord,
    cells : BTreeMap<Coord, Cell>
}

impl Default for World {
    fn default() -> Self { Self {
        head  : Coord::ZERO,
        cells : BTreeMap::new()
    } }
}

impl World {

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


fn main() {
    println!("Hello, world!");
}
