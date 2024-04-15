use crate::text::Span;

use crate::prelude::*;

const DEFAULT_HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);

/// A state for the [`Tab`] widget.
///
/// It contains the index of the selected tab and the total number of tabs. The selected tab is the one that is currently highlighted.
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TabState {
    pub selected: usize,
    pub len: usize,
}

impl TabState {
    /// Create a new state with the given number of tabs.
    pub fn new(len: usize) -> Self {
        assert!(len > 0, "TabState must have at least one tab");
        Self { selected: 0, len }
    }

    /// Get the current index position
    pub fn current(&mut self, selected: usize) {
        self.selected = selected;
    }

    /// Go to the next tab where the index is wrapped around.
    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.len;
    }

    /// Go to the previous tab where the index is wrapped around.
    pub fn previous(&mut self) {
        self.selected = (self.selected + self.len - 1) % self.len;
    }

    /// Reset the index to the first tab where the index is 0.
    pub fn reset_index(&mut self) {
        self.selected = 0;
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Tab<'a> {
    /// The optional block of the tab.
    block: Option<Block<'a>>,
    /// The dividers between the tabs.
    divider: Span<'a>,
    /// The style of the tab.
    style: Style,
    /// The style of the tab that is selected.
    highlight_style: Style,
    /// The title of the tab.
    title: Vec<Line<'a>>,
}

impl<'a> Tab<'a> {
    /// Create a new tab by providing a title.
    pub fn new<T>(title: Vec<T>) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            block: None,
            divider: Span::raw(symbols::line::VERTICAL),
            style: Style::default(),
            highlight_style: DEFAULT_HIGHLIGHT_STYLE,
            title: title.into_iter().map(Into::into).collect(),
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
}

impl<'a> Styled for Tab<'a> {
    type Item = Tab<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

impl<'a> StateWidget for Tab<'a> {
    type State = TabState;
    fn render(self, area: Geometry, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let tabs_area = match self.block {
            Some(block) => {
                let inner_tabs_area = block.inner(area);
                block.render(area, buf);
                inner_tabs_area
            }
            None => area,
        };

        if tabs_area.rows < 1 {
            return;
        }

        let mut x = tabs_area.left();
        let title_len = self.title.len();
        for (idx, t) in self.title.into_iter().enumerate() {
            let last_indx = title_len - 1 == idx;
            let remaining_width = tabs_area.right().saturating_sub(x);

            if remaining_width == 0 {
                break;
            }

            // Title
            let y = tabs_area.top();
            let pos = buf.set_line(x, y, &t, remaining_width);
            if idx == state.selected {
                buf.set_style(
                    Geometry {
                        x,
                        y,
                        cols: pos.0.saturating_sub(x),
                        rows: 1,
                    },
                    self.highlight_style,
                );
            }
            x = pos.0;
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 || last_indx {
                break;
            }

            let pos = buf.set_span(x, tabs_area.top(), &self.divider, remaining_width);
            x = pos.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn tab_new_with_str() {
        let tab = Tab::new(Vec::from(["Tab"]));
        assert_eq!(tab.block, None);
        assert_eq!(tab.divider, Span::raw(symbols::line::VERTICAL));
        assert_eq!(tab.style, Style::default());
        assert_eq!(tab.highlight_style, DEFAULT_HIGHLIGHT_STYLE);
        assert_eq!(tab.title, vec![Line::raw("Tab")]);
    }

    #[test]
    fn tab_new_with_vec() {
        let tab_title = ["Tab", "Another Tab"];
        let tab = Tab::new(tab_title.iter().map(|s| Span::raw(*s)).collect::<Vec<_>>());
        assert_eq!(tab.block, None);
        assert_eq!(tab.divider, Span::raw(symbols::line::VERTICAL));
        assert_eq!(tab.style, Style::default());
        assert_eq!(tab.highlight_style, DEFAULT_HIGHLIGHT_STYLE);

        let line_1 = Line {
            spans: Vec::from([Span {
                content: Cow::Borrowed("Tab"),
                style: Style {
                    fg: None,
                    bg: None,
                    add_modifier: Modifier::empty(),
                    sub_modifier: Modifier::empty(),
                },
            }]),
            alignment: None,
        };

        let line_2 = Line {
            spans: Vec::from([Span {
                content: Cow::Borrowed("Another Tab"),
                style: Style {
                    fg: None,
                    bg: None,
                    add_modifier: Modifier::empty(),
                    sub_modifier: Modifier::empty(),
                },
            }]),
            alignment: None,
        };
        assert_eq!(tab.title, vec![line_1, line_2]);
    }

    #[test]
    fn tab_set_block() {
        let block = Block::default().title("Block");
        let tab = Tab::new(vec!["Tab"]).block(block.clone());
        assert_eq!(tab.block, Some(block));
    }

    #[test]
    fn tab_set_divider() {
        let tab = Tab::new(vec!["Tab"]).divider(Span::raw(" "));
        assert_eq!(tab.divider, Span::raw(" "));
    }

    #[test]
    fn tab_set_style() {
        let style = Style::default().fg(Color::Red);
        let tab = Tab::new(vec!["Tab"]).style(style);
        assert_eq!(tab.style, style);
    }

    #[test]
    fn tab_set_highlight_style() {
        let style = Style::default().fg(Color::Red);
        let tab = Tab::new(vec!["Tab"]).highlight_style(style);
        assert_eq!(tab.highlight_style, style);
    }

    #[test]
    fn tab_state_widget() {
        let tab = Tab::new(vec!["Tab1", "Tab2"]);
        let mut state = TabState::new(1);
        let mut buf = Buffer::empty(Geometry::new(20, 20));
        tab.render(Geometry::new(20, 20), &mut buf, &mut state);
        assert_eq!(buf.content()[0].symbol, String::from("T"));
        assert_eq!(buf.content()[1].symbol, String::from("a"));
        assert_eq!(buf.content()[2].symbol, String::from("b"));
        assert_eq!(buf.content()[3].symbol, String::from("1"));
        assert_eq!(buf.content()[4].symbol, String::from("|"));
        assert_eq!(buf.content()[5].symbol, String::from("T"));
        assert_eq!(buf.content()[6].symbol, String::from("a"));
        assert_eq!(buf.content()[7].symbol, String::from("b"));
        assert_eq!(buf.content()[8].symbol, String::from("2"));
    }
}
