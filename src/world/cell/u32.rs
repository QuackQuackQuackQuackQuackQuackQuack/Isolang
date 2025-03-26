use crate::world::Cell;
use core::ops::{ Add, Sub, Mul, Div };
use core::fmt;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct U32Cell(u32);

impl Default for U32Cell {
    #[inline(always)]
    fn default() -> Self { Self(1) }
}

impl Add for U32Cell {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for U32Cell {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for U32Cell {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for U32Cell {
    type Output = Self;
    fn div(self, rhs : Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Cell for U32Cell {
    fn is_zero(&self) -> bool { self.0 == 0 }
}

impl fmt::Display for U32Cell {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (char::from_u32(self.0)) {
            Some(ch) => write!(f, "{}", ch),
            None     => Ok(())
        }
    }
}
