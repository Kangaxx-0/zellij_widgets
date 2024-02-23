#![allow(unused_imports, dead_code)]
use crate::{buffer::Buffer, layout::Geometry, layout::Layout, widget::Widget};

/// A consistent view into the terminal state for rendering a single frame, think of it as a big
/// board that you can draw everything on, and everything will be rendered all at once.
///
/// # NOTE
/// This concept is borrowed from the `widgets` since [Immediate Mode Rendering](https://widgets.rs/concepts/rendering/) fits well with the zellij plugin architecture.
#[derive(Debug)]
pub struct Frame<'b> {
    /// Where should the cursor be after drawing this frame?
    ///
    /// If `None`, the cursor is hidden and its position is controlled by the backend. If `Some((x,
    /// y))`, the cursor is shown and placed at `(x, y)` after the call to `Terminal::draw()`.
    pub(crate) cursor_position: Option<(u16, u16)>,

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
}
