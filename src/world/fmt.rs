use crate::world::{ World, Cell, Coord };
use core::fmt;


impl<C : Cell> fmt::Display for World<C> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HEAD {}", self.head)?;

        let mut min_x     = 0isize;
        let mut max_x     = 0isize;
        let mut min_y     = 0isize;
        let mut max_y     = 0isize;
        let mut max_value = 0usize;
        for (coord, value) in &self.cells {
            let x = coord.absolute_x();
            let y = coord.absolute_y();
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            max_value = max_value.max(value.get_usize_val());
        }
        let value_len = (max_value.checked_ilog10()).map_or(1, |v| (v as usize) + 1);

        for y in (min_y..=max_y).rev() {
            write!(f, "| ")?;
            let first_x = min_x.rem_euclid(2) != y.rem_euclid(2);
            if (first_x) { write!(f, "{: >value_len$}", "")?; }
            let mut x = min_x + (first_x as isize);
            while (x <= max_x) {
                let coord = Coord::from_absolute(x, y);
                let cell  = self.cells.get(&coord).cloned().unwrap_or(C::ONE).get_usize_val();
                if (coord == Coord::ZERO) { write!(f, "\x1b[96m\x1b[1m")?; }
                write!(f, "{: >value_len$}{: >value_len$}", cell, "")?;
                if (coord == Coord::ZERO) { write!(f, "\x1b[0m")?; }
                x += 2;
            }
            writeln!(f)?;
        }
        write!(f, "{:-<width$}", Coord::from_absolute(min_x, min_y), width = 2 + ((max_x.abs_diff(min_x) + 1) * value_len))?;
        Ok(())
    }
}
