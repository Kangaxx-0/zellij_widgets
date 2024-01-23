use crate::buffer::Buffer;
use crate::layout::Geometry;
use crate::widget::Widget;

/// A consistent view into the terminal state for rendering a single frame.
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

    /// The buffer that is used to draw the current frame
    pub(crate) buffer: &'b mut Buffer,
}

impl Frame<'_> {
    /// The size of the current frame
    ///
    /// This is guaranteed not to change when rendering.
    pub fn size(&self) -> Geometry {
        self.viewport_area
    }

    /// Render a [`Widget`] to the current buffer using [`Widget::render`].
    ///
    /// Usually the area argument is the size of the current frame or a sub-area of the current
    /// frame (which can be obtained using [`Layout`] to split the total area).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use widgets::{backend::TestBackend, prelude::*, widgets::Block};
    /// # let backend = TestBackend::new(5, 5);
    /// # let mut terminal = Terminal::new(backend).unwrap();
    /// # let mut frame = terminal.get_frame();
    /// let block = Block::default();
    /// let area = Geometry::new(0, 0, 5, 5);
    /// frame.render_widget(block, area);
    /// ```
    ///
    /// [`Layout`]: crate::layout::Layout
    pub fn render_widget<W>(&mut self, widget: W, area: Geometry)
    where
        W: Widget,
    {
        widget.render(area, self.buffer);
    }
}
