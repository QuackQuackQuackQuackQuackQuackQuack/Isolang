use crate::world::{ Adj, Dir };


/// Instruction modifier
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct InsMod {
    pub kind         : InsModKind,
    pub random_maybe : bool
}

/// Instruction modifier kind
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum InsModKind {
    Invert,
    IfNotZeroCond
}


/// Instruction
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

    IfNotZeroCond { ins : Box<Ins> },

    IfZeroCond { ins : Box<Ins> },

    RandomlyChoose { options : Box<(Ins, Ins)> }

}

impl Ins {

    /// Inverts the instruction.
    pub fn invert(self) -> Result<Self, ()> { match (self) {

        Self::MoveHead { adj, dir } => Ok(Self::MoveHead { adj, dir : -dir }),

        Self::Add { adj } => Ok(Self::Sub { adj }),

        Self::Sub { adj } => Ok(Self::Add { adj }),

        Self::Mul { adj } => Ok(Self::SDiv { adj }),

        Self::SDiv { adj } => Ok(Self::Mul { adj }),

        Self::IfZeroCond { ins } => Ok(Self::IfNotZeroCond { ins }),

        Self::IfNotZeroCond { ins } => Ok(Self::IfZeroCond { ins }),

        Self::RandomlyChoose { .. } => Err(())

    } }


    /// Applies an instruction modifier to this instruction.
    pub fn modify(self, modifier : InsMod) -> Result<Self, ()> {
        let ins = match (modifier.kind) {
            InsModKind::Invert        => self.clone().invert(),
            InsModKind::IfNotZeroCond => Ok(Self::IfNotZeroCond { ins : Box::new(self.clone()) })
        }?;
        Ok(if (modifier.random_maybe) {
            Ins::RandomlyChoose { options : Box::new((self, ins,)) }
        } else {
            ins
        })
    }

}
