#![allow(unused_imports, dead_code)]
//! Provides the [`Frame`] type.
//!
//! The [`Frame`] is a consistent view into the terminal state for rendering a single frame. It is obtained via
//! the closure argument of [`Terminal::draw`]. It is used to render widgets to the zellij host
//!
use crate::{
    buffer::Buffer,
    layout::Geometry,
    layout::Layout,
    widget::{StateWidget, Widget},
};

/// A consistent view into the terminal state for rendering a single frame, think of it as a big
/// board that you can draw everything on, and everything will be rendered all at once.
///
/// # NOTE
/// This concept is borrowed from the `ratatui` since [Immediate Mode Rendering](https://ratatui.rs/concepts/rendering/) fits well with the zellij plugin architecture, one caveat is that there is no loop in zellij.
#[derive(Debug)]
pub struct Frame<'b> {
    /// The area of the viewport
    pub(crate) viewport_area: Geometry,

    /// The buffer that is used to draw the current frame, it saved content will be rendered.
    pub(crate) buffer: &'b mut Buffer,
}

impl Frame<'_> {
    /// The size of the current frame
    ///
    /// This is guaranteed not to change when rendering.
    pub fn size(&self) -> Geometry {
        self.viewport_area
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`], or you can think of it
    /// as writing widget content to the buffer with the given area.
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    pub fn render_widget<W>(&mut self, widget: W, area: Geometry)
    where
        W: Widget,
    {
        widget.render(area, self.buffer);
    }

    /// Render a stateful [`Widget`] to the current buffer using [`Widget::render`], or you can think of it
    /// as writing widget content to the buffer with the given area,but with a state.
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    pub fn render_state_widget<W>(&mut self, widget: W, area: Geometry, state: &mut W::State)
    where
        W: StateWidget,
    {
        widget.render(area, self.buffer, state);
    }
}
