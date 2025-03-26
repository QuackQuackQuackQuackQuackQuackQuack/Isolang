use crate::world::{ World, Cell };
use std::fs::File;


mod ins;
pub use ins::Ins;


pub struct ScriptRunner<C : Cell> {
    f     : File,
    world : World<C>
}

impl<C : Cell> ScriptRunner<C> {
    pub fn new(f : File) -> Self {
        Self { f, world : World::new() }
    }
}


impl<C : Cell> ScriptRunner<C> {

    fn ins(&mut self, ins : Ins) { match (ins) {

        Ins::MoveHead { adj, dir } => { *self.world.head_mut() += (adj, dir,); },

        Ins::Add { adj } => {
            let head = self.world.head();
            let (l, r,) = head + adj;
            self.world.insert(head, self.world.get(l) + self.world.get(r));
        }

    } }

}
