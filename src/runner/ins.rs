use crate::world::{ Adj, Dir };


pub enum Ins {

    MoveHead {
        adj : Adj,
        dir : Dir
    },

    Add {
        adj : Adj
    }

}
