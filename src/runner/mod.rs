use crate::world::{ World, Cell, Adj };
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

    fn binop(&mut self, adj : Adj, f : impl FnOnce(Cell, Cell) -> Cell) {
        let head = self.world.head();
        let (l, r,) = head + adj;
        self.world.insert(head, self.world.get(l) + self.world.get(r));
    }

    fn ins(&mut self, ins : Ins) { match (ins) {

        Ins::MoveHead { adj, dir } => { *self.world.head_mut() += (adj, dir,); },

        Ins::Add { adj } => { self.binop(adj, |a, b| a + b); },

        Ins::Sub { adj } => { self.binop(adj, |a, b| a - b); },

        Ins::Mul { adj } => { self.binop(adj, |a, b| a * b); },

        Ins::Div { adj } => { self.binop(adj, |a, b| a / b); },

    } }

}
