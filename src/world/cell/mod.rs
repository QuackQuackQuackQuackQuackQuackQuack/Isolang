//! Cell traits and types.


use core::fmt;
use core::ops::{ Add, Sub, Mul, Div };
use std::io;
use std::str::Utf8Error;


mod u8;
pub use u8::U8Cell;

mod u32;
pub use u32::U32Cell;



/// A cell that can be in a [`World`].
pub trait Cell
where Self
    : PartialEq
    + Clone + Copy
    + fmt::Display
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{

    /// An iterator over values of this cell type read from stdin.
    type StdinReader : Iterator<Item = Result<Self, CellStdinReadError>>;

    /// A cell containing value zero.
    const ZERO : Self;
    /// A cell containing value one.
    const ONE  : Self;

    /// Converts value to usize
    fn get_usize_val(&self) -> usize;

    /// Creates an iterator over values of this cell type read from stdin.
    fn create_stdin_reader() -> Self::StdinReader;

}



/// An error occured while reading characters from a reader.
pub enum CellStdinReadError {
    /// An error was returned by the reader.
    Io(io::Error),
    /// An invalid UTF8 sequence was found.
    Utf8(Utf8Error)
}

impl From<io::Error> for CellStdinReadError {
    fn from(value : io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<Utf8Error> for CellStdinReadError {
    fn from(value : Utf8Error) -> Self {
        Self::Utf8(value)
    }
}
