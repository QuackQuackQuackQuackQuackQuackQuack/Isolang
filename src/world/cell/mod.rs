use core::fmt;
use core::ops::Add;


mod u8;
pub use u8::U8Cell;

mod u32;
pub use u32::U32Cell;



pub trait Cell : PartialEq + Clone + Copy + Default + fmt::Display + Add<Output = Self> { }
