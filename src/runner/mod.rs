//! Isolang script runner.


use crate::world::{ World, Cell, Adj };
use rand::random;


pub mod ins;
use ins::Ins;


/// Isolang script runner.
pub struct ScriptRunner<C : Cell> {

    /// The instructions in the script.
    script : Vec<Ins>,

    /// The current running state.
    state  : ScriptRunnerState<C>

}

/// The state of a [`ScriptRunner`].
struct ScriptRunnerState<C : Cell> {

    /// The script running head.
    script_head : usize,

    /// The world to run on.
    world       : World<C>

}

impl<C : Cell> ScriptRunner<C> {

    /// Construct a new runner from a [`Vec`] of instructions.
    pub fn new(script : Vec<Ins>) -> Self { Self {
        script,
        state : ScriptRunnerState {
            script_head : 0,
            world       : World::default()
        }
    } }

}


impl<C : Cell> ScriptRunner<C> {

    /// Runs the next step in the script.
    /// 
    /// ### Returns
    /// Returns `false` if the program has finished.
    pub fn run_next(&mut self) -> bool {
        let Some(ins) = self.script.get(self.state.script_head)
            else { return false; };
        self.state.run_ins(ins);
        true
    }

}

impl<C : Cell> ScriptRunnerState<C> {

    /// Calls the function `f` on the two cells currently targeted by `adj`.
    fn run_binop<F>(&mut self, adj : Adj, f : F)
    where
        F : FnOnce(C, C) -> C
    {
        let head = self.world.head();
        let (l, r,) = head + adj;
        self.world.insert(head, f(self.world.get(l), self.world.get(r)));
    }


    /// Runs a single instruction in this [`World`].
    pub fn run_ins(&mut self, ins : &Ins) { match (ins) {

        Ins::MoveHeadOne { adj, dir } => { *self.world.head_mut() += (*adj, *dir,); },

        Ins::MoveHeadDynamic { adj, dir } => { *self.world.head_mut() += self.world.get(self.world.head()).get_usize_val() * 
        (*adj, *dir)},

        Ins::Add { adj } => { self.run_binop(*adj, |a, b| a + b); },

        Ins::Sub { adj } => { self.run_binop(*adj, |a, b| a - b); },

        Ins::Mul { adj } => { self.run_binop(*adj, |a, b| a * b); },

        Ins::SDiv { adj } => { self.run_binop(*adj, |a, b| a / b); },

        Ins::Swap { adj } => {
            let head = self.world.head();
            let (l, r,) = head + *adj;
            let save_l_val = self.world.get(l);
            self.world.insert(l, self.world.get(r));
            self.world.insert(r, save_l_val);
        },

        Ins::Noop => { },

        Ins::IfNotZeroCond { ins } => {
            if (! self.world.get(self.world.head()).is_zero()) {
                self.run_ins(ins);
            }
        },

        Ins::IfZeroCond { ins } => {
            if (self.world.get(self.world.head()).is_zero()) {
                self.run_ins(ins);
            }
        },

        Ins::RandomlyChoose { options } => {
            if (random::<bool>()) { self.run_ins(&options.0); }
            else                  { self.run_ins(&options.1); }
        }

    } }

}
