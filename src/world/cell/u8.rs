//! A cell containing a [`u8`].


use crate::world::{ Cell, CellStdinReadError };
use core::ops::{ Add, Sub, Mul, Div };
use core::fmt;
use std::io::{ self, BufReader, Stdin, Read };


/// A cell containing a [`u8`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct U8Cell(u8);

impl Add for U8Cell {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for U8Cell {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for U8Cell {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for U8Cell {
    type Output = Self;
    fn div(self, rhs : Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Cell for U8Cell {

    type StdinReader = StdinU8Reader;

    const ZERO : Self = Self(0);
    const ONE  : Self = Self(1);

    fn get_usize_val(&self) -> usize {
        self.0 as usize
    }

    fn create_stdin_reader() -> Self::StdinReader {
        StdinU8Reader::default()
    }

}

impl fmt::Display for U8Cell {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl fmt::Debug for U8Cell {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// An iterator over u8s read from stdin.
pub struct StdinU8Reader {
    /// Buffered stdin reader.
    reader : BufReader<Stdin>
}

impl Default for StdinU8Reader {
    fn default() -> Self {
        Self { reader : BufReader::new(io::stdin()) }
    }
}

impl Iterator for StdinU8Reader {
    type Item = Result<U8Cell, CellStdinReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8];
        match (self.reader.read(&mut buf)) {
            Ok(0)    => None,
            Ok(1)    => Some(Ok(U8Cell(buf[0]))),
            Ok(_)    => unreachable!(),
            Err(err) => Some(Err(CellStdinReadError::Io(err)))
        }
    }

}
