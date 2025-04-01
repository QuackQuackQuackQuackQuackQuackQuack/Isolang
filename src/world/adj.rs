//! Pairs of adjacent cells in the [`World`].


/// Pairs of adjacent cells in the [`World`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Adj {

    /// Left/right
    /// ```
    ///  . .
    /// + @ +
    ///  . .
    /// ```
    LR,

    /// Up-left/down-right
    /// ```
    ///  + .
    /// . @ .
    ///  . +
    /// ```
    ULDR,

    /// Down-left/up-right
    /// ```
    ///  . +
    /// . @ .
    ///  + .
    /// ```
    DLUR,

    /// Up-left/up-right
    /// ```
    ///  + +
    /// . @ .
    ///  . .
    /// ```
    U2,

    /// Down-left/down-right
    /// ```
    ///  . .
    /// . @ .
    ///  + +
    /// ```
    D2

}

impl TryFrom<char> for Adj {
    type Error = ();
    fn try_from(ch : char) -> Result<Self, Self::Error> { Ok(match (ch) {
        '-'  => Self::LR,
        '\\' => Self::ULDR,
        '/'  => Self::DLUR,
        '^'  => Self::U2,
        'v'  => Self::D2,
        _    => { return Err(()); }
    }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_horiz() {
        assert_eq!(Adj::try_from('-'), Ok(Adj::LR));
        assert_eq!(Adj::try_from('\\'), Ok(Adj::ULDR));
        assert_eq!(Adj::try_from('^'), Ok(Adj::U2));
        assert_eq!(Adj::try_from('+'), Err(()));
    }
}