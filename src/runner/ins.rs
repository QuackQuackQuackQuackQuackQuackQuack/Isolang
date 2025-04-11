//! The Isolang instruction set.


use crate::world::{ Adj, Dir };


/// A modifier for an instruction.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct InsMod {

    /// The kind of instruction modifier.
    pub kind         : InsModKind,

    /// If `true`, randomly choose whether this modifier is applied or not.
    pub random_maybe : bool

}

/// The kind of modifier for an instruction.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum InsModKind {

    /// Inverts an instruction.
    Invert,

    /// Makes the instruction only run if the cell at the world head is not zero.
    IfNotZeroCond,

    /// Skips the instruction, ie it is not run
    Skip
}


/// A runnable instruction and its arguments.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Ins {

    /// Moves the head left or right a single cell on an adj.
    MoveHeadOne {
        /// The adj to move along.
        adj : Adj,
        /// Whether to move left or right.
        dir : Dir
    },

    /// Moves the head left or right some number of cells on an adj.
    /// The distance depends on the value in the cell at the world head.
    MoveHeadDynamic { 
        /// The adj to move along.
        adj : Adj,
        /// Whether to move left or right.
        dir : Dir
    },

    /// Adds the two cells targeted by the adj and stores it at the world head.
    Add {
        /// The adj on which the two cells fall.
        adj : Adj
    },

    /// Subtracts the two cells targeted by the adj (left minus right) and stores it at the world head.
    Sub {
        /// The adj on which the two cells fall.
        adj : Adj
    },

    /// Multiplies the two cells targeted by the adj and stores it at the world head.
    Mul {
        /// The adj on which the two cells fall.
        adj : Adj
    },

    /// Signed integer divides the two cells targeted by the adj (left divided by right) and stores it at the world head.
    SDiv {
        /// The adj on which the two cells fall.
        adj : Adj
    },

    /// Swaps the two cells targeted by the adj.
    Swap {
        /// The adj on which the two cells fall.
        adj : Adj
    },

    /// Does nothing, but counts towards the instruction length.
    Noop,

    /// Jumps through code a number of instructions equal to the current cell.
    JumpThruCode {
        /// R is forward in code, L is backward in code
        dir : Dir
    },

    /// Runs the contained instruction if the cell at the world head is not zero.
    IfNotZeroCond {
        /// The instruction to conditionally run.
        ins : Box<Ins>
    },

    /// Runs the contained instruction if the cell at the world head is zero.
    IfZeroCond {
        /// The instruction to conditionally run.
        ins : Box<Ins>
    },

    /// Runs one of two contained instruction at random.
    RandomlyChoose {
        /// The two instructions to randomly choose between.
        options : Box<(Ins, Ins)>
    },

    #[cfg(debug_assertions)]
    /// Dumps the current world state to the console.
    DumpWorld

}

impl Ins {

    /// Inverts the instruction.
    pub fn invert(self) -> Result<Self, BadInvertError> { match (self) {

        Self::MoveHeadOne { adj, dir } => Ok(Self::MoveHeadOne { adj, dir : -dir }),

        Self::MoveHeadDynamic { adj, dir } => Ok(Self::MoveHeadDynamic { adj, dir : -dir }),

        Self::Add { adj } => Ok(Self::Sub { adj }),

        Self::Sub { adj } => Ok(Self::Add { adj }),

        Self::Mul { adj } => Ok(Self::SDiv { adj }),

        Self::SDiv { adj } => Ok(Self::Mul { adj }),

        Self::Swap { .. } => Err(BadInvertError),

        Self::Noop => Err(BadInvertError),

        Self::JumpThruCode { dir } => Ok(Self::JumpThruCode { dir: -dir }),

        Self::IfZeroCond { ins } => Ok(Self::IfNotZeroCond { ins }),

        Self::IfNotZeroCond { ins } => Ok(Self::IfZeroCond { ins }),

        Self::RandomlyChoose { .. } => Err(BadInvertError),

        #[cfg(debug_assertions)]
        Self::DumpWorld => Err(BadInvertError)

    } }


    /// Applies an instruction modifier to this instruction.
    pub fn modify(self, modifier : InsMod) -> Result<Self, BadInvertError> {
        let ins = match (modifier.kind) {
            InsModKind::Invert        => self.clone().invert(),
            InsModKind::IfNotZeroCond => Ok(Self::IfNotZeroCond { ins : Box::new(self.clone()) }),
            InsModKind::Skip => Ok(Self::Noop),
        }?;
        Ok(if (modifier.random_maybe) {
            Ins::RandomlyChoose { options : Box::new((self, ins,)) }
        } else {
            ins
        })
    }

}


/// An instruction that can not be inverted was inverted.
pub struct BadInvertError;

// TODO tests