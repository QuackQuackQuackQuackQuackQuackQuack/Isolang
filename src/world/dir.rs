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

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn neg_dir () {
        assert_eq!(-Dir::L, Dir::R);
        assert_eq!(-Dir::R, Dir::L);
    }

    #[test]
    fn try_from_char_dir () {
        assert_eq!(Dir::try_from('0'), Err(()));
        assert_eq!(Dir::try_from('<'), Ok(Dir::L));
        assert_eq!(Dir::try_from('>'), Ok(Dir::R));
    }
}