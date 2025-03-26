use crate::world::{ Axis, Adj, Dir };
use core::fmt;
use core::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut };


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coord {
    r  : isize,
    ul : isize
}

impl Coord {
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

impl Add<Axis> for Coord {
    type Output = (Self, Self,);
    fn add(self, axis : Axis) -> Self::Output {
        self + axis.to_adj()
    }
}
impl Add<(Axis, Dir,)> for Coord {
    type Output = Self;
    fn add(self, (axis, dir,) : (Axis, Dir,)) -> Self::Output {
        (self + axis)[dir]
    }
}
impl AddAssign<(Axis, Dir,)> for Coord {
    fn add_assign(&mut self, rhs : (Axis, Dir,)) {
        *self = *self + rhs;
    }
}

impl Add<Adj> for Coord {
    type Output = (Self, Self,);
    fn add(self, adj : Adj) -> Self::Output { match (adj) {
        Adj::LR   => (self + (Axis::LR,   Dir::L,), self + (Axis::LR,   Dir::R,)),
        Adj::ULDR => (self + (Axis::ULDR, Dir::L,), self + (Axis::ULDR, Dir::R,)),
        Adj::DLUR => (self + (Axis::DLUR, Dir::L,), self + (Axis::DLUR, Dir::R,)),
        Adj::U2   => (self + (Axis::ULDR, Dir::L,), self + (Axis::DLUR, Dir::R)),
        Adj::D2   => (self + (Axis::DLUR, Dir::L,), self + (Axis::ULDR, Dir::R)),
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
