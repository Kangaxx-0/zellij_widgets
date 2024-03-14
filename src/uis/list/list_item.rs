use crate::prelude::*;

/// A single item in a [`List`]
///
/// The item's height is defined by the number of lines it contains. This can be queried using
/// [`ListItem::height`]. Similarly, [`ListItem::width`] will return the maximum width of all
/// lines.
///
/// You can set the style of an item with [`ListItem::style`] or using the [`Stylize`] trait.
/// This [`Style`] will be combined with the [`Style`] of the inner [`Text`]. The [`Style`]
/// of the [`Text`] will be added to the [`Style`] of the [`ListItem`].
///
/// # Examples
///
/// You can create [`ListItem`]s from simple `&str`
///
/// ```rust
/// # use ratatui::{prelude::*, widgets::*};
/// let item = ListItem::new("Item 1");
/// ```
///
/// Anything that can be converted to [`Text`] can be a [`ListItem`].
///
/// ```rust
/// # use zellij_widgets::prelude::*;
/// let item1: ListItem = "Item 1".into();
/// let item2: ListItem = Line::raw("Item 2").into();
/// ```
///
/// A [`ListItem`] styled with [`Stylize`]
///
/// ```rust
/// # use zellij_widgets::prelude::*;
/// let item = ListItem::new("Item 1").red().on_white();
/// ```
///
/// If you need more control over the item's style, you can explicitly style the underlying
/// [`Text`]
///
/// ```rust
/// # use zellij_widgets::prelude::*;
/// let mut text = Text::default();
/// text.extend(["Item".blue(), Span::raw(" "), "1".bold().red()]);
/// let item = ListItem::new(text);
/// ```
///
/// [`Stylize`]: crate::style::Stylize
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListItem<'a> {
    pub field: Text<'a>,
    pub style: Style,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(text: T) -> ListItem<'a>
    where
        T: Into<Text<'a>>,
    {
        ListItem {
            field: text.into(),
            style: Style::default(),
        }
    }

    pub(crate) fn height(&self) -> usize {
        self.field.height()
    }

    pub(crate) fn width(&self) -> usize {
        self.field.width()
    }

    pub(crate) fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_item() {
        let item = ListItem::new("Item 1");
        assert_eq!(item.height(), 1);
        assert_eq!(item.width(), 6);
    }

    #[test]
    fn list_item_styled() {
        let mut item = ListItem::new("Item 1");
        item.set_style(Style::default().fg(Color::Red));
        assert_eq!(item.height(), 1);
        assert_eq!(item.width(), 6);
    }
}
