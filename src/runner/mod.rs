use crate::world::{ World, Cell, Adj };
use std::fs::File;
use rand::random;


pub mod ins;
use ins::Ins;

mod parser;


pub struct ScriptRunner<C : Cell> {
    f     : File,
    world : World<C>,
    code_head : usize
}

impl<C : Cell> ScriptRunner<C> {
    pub fn new(f : File) -> Self {
        Self { f, world : World::new() , code_head : 0}
    }
}


impl<C : Cell> ScriptRunner<C> {

    fn run_binop<F>(&mut self, adj : Adj, f : F)
    where
        F : FnOnce(C, C) -> C
    {
        let head = self.world.head();
        let (l, r,) = head + adj;
        self.world.insert(head, f(self.world.get(l), self.world.get(r)));
    }

    pub fn run_ins(&mut self, ins : Ins) { match (ins) {

        Ins::MoveHead { adj, dir } => { *self.world.head_mut() += (adj, dir,); },

        Ins::Add { adj } => { self.run_binop(adj, |a, b| a + b); },

        Ins::Sub { adj } => { self.run_binop(adj, |a, b| a - b); },

        Ins::Mul { adj } => { self.run_binop(adj, |a, b| a * b); },

        Ins::SDiv { adj } => { self.run_binop(adj, |a, b| a / b); },

        Ins::IfNotZeroCond { ins } => {
            if (! self.world.get(self.world.head()).is_zero()) {
                self.run_ins(*ins);
            }
        },

        Ins::IfZeroCond { ins } => {
            if (self.world.get(self.world.head()).is_zero()) {
                self.run_ins(*ins);
            }
        },

        Ins::RandomlyChoose { options } => {
            if (random::<bool>()) { self.run_ins(options.0); }
            else                  { self.run_ins(options.1); }
        }

    } }

}
