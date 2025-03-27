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

    /// Inverts an instruction.
    Invert,

    /// Makes the instruction only run if the cell at the world head is not zero.
    IfNotZeroCond

}


/// Instruction
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Ins {

    /// Moves the head left or right a single cell on an adj.
    MoveHeadOne {
        adj : Adj,
        dir : Dir
    },

    /// Moves the head left or right some number of cells on an adj.
    /// The distance depends on the value in the cell at the world head.
    MoveHeadDynamic { 
        adj : Adj,
        dir : Dir
    },

    /// Adds the two cells targeted by the adj and stores it at the world head.
    Add { adj : Adj },

    /// Subtracts the two cells targeted by the adj (left minus right) and stores it at the world head.
    Sub { adj : Adj },

    /// Multiplies the two cells targeted by the adj and stores it at the world head.
    Mul { adj : Adj },

    /// Signed integer divides the two cells targeted by the adj (left divided by right) and stores it at the world head.
    SDiv { adj : Adj },

    /// Swaps the two cells targeted by the adj.
    Swap { adj : Adj },

    /// Does nothing, but counts towards the instruction length.
    Noop,

    /// Runs the contained instruction if the cell at the world head is not zero.
    IfNotZeroCond { ins : Box<Ins> },

    /// Runs the contained instruction if the cell at the world head is zero.
    IfZeroCond { ins : Box<Ins> },

    /// Runs one of two contained instruction at random.
    RandomlyChoose { options : Box<(Ins, Ins)> }

}

impl Ins {

    /// Inverts the instruction.
    pub fn invert(self) -> Result<Self, ()> { match (self) {

        Self::MoveHeadOne { adj, dir } => Ok(Self::MoveHeadOne { adj, dir : -dir }),

        Self::MoveHeadDynamic { adj, dir } => Ok(Self::MoveHeadDynamic { adj, dir : -dir }),

        Self::Add { adj } => Ok(Self::Sub { adj }),

        Self::Sub { adj } => Ok(Self::Add { adj }),

        Self::Mul { adj } => Ok(Self::SDiv { adj }),

        Self::SDiv { adj } => Ok(Self::Mul { adj }),

        Self::Swap { .. } => Err(()),

        Self::Noop => Err(()),

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
