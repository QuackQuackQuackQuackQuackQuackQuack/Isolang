use core::fmt;
use core::ops::{ Add, Sub, Mul, Div };


mod u8;
pub use u8::U8Cell;

mod u32;
pub use u32::U32Cell;



pub trait Cell
where Self
    : PartialEq
    + Clone + Copy
    + Default
    + fmt::Display
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{ }
