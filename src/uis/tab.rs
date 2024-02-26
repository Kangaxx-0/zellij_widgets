use crate::text::Span;

use crate::prelude::*;

const DEFAULT_HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Tab<'a> {
    /// The optional block of the tab.
    block: Option<Block<'a>>,
    /// The dividers between the tabs.
    divider: Span<'a>,
    /// The index of the selected tab.
    selected: usize,
    /// The style of the tab.
    style: Style,
    /// The style of the tab that is selected.
    highlight_style: Style,
    /// The title of the tab.
    title: Line<'a>,
}

impl<'a> Tab<'a> {
    /// Create a new tab by providing a title.
    pub fn new(title: impl Into<Line<'a>>) -> Self {
        Self {
            block: None,
            divider: Span::raw(symbols::line::VERTICAL),
            selected: 0,
            style: Style::default(),
            highlight_style: DEFAULT_HIGHLIGHT_STYLE,
            title: title.into(),
        }
    }

    /// Set the block of the tab.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the divider between the tabs.
    pub fn divider(mut self, divider: impl Into<Span<'a>>) -> Self {
        self.divider = divider.into();
        self
    }

    /// Set the style of the tab.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the style of the tab that is selected.
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Set the index of the selected tab.
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn tab_new_with_str() {
        let tab = Tab::new("Tab");
        assert_eq!(tab.block, None);
        assert_eq!(tab.divider, Span::raw(symbols::line::VERTICAL));
        assert_eq!(tab.selected, 0);
        assert_eq!(tab.style, Style::default());
        assert_eq!(tab.highlight_style, DEFAULT_HIGHLIGHT_STYLE);
        assert_eq!(tab.title, Line::raw("Tab"));
    }

    #[test]
    fn tab_new_with_vec() {
        let tab_title = vec!["Tab", "Another Tab"];
        let tab = Tab::new(tab_title.iter().map(|s| Span::raw(*s)).collect::<Vec<_>>());
        assert_eq!(tab.block, None);
        assert_eq!(tab.divider, Span::raw(symbols::line::VERTICAL));
        assert_eq!(tab.selected, 0);
        assert_eq!(tab.style, Style::default());
        assert_eq!(tab.highlight_style, DEFAULT_HIGHLIGHT_STYLE);
        assert_eq!(
            tab.title,
            Line {
                spans: Vec::from([
                    Span {
                        content: Cow::Borrowed("Tab"),
                        style: Style {
                            fg: None,
                            bg: None,
                            add_modifier: Modifier::empty(),
                            sub_modifier: Modifier::empty()
                        }
                    },
                    Span {
                        content: Cow::Borrowed("Another Tab"),
                        style: Style {
                            fg: None,
                            bg: None,
                            add_modifier: Modifier::empty(),
                            sub_modifier: Modifier::empty()
                        }
                    }
                ]),
                alignment: None
            }
        );
    }

    #[test]
    fn tab_set_block() {
        let block = Block::default().title("Block");
        let tab = Tab::new("Tab").block(block.clone());
        assert_eq!(tab.block, Some(block));
    }

    #[test]
    fn tab_set_divider() {
        let tab = Tab::new("Tab").divider(Span::raw(" "));
        assert_eq!(tab.divider, Span::raw(" "));
    }

    #[test]
    fn tab_set_style() {
        let style = Style::default().fg(Color::Red);
        let tab = Tab::new("Tab").style(style);
        assert_eq!(tab.style, style);
    }

    #[test]
    fn tab_set_highlight_style() {
        let style = Style::default().fg(Color::Red);
        let tab = Tab::new("Tab").highlight_style(style);
        assert_eq!(tab.highlight_style, style);
    }

    #[test]
    fn tab_set_selected() {
        let tab = Tab::new("Tab").selected(1);
        assert_eq!(tab.selected, 1);
    }
}
