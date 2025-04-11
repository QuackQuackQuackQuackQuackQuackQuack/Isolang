//! Isolang script runner.


use crate::world::{ World, Cell, Adj, Coord, Dir };
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

    /// Get a mutable reference to the [`World`] in this runner.
    pub fn world_mut(&mut self) -> &mut World<C> { &mut self.state.world }

    /// Get a non-mutable reference to the [`World`] in this runner. 
    pub fn world(&self) -> &World<C> { &self.state.world }

}


impl<C : Cell> ScriptRunner<C> {

    /// Runs the next step in the script.
    ///
    /// ### Returns
    /// Returns `false` if the program has finished.
    pub fn run_next(&mut self) -> bool {
        let Some(ins) = self.script.get(self.state.script_head)
            else { return false; };
        if (self.state.run_ins(ins)) {
            self.state.script_head += 1;
        }
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
        let l = self.world.get(l);
        let r = self.world.get(r);
        self.world.insert(head, f(l, r));
    }


    /// Runs a single instruction in this [`World`].
    pub fn run_ins(&mut self, ins : &Ins) -> bool {

        match (ins) {

            Ins::MoveHeadOne { adj, dir } => { *self.world.head_mut() += (*adj, *dir,); },

            Ins::MoveHeadDynamic { adj, dir } => {
                let cell_val = self.world.get(self.world.head()).get_usize_val() as isize;
                *self.world.head_mut() += Coord::from((*adj, *dir)) * cell_val
            },

            Ins::Add { adj } => { self.run_binop(*adj, |a, b| a + b); },

            Ins::Sub { adj } => { self.run_binop(*adj, |a, b| a - b); },

            Ins::Mul { adj } => { self.run_binop(*adj, |a, b| a * b); },

            Ins::SDiv { adj } => { self.run_binop(*adj, |a, b| a / b); },

            Ins::Swap { adj } => {
                let head = self.world.head();
                let (l, r,) = head + *adj;
                let lv = self.world.get(l);
                let rv = self.world.get(r);
                self.world.insert(l, rv);
                self.world.insert(r, lv);
            },

            Ins::Noop => { },

            Ins::IfNotZeroCond { ins } => {
                if (self.world.get(self.world.head()) != C::ZERO) {
                    self.run_ins(ins);
                }
            },

            Ins::IfZeroCond { ins } => {
                if (self.world.get(self.world.head()) == C::ZERO) {
                    self.run_ins(ins);
                }
            },

            Ins::RandomlyChoose { options } => {
                if (random::<bool>()) { self.run_ins(&options.0); }
                else                  { self.run_ins(&options.1); }
            }

            Ins::JumpThruCode { dir } => {
                let cell_val = self.world.get(self.world.head()).get_usize_val();
                match (dir) {
                    Dir::L => { self.script_head = self.script_head.saturating_sub(cell_val); },
                    Dir::R => {
                        self.script_head = self.script_head.saturating_add(cell_val);
                    },
                }
                return false;
            },

            Ins::DumpWorld => {
                println!("\n{}", self.world);
            }

        }
        true
    }
}

// TODO tests
