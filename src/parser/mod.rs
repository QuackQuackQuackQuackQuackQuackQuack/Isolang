//! Isolang script parser.


use crate::world::{ Adj, Dir };
use crate::runner::ins::{ Ins, InsMod, InsModKind };
use std::io;
use std::iter::Peekable;


/// Isolang script parser.
pub struct ScriptParser<F : Iterator<Item = io::Result<u8>>> {
    /// An iterator over the bytes to parse.
    f : Peekable<F>
}

impl<F : Iterator<Item = io::Result<u8>>> ScriptParser<F> {

    /// Parse a script into a [`Vec`] of instructions.
    /// 
    /// ### Returns
    /// Returns:
    /// - `Ok(_)` if the script was successfully parsed.
    /// - `Err(_)` if some other error occured.
    pub fn parse(f : F) -> Result<Vec<Ins>, ParseError> {
        let mut parser = Self { f : f.peekable() };
        let mut script = Vec::new();
        while let Some(ins) = parser.parse_ins()? {
            script.push(ins);
        }
        Ok(script)
    }

}


impl<F : Iterator<Item = io::Result<u8>>> ScriptParser<F> {

    /// Gets the next unread character, and marks it as read.
    /// 
    /// # Returns
    /// Returns
    /// - `Ok(Some(_))` if a character was found.
    /// - `Ok(None)` if the end of the file has been reached.
    /// - `Err(_)` if some other error occured.
    fn next_char(&mut self) -> Result<Option<char>, ParseError> {
        let Some(ch) = self.f.next()
            else { return Ok(None); };
        Ok(Some(ch? as char))
    }

    /// Gets the next unread character without marking it as read.
    /// 
    /// # Returns
    /// Returns
    /// - `Ok(Some(_))` if a character was found.
    /// - `Ok(None)` if the end of the file has been reached.
    /// - `Err(_)` if some other error occured.
    /// 
    /// *Note: If this returns `Err(_)`, the item will be marked as read.*
    fn peek_char(&mut self) -> Result<Option<char>, ParseError> {
        match (self.f.peek()) {
            Some(Ok(b))  => Ok(Some(*b as char)),
            Some(Err(_)) => Err(ParseError::Io(self.f.next().unwrap().unwrap_err())),
            None         => Ok(None)
        }
    }

    /// Marks the next character as read.
    /// 
    /// ### Returns
    /// Returns `Err(_)` if some other error occured.
    fn skip_char(&mut self) -> Result<(), ParseError> {
        let _ = self.next_char()?;
        Ok(())
    }

}


impl<F : Iterator<Item = io::Result<u8>>> ScriptParser<F> {

    /// Parses a single instruction, including its arguments and modifiers.
    /// 
    /// # Returns
    /// Returns
    /// - `Ok(Some(_))` if an instruction was successfully parsed.
    /// - `Ok(None)` if there is no instruction.
    /// - `Err(_)` if some other error occured.
    fn parse_ins(&mut self) -> Result<Option<Ins>, ParseError> {
        loop {
            let Some(ch) = self.next_char()?
                else { return Ok(None); };
            return Ok(Some(match (ch) {
                '+' => Ins::Add { adj : self.parse_adj()? },
                '*' => Ins::Mul { adj : self.parse_adj()? },
                '~' => Ins::Swap { adj : self.parse_adj()? },
                '>' => Ins::MoveHeadOne { adj : self.parse_adj()?, dir : Dir::R },
                ';' => Ins::MoveHeadDynamic { adj: self.parse_adj()?, dir: Dir::R },
                ':' => Ins::JumpThruCode { dir: Dir::R },
                _   => { continue; }
            }));
        }
    }

    /// Parses a single adj (axis) character.
    /// 
    /// ### Returns
    /// Returns:
    /// - `Ok(_)` if an adj was successfully parsed.
    /// - `Err(_)` if some other error occured.
    fn parse_adj(&mut self) -> Result<Adj, ParseError> {
        let Some(ch) = self.next_char()?
            else { return Err(ParseError::BadEOF); };
        return Ok(match (ch) {
            '\\' => Adj::ULDR,
            '/' => Adj::DLUR,
            '-' => Adj::LR,
            '^' => Adj::D2,
            'v' => Adj::U2,
            _ => todo!()
        });
    }

    /// Parses a single left/right direction character.
    /// 
    /// ### Returns
    /// Returns:
    /// - `Ok(_)` if a dir was successfully parsed.
    /// - `Err(_)` if some other error occured.
    fn parse_dir(&mut self) -> Result<Dir, ParseError> {
        todo!()
    }

    /// Parses a single instruction modifier, if it exists.
    /// 
    /// # Returns
    /// Returns
    /// - `Ok(Some(_))` if an instruction modifier was successfully parsed.
    /// - `Ok(None)` if there is no instruction modifier.
    /// - `Err(_)` if some other error occured.
    fn parse_ins_mod(&mut self) -> Result<Option<InsMod>, ParseError> {
        // Get the kind of modifier, if possible.
        let kind = match (self.parse_ins_mod_kind()) {
            Ok(Some(kind)) => kind,
            Ok(None)       => { return Ok(None); },
            Err(err)       => { return Err(err); }
        };
        // Construct the value to return.
        let mut ins_mod = InsMod {
            kind,
            random_maybe : false
        };
        // Check for random_maybe.
        if let Some('#') = self.peek_char()? {
            ins_mod.random_maybe = true;
            self.skip_char()?;
        }
        // Return the parsed value.
        Ok(Some(ins_mod))
    }

    /// Parses a single instruction modifier kind, if it eixsts.
    /// 
    /// # Returns
    /// Returns
    /// - `Ok(Some(_))` if an instruction modifier kind was successfully parsed.
    /// - `Ok(None)` if there is no instruction modifier kind.
    /// - `Err(_)` if some other error occured.
    fn parse_ins_mod_kind(&mut self) -> Result<Option<InsModKind>, ParseError> {
        // Get the next unread character without marking it as read.
        let Some(ch) = self.peek_char()?
            else { return Ok(None); };
        // Figure out what modifier kind the character is.
        let ins_mod_kind = match (ch) {
            '!' => InsModKind::Invert,
            '?' => InsModKind::IfNotZeroCond,
            _   => { return Ok(None); }
        };
        // If the character was a valid modifier kind, mark it as read.
        self.skip_char()?;
        Ok(Some(ins_mod_kind))
    }

}


/// An error raised while parsing an Isolang script.
#[derive(Debug)]
pub enum ParseError {

    /// Some IO-related error occured.
    Io(io::Error),

    /// The end of the file was found, but not all required arguments were provided.
    BadEOF,

    /// An unexpected character was found.
    BadChar(char)
}

/// Allows using the `?` operator on `Err(io::Error)` types to auto-convert them to [`ParseError`].
impl From<io::Error> for ParseError {
    fn from(err : io::Error) -> Self { Self::Io(err) }
}
