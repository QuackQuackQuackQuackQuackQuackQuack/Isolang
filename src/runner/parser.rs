use crate::world::{ Adj, Dir, Cell };
use crate::runner::ScriptRunner;
use crate::runner::ins::{ Ins, InsMod, InsModKind };


impl<C : Cell> ScriptRunner<C> {

    fn parse_ins(&mut self) -> Result<Ins, ParseError> {
        todo!()
    }

    fn parse_adj(&mut self) -> Result<Adj, ParseError> {
        todo!()
    }

    fn parse_dir(&mut self) -> Result<Dir, ParseError> {
        todo!()
    }

    fn parse_ins_mod(&mut self) -> Result<InsMod, ParseError> {
        todo!()
    }

    fn parse_ins_mod_kind(&mut self) -> Result<InsModKind, ParseError> {
        todo!()
    }

}


pub enum ParseError {
    
}
