//! A position in a [`World`].


use crate::world::{ Adj, Dir };
use core::fmt;
use core::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut };
use std::cmp::Ordering;


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
    /// One unit left.
    pub const L : Self = Self { r : -1, ul : 0 };
    /// One unit right.
    pub const R : Self = Self { r : 1, ul : 0 };
    /// One unit up-left.
    pub const UL : Self = Self { r : 0, ul : 1 };
    /// One unit down-left.
    pub const DL : Self = Self { r : -1, ul : -1 };
    /// One unit up-right.
    pub const UR : Self = Self { r : 1, ul : 1 };
    /// One unit down-right.
    pub const DR : Self = Self { r : 0, ul : -1 };

    /// The position in the x-direction of the Coord relative to ZERO
    pub fn relative_pos (&self) -> Ordering {
        let x_pos_times_2 = self.r * 2 - self.ul;
        match x_pos_times_2 {
            ..=-1 => Ordering::Less,
            0 => Ordering::Equal,
            1.. => Ordering::Greater
        }
    }
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
        Adj::LR   => (self + Self::L,  self + Self::R  ),
        Adj::ULDR => (self + Self::UL, self + Self::DR ),
        Adj::DLUR => (self + Self::DL, self + Self::UR ),
        Adj::U2   => (self + Self::UL, self + Self::UR ),
        Adj::D2   => (self + Self::DL, self + Self::DR ),
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

    const A: Coord = Coord { r : 12, ul : 16 };
    const B: Coord = Coord { r :  3, ul : 31 };

    #[test]
    fn add_sub_coords() {
        assert_eq!(A + B, Coord { r : 15, ul :  47 });
        assert_eq!(A - B, Coord { r :  9, ul : -15 });
    }

    #[test]
    fn mul_coord_scalar() {
        assert_eq!(Coord { r : 12, ul : 16 } *  3, Coord { r :  36, ul :   48 });
        assert_eq!(Coord { r :  3, ul : 31 } * -4, Coord { r : -12, ul : -124 });
    }

    #[test]
    fn neg_coord() {
        assert_eq!(-Coord { r : 12, ul : 16 }, Coord { r : -12, ul : -16 });
        assert_eq!(-Coord { r :  3, ul : 31 }, Coord { r :  -3, ul : -31 });
    }

    #[test]
    fn add_coord_adj() {
        assert_eq!(Coord { r : 12, ul : 16 } + Adj::LR,   (Coord { r : 11, ul : 16 }, Coord { r : 13, ul : 16 },));
        assert_eq!(Coord { r :  3, ul : 31 } + Adj::LR,   (Coord { r :  2, ul : 31 }, Coord { r :  4, ul : 31 },));
        assert_eq!(Coord { r : 12, ul : 16 } + Adj::ULDR, (Coord { r : 12, ul : 17 }, Coord { r : 12, ul : 15 },));
        assert_eq!(Coord { r :  3, ul : 31 } + Adj::ULDR, (Coord { r :  3, ul : 32 }, Coord { r :  3, ul : 30 },));
        assert_eq!(Coord { r : 12, ul : 16 } + Adj::DLUR, (Coord { r : 11, ul : 15 }, Coord { r : 13, ul : 17 },));
        assert_eq!(Coord { r :  3, ul : 31 } + Adj::DLUR, (Coord { r :  2, ul : 30 }, Coord { r :  4, ul : 32 },));
        assert_eq!(Coord { r : 12, ul : 16 } + Adj::U2,   (Coord { r : 12, ul : 17 }, Coord { r : 13, ul : 17 },));
        assert_eq!(Coord { r :  3, ul : 31 } + Adj::U2,   (Coord { r :  3, ul : 32 }, Coord { r :  4, ul : 32 },));
        assert_eq!(Coord { r : 12, ul : 16 } + Adj::D2,   (Coord { r : 11, ul : 15 }, Coord { r : 12, ul : 15 },));
        assert_eq!(Coord { r :  3, ul : 31 } + Adj::D2,   (Coord { r :  2, ul : 30 }, Coord { r :  3, ul : 30 },));
    }

    #[test]
    fn add_coord_adj_dir() {
        // assert_eq!(Coord { r : })
        // TODO
    }
    
    #[test]
    fn coord_coord_index_dir() {
        // TODO
    }

}
