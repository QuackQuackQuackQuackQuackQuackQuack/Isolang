#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Adj {

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
    DLUR,

    //  # #
    // - @ -
    //  - -
    U2,

    //  - -
    // - @ -
    //  # #
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
