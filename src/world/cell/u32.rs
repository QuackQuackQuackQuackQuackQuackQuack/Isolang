//! A cell containing a [`u8`].


use crate::world::{ Cell, CellStdinReadError };
use core::ops::{ Add, Sub, Mul, Div };
use core::fmt;
use std::io::{ self, Read, BufReader, Stdin };


/// A cell containing a [`u32`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(transparent)]
pub struct U32Cell(u32);

impl Default for U32Cell {
    #[inline(always)]
    fn default() -> Self { Self(1) }
}

impl Add for U32Cell {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for U32Cell {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for U32Cell {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for U32Cell {
    type Output = Self;
    fn div(self, rhs : Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Cell for U32Cell {

    type StdinReader = StdinCharReader;

    const ZERO : Self = Self(0);
    const ONE  : Self = Self(1);

    fn get_usize_val(&self) -> usize {
        self.0 as usize
    }

    fn create_stdin_reader() -> Self::StdinReader {
        StdinCharReader::default()
    }

}

impl fmt::Display for U32Cell {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (char::from_u32(self.0)) {
            Some(ch) => write!(f, "{}", ch),
            None     => Ok(())
        }
    }
}

impl fmt::Debug for U32Cell {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// An iterator over UTF8 characters read from stdin.
pub struct StdinCharReader {
    /// Buffered stdin reader.
    reader : BufReader<Stdin>
}

impl Default for StdinCharReader {
    fn default() -> Self {
        Self { reader : BufReader::new(io::stdin()) }
    }
}

impl Iterator for StdinCharReader {
    type Item = Result<U32Cell, CellStdinReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bytes = [0u8; 4];
        for n in 0..4 {
            let mut buf = [0u8];
            match (self.reader.read(&mut buf)) {
                Ok(0)    => { return None; },
                Ok(1)    => { bytes[n] = buf[0]; },
                Ok(_)    => unreachable!(),
                Err(err) => { return Some(Err(err.into())); }
            }
            match (str::from_utf8(&bytes[..=n])) {
                Ok(s) => { return Some(Ok(U32Cell(s.chars().next().unwrap() as u32))); },
                Err(err) if (err.error_len().is_some()) => {
                    return Some(Err(err.into()));
                },
                _ => { }
            }
        }
        unreachable!()
    }
}
