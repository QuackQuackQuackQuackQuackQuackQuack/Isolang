//! A sideways direction in the [`World`].


use core::ops::Neg;


/// A sideways direction in the [`World`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Dir {
    /// Left
    L,
    /// Right
    R
}

impl TryFrom<char> for Dir {
    type Error = ();
    fn try_from(ch : char) -> Result<Self, Self::Error> { Ok(match (ch) {
        '<' => Self::L,
        '>' => Self::R,
        _   => { return Err(()); }
    }) }
}

impl Neg for Dir {
    type Output = Self;
    fn neg(self) -> Self::Output { match (self) {
        Self::L => Self::R,
        Self::R => Self::L
    } }
}
