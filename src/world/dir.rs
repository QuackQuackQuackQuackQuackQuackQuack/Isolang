use core::ops::Neg;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Dir {
    L,
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
