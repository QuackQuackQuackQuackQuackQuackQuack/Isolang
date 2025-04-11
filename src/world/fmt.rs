use crate::world::{ World, Cell, Coord };
use core::fmt;


impl<C : Cell> fmt::Display for World<C> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut min_x     = 0isize;
        let mut max_x     = 0isize;
        let mut min_y     = 0isize;
        let mut max_y     = 0isize;
        let mut max_value = 0usize;
        for (coord, value) in self.cells.iter().chain([(&self.head, self.cells.get(&self.head).unwrap_or(&C::ONE))]) {
            let x = coord.absolute_x();
            let y = coord.absolute_y();
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            max_value = max_value.max(value.get_usize_val());
        }
        let value_len = (max_value.checked_ilog10()).map_or(1, |v| (v as usize) + 1);

        writeln!(f, "\x1b[95m\x1b[1mWORLD\x1b[0m")?;
        for y in (min_y..=max_y).rev() {
            write!(f, "| ")?;
            let first_x = min_x.rem_euclid(2) != y.rem_euclid(2);
            if (first_x) { write!(f, "{: >value_len$}", "")?; }
            let mut x = min_x + (first_x as isize);
            while (x <= max_x) {
                let coord = Coord::from_absolute(x, y);
                let cell  = self.cells.get(&coord).cloned().unwrap_or(C::ONE).get_usize_val();
                match ((coord == Coord::ZERO, coord == self.head)) {
                    (true, true) => { write!(f, "\x1b[93m\x1b[1m")?; },
                    (true, false) => { write!(f, "\x1b[91m\x1b[1m")?; },
                    (false, true) => { write!(f, "\x1b[92m\x1b[1m")?; },
                    (false, false) => { }
                }
                if (coord == Coord::ZERO) {  }
                write!(f, "{: >value_len$}{: >value_len$}", cell, "")?;
                write!(f, "\x1b[0m")?;
                x += 2;
            }
            writeln!(f)?;
        }
        write!(f, "\x1b[94mBL{:-<width$}\x1b[0m", Coord::from_absolute(min_x, min_y), width = 2 + ((max_x.abs_diff(min_x) + 1) * value_len))?;
        write!(f, " | \x1b[92m\x1b[1mORIGIN{}\x1b[0m", Coord::ZERO)?;
        write!(f, " | \x1b[91m\x1b[1mHEAD{}\x1b[0m", self.head)?;
        Ok(())
    }
}
