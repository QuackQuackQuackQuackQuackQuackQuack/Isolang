use std::io::Read;

use crate::world::{ Adj, Dir, Cell };
use crate::runner::ScriptRunner;
use crate::runner::ins::{ Ins, InsMod, InsModKind };


impl<C : Cell> ScriptRunner<C> {

    fn parse_ins(&mut self) -> Result<Ins, ParseError> {
        let first_char = self.f.bytes().skip(self.code_head).next();
        match ' ' {
            '+' => Ok(Ins::Add { adj: self.parse_adj()? }),
            '*' => Ok(Ins::Mul { adj: self.parse_adj()? }),
            '~' => Ok(Ins::Swap { adj: self.parse_adj()? }),
            '<' => Ok(Ins::MoveHead { adj: self.parse_adj()?, dir: Dir::L }),
            '>' => Ok(Ins::MoveHead { adj: self.parse_adj()?, dir: Dir::R }),
            _ => todo!()
        }
    }

    fn parse_adj(&mut self) -> Result<Adj, ParseError> {
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
