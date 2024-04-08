//! A prelude for conveniently writing applications using this library.
//!
//! ```rust,no_run
//! use zellij_widgets::prelude::*;
//! ```

pub use crate::{
    buffer::Buffer,
    frame::Frame,
    layout::Layout,
    layout::{self, Alignment, Constraint, Geometry, Margin, Orientation},
    plugin_pane::PluginPane,
    style::{self, symbols, Color, Modifier, Style, Styled, Stylize},
    text::{self, Line, Masked, Span, StyledGrapheme, Text},
    title::{self, Position, Title},
    uis::{
        self, Block, BorderOptions, BorderType, Borders, Erase, HighlightStyle, HighlightSymbol,
        List, ListItem, ListState, Padding, Paragraph, ScrollDirection, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Tab, TabState, Wrap,
    },
    widget::{StateWidget, Widget},
};
