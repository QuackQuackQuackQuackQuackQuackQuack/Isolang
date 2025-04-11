//! Isolang script parser.


use crate::world::{ Adj, Dir };
use crate::runner::ins::{ Ins, InsMod, InsModKind, BadInvertError };
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
            let mut ins = match (ch) {
                '+' => Ins::Add { adj : self.parse_adj()? },
                '*' => Ins::Mul { adj : self.parse_adj()? },
                '~' => Ins::Swap { adj : self.parse_adj()? },
                '>' => Ins::MoveHeadOne { adj : self.parse_adj()?, dir : Dir::R },
                ';' => Ins::MoveHeadDynamic { adj: self.parse_adj()?, dir: Dir::R },
                ':' => Ins::JumpThruCode { dir: Dir::R },
                #[cfg(debug_assertions)]
                '@' => Ins::DumpWorld,
                ' '|'\n'|'\t'|'\r' => { continue; }
                _   => {
                    #[cfg(debug_assertions)]
                    return Err(ParseError::BadChar(ch));
                    #[cfg(not(debug_assertions))]
                    continue;
                }
            };
            while let Some(ins_mod) = self.parse_ins_mod()? {
                ins = ins.modify(ins_mod)?;
            }
            return Ok(Some(ins));
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
        Ok(match (ch) {
            '\\' => Adj::ULDR,
            '/' => Adj::DLUR,
            '-' => Adj::LR,
            '^' => Adj::D2,
            'v' => Adj::U2,
            _ => return Err(ParseError::BadChar(ch))
        })
    }

    /// Parses a single instruction modifier.
    /// 
    /// ### Returns
    /// Returns:
    /// - `Ok(Some(_))` if an instruction modifier was successfully parsed.
    /// - `Ok(None)` if no instruction modifier was parsed.
    /// - `Err())` if some other error occured.
    fn parse_ins_mod(&mut self) -> Result<Option<InsMod>, ParseError> {
        let Some(ch) = self.peek_char()?
            else { return Ok(None); };
        if (ch == '#') {
            self.skip_char()?;
            Ok(Some(InsMod {
                kind         : InsModKind::Skip,
                random_maybe : true
            }))
        } else {
            let kind = match (ch) {
                '?' => InsModKind::IfNotZeroCond,
                '!' => InsModKind::Invert,
                _   => { return Ok(None); }
            };
            self.skip_char()?;
            let mut random_maybe = false;
            if let Some('#') = self.peek_char()? {
                random_maybe = true;
                self.skip_char()?;
            }
            Ok(Some(InsMod { kind, random_maybe }))
        }
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
    BadChar(char),

    /// An instruction that can not be inverted was inverted.
    BadInvert

}

/// Allows using the `?` operator on `Err(io::Error)` types to auto-convert them to [`ParseError`].
impl From<io::Error> for ParseError {
    fn from(err : io::Error) -> Self { Self::Io(err) }
}

/// Allows using the `?` operator on `Err(BadInvertError)` types to auto-convert them to [`ParseError`].
impl From<BadInvertError> for ParseError {
    fn from(_ : BadInvertError) -> Self { Self::BadInvert }
}

// TODO tests