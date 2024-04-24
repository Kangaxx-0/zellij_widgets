//! This module contains all the widgets that are available in the library.
pub use block::{Block, BorderOptions, BorderType, Borders, Padding};
pub use erase::Erase;
pub use gauge::Gauge;
pub use list::{HighlightStyle, HighlightSymbol, List, ListItem, ListState};
pub use paragraph::{Paragraph, Wrap};
pub use scrollbar::{ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState};
pub use tab::{Tab, TabState};

mod block;
mod erase;
mod gauge;
mod list;
mod paragraph;
mod reflow;
mod scrollbar;
mod tab;
