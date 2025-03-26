use crate::world::Cell;
use core::ops::Add;
use core::fmt;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct U8Cell(u8);

impl Default for U8Cell {
    #[inline(always)]
    fn default() -> Self { Self(1) }
}

impl Add for U8Cell {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Cell for U8Cell { }

impl fmt::Display for U8Cell {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}
