use bitflags::bitflags;
use std::fmt::{self, Debug};

bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on the block widget.
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Borders: u8 {
        /// Show no border (default)
        const NONE   = 0b0000;
        /// Show the top border
        const TOP    = 0b0001;
        /// Show the right border
        const RIGHT  = 0b0010;
        /// Show the bottom border
        const BOTTOM = 0b0100;
        /// Show the left border
        const LEFT   = 0b1000;
        /// Show all borders
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    }
}

/// Implement the `Debug` trait for the `Borders` bitflags. This is a manual implementation to
/// display the flags in a more readable way. The default implementation would display the
/// flags as 'Border(0x0)' for `Borders::NONE` for example.
impl Debug for Borders {
    /// Display the Borders bitflags as a list of names. For example, `Borders::NONE` will be
    /// displayed as `NONE` and `Borders::ALL` will be displayed as `ALL`. If multiple flags are
    /// set, they will be displayed separated by a pipe character.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        if self.is_all() {
            return write!(f, "ALL");
        }
        let mut first = true;
        for (name, border) in self.iter_names() {
            if border == Borders::NONE {
                continue;
            }
            if first {
                write!(f, "{name}")?;
                first = false;
            } else {
                write!(f, " | {name}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let borders = Borders::NONE;
        assert_eq!(format!("{:?}", borders), "NONE");
    }

    #[test]
    fn test_all() {
        let borders = Borders::ALL;
        assert_eq!(format!("{:?}", borders), "ALL");
    }

    #[test]
    fn test_individual_flags() {
        let top = Borders::TOP;
        let right = Borders::RIGHT;
        let bottom = Borders::BOTTOM;
        let left = Borders::LEFT;

        assert_eq!(format!("{:?}", top), "TOP");
        assert_eq!(format!("{:?}", right), "RIGHT");
        assert_eq!(format!("{:?}", bottom), "BOTTOM");
        assert_eq!(format!("{:?}", left), "LEFT");
    }

    #[test]
    fn test_combinations() {
        let top_right = Borders::TOP | Borders::RIGHT;
        let top_bottom_left = Borders::TOP | Borders::BOTTOM | Borders::LEFT;

        assert_eq!(format!("{:?}", top_right), "TOP | RIGHT");
        assert_eq!(format!("{:?}", top_bottom_left), "TOP | BOTTOM | LEFT");
    }
}
