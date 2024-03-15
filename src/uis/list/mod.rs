use crate::prelude::*;

pub use highlight_style::HighlightStyle;
pub use list_item::ListItem;
pub use state::ListState;

mod highlight_style;
mod list_item;
mod state;

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

    fn get_items_bounds(&self, max_length: usize, start_pos: usize) -> (usize, usize) {
        let offset = start_pos.min(self.items.len().saturating_sub(1));
        let relative_start = offset;
        let mut relative_end = offset;
        let mut height = 0;
        for item in self.items.iter().skip(start_pos) {
            if height + item.height() >= max_length {
                break;
            }
            height += item.height();
            relative_end += 1;
        }

        (relative_start, relative_end)
    }
}

impl<'a> StateWidget for List<'a> {
    type State = ListState;

    fn render(self, area: Geometry, buf: &mut Buffer, state: &mut ListState) {
        let block_style = self.block_style.unwrap_or_default();

        buf.set_style(area, block_style);

        let list_area = match self.block.clone() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(inner_area, buf);
                inner_area
            }
            None => area,
        };

        if self.items.is_empty() {
            return;
        }

        let max_length = list_area.rows as usize;
        let max_cols = list_area.cols;
        let (start, end) = self.get_items_bounds(max_length, state.start_pos_to_display);

        let mut current_height = 0;

        self.items
            .iter()
            .skip(start)
            .take(end - start)
            .enumerate()
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
                                let highlight_symbol =
                                    format!("{} {}", self.highlight_style.symbol, line.to_string());
                                buf.set_string(x, y, highlight_symbol, self.highlight_style.style);

                                let pos = self.highlight_style.symbol.len() as u16;

                                buf.set_line(
                                    item_gemo.x + pos + 1,
                                    item_gemo.y + j as u16,
                                    &line,
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
