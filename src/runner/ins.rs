use crate::world::{ Adj, Dir };


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum InsMod {
    Invert,
    IfNotZeroCond
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Ins {

    MoveHead {
        adj : Adj,
        dir : Dir
    },

    Add { adj : Adj },

    Sub { adj : Adj },

    Mul { adj : Adj },

    SDiv { adj : Adj },

    IfZeroCond { ins : Box<Ins> },

    IfNotZeroCond { ins : Box<Ins> }

}

impl Ins {

    pub fn invert(self) -> Self { match (self) {

        Self::MoveHead { adj, dir } => Self::MoveHead { adj, dir : -dir },

        Self::Add { adj } => Self::Sub { adj },

        Self::Sub { adj } => Self::Add { adj },

        Self::Mul { adj } => Self::SDiv { adj },

        Self::SDiv { adj } => Self::Mul { adj },

        Self::IfZeroCond { ins } => Self::IfNotZeroCond { ins },

        Self::IfNotZeroCond { ins } => Self::IfZeroCond { ins }

    } }

    pub fn modifier(self, modifier : InsMod) -> Self { match (modifier) {
        InsMod::Invert        => self.invert(),
        InsMod::IfNotZeroCond => Self::IfNotZeroCond { ins : Box::new(self) }
    } }

}
