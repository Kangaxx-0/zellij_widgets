use crate::prelude::*;

pub use highlight_style::{HighlightStyle, HighlightSymbol};
pub use list_item::ListItem;
pub use state::ListState;

mod highlight_style;
mod list_item;
mod state;

/// A widget to display multiple items among which one can be selected
///
/// A list is a collection of [`ListItem`]s.
///
///
/// # Examples
/// ``` rust
/// use zellij_widgets::prelude::*;
///
/// let list1 = List::new_with_items(vec![
///     ListItem::new("Item 1"),
///     ListItem::new("Item 2"),
///     ListItem::new("Item 3")])
/// .block_style(Style::default().bg(Color::Red))
/// .highlight_style(HighlightStyle::new(HighlightSymbol::SingleArrow, Style::default().fg(Color::Yellow)));
/// ```
#[derive(Debug, Default, PartialEq, Hash)]
pub struct List<'a> {
    items: Vec<ListItem<'a>>,
    pub block: Option<Block<'a>>,
    pub block_style: Option<Style>,
    pub highlight_style: HighlightStyle,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_items(items: Vec<ListItem<'a>>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn block_style(mut self, block_style: Style) -> Self {
        self.block_style = Some(block_style);
        self
    }

    pub fn highlight_style(mut self, style: HighlightStyle) -> Self {
        self.highlight_style = style;
        self
    }

    pub fn item_style(&mut self, style: Style) {
        for item in &mut self.items {
            item.set_style(style)
        }
    }

    fn get_items_relative_pos(
        &self,
        max_length: usize,
        start_pos: usize,
        current_highlight: Option<usize>,
    ) -> (usize, usize) {
        let offset = start_pos.min(self.items.len().saturating_sub(1));
        let mut list_item_start_index = offset;
        let mut list_item_end_index = offset;
        let mut height = 0;
        for item in self.items.iter().skip(start_pos) {
            if height + item.height() >= max_length {
                break;
            }
            height += item.height();
            list_item_end_index += 1;
        }

        let current = current_highlight
            .unwrap_or(0)
            .min(self.items.len().saturating_sub(1));

        // If the current selected item is greater than the relative_end, we need to adjust the start position,
        // and recalculate the relative_end, e.g - Hit `Down` to move highlight_index to next.
        //
        // The new start position should be from the current selected item
        while current >= list_item_end_index {
            height = height.saturating_add(self.items[list_item_end_index].height());
            list_item_end_index += 1;
            while height > max_length {
                height = height.saturating_sub(self.items[list_item_start_index].height());
                list_item_start_index += 1;
            }
        }

        // If the current selected item is less than the relative_start, we need to adjust the start and end position.
        // E.g - Hit `Up` to move highlight_index to previous.
        while list_item_start_index > current {
            height = height.saturating_add(self.items[list_item_start_index].height());
            list_item_start_index -= 1;
            while height > max_length {
                list_item_end_index -= 1;
                height = height.saturating_sub(self.items[list_item_end_index].height());
            }
        }

        (list_item_start_index, list_item_end_index)
    }
}

impl<'a> StateWidget for List<'a> {
    type State = ListState;

    fn render(mut self, area: Geometry, buf: &mut Buffer, state: &mut ListState) {
        let block_style = self.block_style.unwrap_or_default();

        buf.set_style(area, block_style);

        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if self.items.is_empty() {
            return;
        }

        let max_length = list_area.rows as usize;
        let max_cols = list_area.cols;
        let (start, end) = self.get_items_relative_pos(
            max_length,
            state.start_position(),
            state.highlight_index(),
        );
        state.set_start_position(start);
        let mut current_height = 0;

        self.items
            .iter()
            .enumerate()
            .skip(start)
            .take(end - start)
            .for_each(|(i, item)| {
                let (x, y) = {
                    let pos = (list_area.left(), list_area.top() + current_height);
                    current_height += item.height() as u16;
                    pos
                };

                let item_gemo = Geometry {
                    x,
                    y,
                    cols: list_area.cols,
                    rows: item.height() as u16,
                };

                if let Some(index) = state.highlight_index() {
                    if index == i {
                        buf.set_style(item_gemo, self.highlight_style.style);
                        for (j, line) in item.field.lines.iter().enumerate() {
                            if j == 0 {
                                let line_text: String = line.clone().into();
                                let highlight_symbol =
                                    format!("{} {}", self.highlight_style.symbol, line_text);
                                buf.set_string(x, y, highlight_symbol, self.highlight_style.style);

                                let pos = self.highlight_style.symbol.len() as u16;

                                buf.set_line(
                                    item_gemo.x + pos + 1,
                                    item_gemo.y + j as u16,
                                    line,
                                    max_cols,
                                );
                            } else {
                                let pos = self.highlight_style.symbol.len() as u16;
                                buf.set_line(
                                    item_gemo.x + pos,
                                    item_gemo.y + j as u16,
                                    line,
                                    max_cols,
                                );
                            }
                        }
                    } else {
                        buf.set_style(area, item.style);
                        for (j, line) in item.field.lines.iter().enumerate() {
                            buf.set_line(item_gemo.x, item_gemo.y + j as u16, line, max_cols);
                        }
                    }
                }
            });
    }
}
