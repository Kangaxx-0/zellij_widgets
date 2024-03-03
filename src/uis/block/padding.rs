use crate::prelude::*;
//
/// Defines the padding of a [`Block`].
///
/// See the [`padding`](Block::padding) method of [`Block`] to configure its padding.
///
/// This concept is similar to [CSS padding](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_model/Introduction_to_the_CSS_box_model#padding_area).
///
/// # Example
///
/// ```
/// # use zellij_widgets::prelude::*;
///
/// Padding::uniform(1);
/// Padding::horizontal(2);
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Padding {
    /// Left padding
    pub left: u16,
    /// Right padding
    pub right: u16,
    /// Top padding
    pub top: u16,
    /// Bottom padding
    pub bottom: u16,
}

impl Padding {
    /// Creates a new `Padding` by specifying every field individually.
    pub const fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Padding {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Creates a `Padding` of 0.
    ///
    /// This is also the default.
    pub const fn zero() -> Self {
        Padding {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    /// Defines the [`left`](Padding::left) and [`right`](Padding::right) padding.
    ///
    /// This leaves [`top`](Padding::top) and [`bottom`](Padding::bottom) to `0`.
    pub const fn horizontal(value: u16) -> Self {
        Padding {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    /// Defines the [`top`](Padding::top) and [`bottom`](Padding::bottom) padding.
    ///
    /// This leaves [`left`](Padding::left) and [`right`](Padding::right) at `0`.
    pub const fn vertical(value: u16) -> Self {
        Padding {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }

    /// Applies the same value to every `Padding` field.
    pub const fn uniform(value: u16) -> Self {
        Padding {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}
