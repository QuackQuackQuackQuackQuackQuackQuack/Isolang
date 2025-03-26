use crate::world::Adj;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Axis {

    //  - -
    // # @ #
    //  - -
    LR,

    //  # -
    // - @ -
    //  - #
    ULDR,

    //  - #
    // - @ -
    //  # -
    DLUR

}

impl Axis {
    pub fn to_adj(self) -> Adj { self.into() }
}

impl TryFrom<char> for Axis {
    type Error = ();
    fn try_from(ch : char) -> Result<Self, Self::Error> { Ok(match (ch) {
        '-'  => Self::LR,
        '\\' => Self::ULDR,
        '/'  => Self::DLUR,
        _    => { return Err(()); }
    }) }
}

impl Into<Adj> for Axis {
    fn into(self) -> Adj { match (self) {
        Self::LR   => Adj::LR,
        Self::ULDR => Adj::ULDR,
        Self::DLUR => Adj::DLUR
    } }
}
