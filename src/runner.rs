use crate::world::{ Axis, Dir };
use std::fs::File;


pub struct ScriptRunner {
    f : File
}

impl From<File> for ScriptRunner {
    fn from(f : File) -> Self { Self { f } }
}


pub enum Ins {

    MoveHead {
        axis : Axis,
        dir  : Dir
    },

    Add {
        axis : Axis
    }

}
