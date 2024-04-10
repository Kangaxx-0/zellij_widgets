//! [`PluginPane`] as it's name suggests, is a pane that is used to render the plugin's buffer to the host via wasm runtime.
//!
//! It acts the main interface that zellij plugin uses to communicate with the host.
//!
//! # Example
//! ```rust
//! use zellij_widgets::prelude::*;
//!
//! fn main() {
//!    let mut plugin_pane = PluginPane::new(std::io::stdout(), 10, 10);
//!    plugin_pane.draw(|f| { ui(f); });
//! }
//!
//! fn ui(frame: &mut Frame){
//!    frame.render_widget(Paragraph::new("Hello World"), frame.size());
//! }
//!
//! ```

use crate::{
    buffer::Buffer,
    core::cursor::MoveTo,
    frame::Frame,
    layout::Geometry,
    prelude::Modifier,
    queue,
    style::{
        Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor,
        SetUnderlineColor,
    },
};

use std::io::{self, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CompletedFrame<'a> {
    /// The buffer that was used to draw the last frame.
    pub buffer: &'a Buffer,
    /// The size of the last frame.
    pub area: Geometry,
}

///PluginPane presents a view of the plugin's buffer to the host
///
/// # Fields
///The `PluginPane` struct has 3 main components:
///1. `writer` - The writer that is used to write to the host via wasm runtime
///2. `geom` - The total rectangle size of the plugin pane
///3. `buffer` - The buffer that is used to render the plugin's content
///
/// # NOTE
///Always keep in mind that your code talks to the host via wasm runtime.
///That having been said, a lot of interfaces you familiar with are not available.
#[derive(Debug)]
pub struct PluginPane<W: Write> {
    /// Zellij plugin communicates with host via wasm sandbox file system
    writer: W,
    /// The total rectangle size of the plugin pane
    geom: Geometry,
    buffer: Buffer,
}

impl<W> PluginPane<W>
where
    W: Write,
{
    /// Set up a new `PluginPane` with the given writer and dimensions.
    /// You can assume that it starts at (0, 0) with rows and cols from `ZellijPlugin::render`
    pub fn new(writer: W, rows: u16, cols: u16) -> Self {
        Self {
            writer,
            geom: Geometry::new(rows, cols),
            buffer: Buffer::empty(Geometry::new(rows, cols)),
        }
    }

    // /// Write the given buffer to the host via wasm runtime
    // pub fn write(&mut self, buf: &[u8]) -> io::Result<()> {
    //     self.writer.write_all(buf)
    // }

    /// An important function that flushes the buffer, and it is also where the magic happens,
    /// such as setting foreground and background colors
    pub fn flush_buffer(&mut self) -> io::Result<()> {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;

        let mut last_pos: Option<(u16, u16)> = None;

        let mut contents = Vec::new();
        let mut modifier = Modifier::empty();

        for (index, content) in self.buffer.content().iter().enumerate() {
            let (x, y) = self.buffer.pos_of(index);
            contents.push((x, y, content));
        }

        for (x, y, cell) in contents {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                queue!(self.writer, MoveTo(x, y))?;
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                diff.queue(&mut self.writer)?;
                modifier = cell.modifier;
            }

            if cell.fg != fg {
                queue!(self.writer, SetForegroundColor(cell.fg))?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                queue!(self.writer, SetBackgroundColor(cell.bg))?;
                bg = cell.bg;
            }

            queue!(self.writer, Print(cell.symbol()))?;
        }

        queue!(
            self.writer,
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
            SetUnderlineColor(Color::Reset),
            SetAttribute(Attribute::Reset),
        )
    }

    /// Draw the given content to the plugin pane.
    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        let mut frame = self.get_frame();
        f(&mut frame);
        self.flush_buffer()?;

        self.flush()?;

        Ok(CompletedFrame {
            buffer: &self.buffer,
            area: self.geom,
        })
    }

    /// Get the current frame of the plugin pane.
    fn get_frame(&mut self) -> Frame {
        Frame {
            cursor_position: None,
            viewport_area: self.geom,
            buffer: self.current_buffer_mut(),
        }
    }

    /// Get a mutable reference to the current buffer.
    fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Finish writing to the host via wasm runtime
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// The `ModifierDiff` struct is used to calculate the difference between two `Modifier`
/// values. This is useful when updating the terminal display, as it allows for more
/// efficient updates by only sending the necessary changes.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct ModifierDiff {
    pub from: Modifier,
    pub to: Modifier,
}

impl ModifierDiff {
    /// Queue the necessary terminal escape sequences to update the terminal display
    fn queue<W>(&self, mut w: W) -> io::Result<()>
    where
        W: io::Write,
    {
        let removed = self.from - self.to;
        if removed.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(Attribute::NoReverse))?;
        }
        if removed.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(Attribute::NormalIntensity))?;
            if self.to.contains(Modifier::DIM) {
                queue!(w, SetAttribute(Attribute::Dim))?;
            }
        }
        if removed.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(Attribute::NoItalic))?;
        }
        if removed.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(Attribute::NoUnderline))?;
        }
        if removed.contains(Modifier::DIM) {
            queue!(w, SetAttribute(Attribute::NormalIntensity))?;
        }
        if removed.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(Attribute::NotCrossedOut))?;
        }
        if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(Attribute::NoBlink))?;
        }

        let added = self.to - self.from;
        if added.contains(Modifier::REVERSED) {
            queue!(w, SetAttribute(Attribute::Reverse))?;
        }
        if added.contains(Modifier::BOLD) {
            queue!(w, SetAttribute(Attribute::Bold))?;
        }
        if added.contains(Modifier::ITALIC) {
            queue!(w, SetAttribute(Attribute::Italic))?;
        }
        if added.contains(Modifier::UNDERLINED) {
            queue!(w, SetAttribute(Attribute::Underlined))?;
        }
        if added.contains(Modifier::DIM) {
            queue!(w, SetAttribute(Attribute::Dim))?;
        }
        if added.contains(Modifier::CROSSED_OUT) {
            queue!(w, SetAttribute(Attribute::CrossedOut))?;
        }
        if added.contains(Modifier::SLOW_BLINK) {
            queue!(w, SetAttribute(Attribute::SlowBlink))?;
        }
        if added.contains(Modifier::RAPID_BLINK) {
            queue!(w, SetAttribute(Attribute::RapidBlink))?;
        }

        Ok(())
    }
}
