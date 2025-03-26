use crate::world::Axis;


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

impl Adj {
    pub fn try_to_axis(self) -> Result<Axis, ()> { self.try_into() }
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

impl TryInto<Axis> for Adj {
    type Error = ();
    fn try_into(self) -> Result<Axis, Self::Error> { match (self) {
        Self::LR   => Ok(Axis::LR),
        Self::ULDR => Ok(Axis::ULDR),
        Self::DLUR => Ok(Axis::DLUR),
        Self::U2   => Err(()),
        Self::D2   => Err(())
    } }
}
