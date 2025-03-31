//! A position in a [`World`].


use crate::world::{ Adj, Dir };
use core::fmt;
use core::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut };


/// A position in a [`World`].
/// 
/// Positive direction is right/up-left.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coord {
    /// Number of cells to the right direction.
    r  : isize,
    /// Number of cells to the up-left direction/
    ul : isize
}

impl Coord {
    /// [`World`] origin.
    pub const ZERO : Self = Self { r : 0, ul : 0 };
}

impl fmt::Debug for Coord {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(r:{},ul:{})", self.r, self.ul)
    }
}

impl Add<Self> for Coord {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self { r : self.r + rhs.r, ul : self.ul + rhs.ul }
    }
}
impl AddAssign<Self> for Coord {
    fn add_assign(&mut self, rhs : Self) {
        *self = *self + rhs;
    }
}

impl Sub<Self> for Coord {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self { r : self.r - rhs.r, ul : self.ul - rhs.ul }
    }
}
impl SubAssign<Self> for Coord {
    fn sub_assign(&mut self, rhs : Self) {
        *self = *self - rhs;
    }
}

impl Mul<isize> for Coord {
    type Output = Self;
    fn mul(self, rhs : isize) -> Self::Output {
        Self { r : self.r * rhs, ul : self.ul * rhs }
    }
}
impl MulAssign<isize> for Coord {
    fn mul_assign(&mut self, rhs : isize) {
        *self = *self * rhs;
    }
}

impl Neg for Coord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { r : -self.r, ul : -self.ul }
    }
}

impl Add<Adj> for Coord {
    type Output = (Self, Self,);
    fn add(self, adj : Adj) -> Self::Output { match (adj) {
        Adj::LR   => (self + (Adj::LR,   Dir::L,), self + (Adj::LR,   Dir::R,)),
        Adj::ULDR => (self + (Adj::ULDR, Dir::L,), self + (Adj::ULDR, Dir::R,)),
        Adj::DLUR => (self + (Adj::DLUR, Dir::L,), self + (Adj::DLUR, Dir::R,)),
        Adj::U2   => (self + (Adj::ULDR, Dir::L,), self + (Adj::DLUR, Dir::R)),
        Adj::D2   => (self + (Adj::DLUR, Dir::L,), self + (Adj::ULDR, Dir::R)),
    } }
}
impl Add<(Adj, Dir,)> for Coord {
    type Output = Self;
    fn add(self, (adj, dir) : (Adj, Dir,)) -> Self::Output {
        (self + adj)[dir]
    }
}
impl AddAssign<(Adj, Dir,)> for Coord {
    fn add_assign(&mut self, rhs : (Adj, Dir,)) {
        *self = *self + rhs;
    }
}

impl Index<Dir> for (Coord, Coord,) {
    type Output = Coord;
    fn index(&self, dir : Dir) -> &Self::Output { match (dir) {
        Dir::L => &self.0,
        Dir::R => &self.1
    } }
}
impl IndexMut<Dir> for (Coord, Coord,) {
    fn index_mut(&mut self, dir : Dir) -> &mut Self::Output { match (dir) {
        Dir::L => &mut self.0,
        Dir::R => &mut self.1
    } }
}

impl From<(Adj, Dir,)> for Coord {
    fn from(value : (Adj, Dir,)) -> Self {
        Coord::ZERO + value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub_coords() {
        let a = Coord { r : 12, ul : 16 };
        let b = Coord { r : 3,  ul : 31 };
        assert_eq!(a + b, Coord { r : 15, ul :  47 });
        assert_eq!(a - b, Coord { r : 9,  ul : -15 });
    }

    #[test]
    fn mul_coord_scalar() {
        assert_eq!(Coord { r : 12, ul : 16 } *  3, Coord { r :  36, ul :   48 });
        assert_eq!(Coord { r : 3,  ul : 31 } * -4, Coord { r : -12, ul : -124 });
    }

    #[test]
    fn neg_coord() {
        assert_eq!(-Coord { r : 12, ul : 16 }, Coord { r : -12, ul : -16 });
        assert_eq!(-Coord { r : 3,  ul : 31 }, Coord { r :  -3, ul : -31 });
    }

}
