use strum::{Display, EnumString};

use crate::style::symbols::border;
use crate::uis::Block;

/// The type of border of a [`Block`].
///
/// See the [`borders`](Block::borders) method of `Block` to configure its borders.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BorderType {
    /// A plain, simple border.
    ///
    /// This is the default
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌───────┐
    /// │       │
    /// └───────┘
    /// ```
    #[default]
    Plain,
    /// A plain border with rounded corners.
    ///
    /// # Example
    ///
    /// ```plain
    /// ╭───────╮
    /// │       │
    /// ╰───────╯
    /// ```
    Rounded,
    /// A doubled border.
    ///
    /// Note this uses one character that draws two lines.
    ///
    /// # Example
    ///
    /// ```plain
    /// ╔═══════╗
    /// ║       ║
    /// ╚═══════╝
    /// ```
    Double,
    /// A thick border.
    ///
    /// # Example
    ///
    /// ```plain
    /// ┏━━━━━━━┓
    /// ┃       ┃
    /// ┗━━━━━━━┛
    /// ```
    Thick,
    /// A border with a single line on the inside of a half block.
    ///
    /// # Example
    ///
    /// ```plain
    /// ▗▄▄▄▄▄▄▄▖
    /// ▐       ▌
    /// ▐       ▌
    /// ▝▀▀▀▀▀▀▀▘
    QuadrantInside,

    /// A border with a single line on the outside of a half block.
    ///
    /// # Example
    ///
    /// ```plain
    /// ▛▀▀▀▀▀▀▀▜
    /// ▌       ▐
    /// ▌       ▐
    /// ▙▄▄▄▄▄▄▄▟
    QuadrantOutside,
}

impl BorderType {
    /// Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
    pub const fn border_symbols(border_type: BorderType) -> border::Set {
        match border_type {
            BorderType::Plain => border::PLAIN,
            BorderType::Rounded => border::ROUNDED,
            BorderType::Double => border::DOUBLE,
            BorderType::Thick => border::THICK,
            BorderType::QuadrantInside => border::QUADRANT_INSIDE,
            BorderType::QuadrantOutside => border::QUADRANT_OUTSIDE,
        }
    }

    /// Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols.
    pub const fn to_border_set(self) -> border::Set {
        Self::border_symbols(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_type() {
        let plain = BorderType::Plain;
        assert_eq!(plain.to_border_set(), border::PLAIN);

        let rounded = BorderType::Rounded;
        assert_eq!(rounded.to_border_set(), border::ROUNDED);

        let double = BorderType::Double;
        assert_eq!(double.to_border_set(), border::DOUBLE);

        let thick = BorderType::Thick;
        assert_eq!(thick.to_border_set(), border::THICK);

        let quadrant_inside = BorderType::QuadrantInside;
        assert_eq!(quadrant_inside.to_border_set(), border::QUADRANT_INSIDE);

        let quadrant_outside = BorderType::QuadrantOutside;
        assert_eq!(quadrant_outside.to_border_set(), border::QUADRANT_OUTSIDE);
    }
}
