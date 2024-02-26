use crate::text::Span;

use super::Style;

/// A trait for objects that have a `Style`.
///
/// This trait enables generic code to be written that can interact with any object that has a
/// `Style`. This is used by the `Stylize` trait to allow generic code to be written that can
/// interact with any object that can be styled.
///
/// In other words, UI component that implements `Styled` trait is able to call `Stylize` functions like `.bg()`
///
/// # Example
/// ```rust
/// use zellij_widgets::prelude::*;
/// // Call `.bg()` on a `Paragraph` object.
/// let parahraph = Paragraph::new("Hello, world!").bg(Color::Red);
pub trait Styled {
    type Item;

    fn style(&self) -> Style;
    fn set_style(self, style: Style) -> Self::Item;
}

impl<'a> Styled for &'a str {
    type Item = Span<'a>;

    fn style(&self) -> Style {
        Style::default()
    }

    fn set_style(self, style: Style) -> Self::Item {
        Span::styled(self, style)
    }
}

impl Styled for String {
    type Item = Span<'static>;

    fn style(&self) -> Style {
        Style::default()
    }

    fn set_style(self, style: Style) -> Self::Item {
        Span::styled(self, style)
    }
}

impl Styled for Style {
    type Item = Style;

    fn style(&self) -> Style {
        *self
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.patch(style)
    }
}
