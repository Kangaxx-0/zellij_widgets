use crate::prelude::*;

pub use highlight_item::HighlightItem;
pub use list_item::ListItem;
pub use state::ListState;

mod highlight_item;
mod list_item;
mod state;

#[derive(Debug, Default, PartialEq, Hash)]
pub struct List<'a> {
    items: Vec<ListItem<'a>>,
    block: Option<Block<'a>>,
    highlight_item: HighlightItem<'a>,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<ListItem<'a>>) -> Self {
        self.items = items;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn highlight_item(mut self, highlight_item: HighlightItem<'a>) -> Self {
        self.highlight_item = highlight_item;
        self
    }
}

impl<'a> StateWidget for List<'a> {
    type State = ListState;
    fn render(self, area: Geometry, buf: &mut Buffer, _state: &ListState) {
        let List {
            items,
            block,
            highlight_item,
        } = self;
        let block = block.unwrap_or_default();
        let inner = block.inner(area);
        block.render(area, buf);
        let mut offset = 0;
        for item in items {
            todo!()
        }
    }
}
