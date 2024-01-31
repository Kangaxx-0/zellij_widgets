use crate::{
    buffer::Buffer,
    core::cursor::MoveTo,
    core::style::{
        Attribute as CAttribute, Color, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor, SetUnderlineColor,
    },
    frame::Frame,
    layout::Geometry,
    queue,
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
    /// Always starts at (0, 0) with rows and cols from `ZellijPlugin::render`
    pub fn new(writer: W, rows: u16, cols: u16) -> Self {
        Self {
            writer,
            geom: Geometry::new(rows, cols),
            buffer: Buffer::empty(Geometry::new(rows, cols)),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }

    pub fn flush_buffer(&mut self) -> io::Result<()> {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;

        let mut last_pos: Option<(u16, u16)> = None;

        let mut contents = Vec::new();

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
            if cell.fg != fg {
                let color = Color::from(cell.fg);
                queue!(self.writer, SetForegroundColor(color))?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                let color = Color::from(cell.bg);
                queue!(self.writer, SetBackgroundColor(color))?;
                bg = cell.bg;
            }

            queue!(self.writer, Print(cell.symbol()))?;
        }

        return queue!(
            self.writer,
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
            SetUnderlineColor(Color::Reset),
            SetAttribute(CAttribute::Reset),
        );
    }

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

    fn get_frame(&mut self) -> Frame {
        Frame {
            cursor_position: None,
            viewport_area: self.geom,
            buffer: self.current_buffer_mut(),
        }
    }

    fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    // Flush everything to wasmer runtime stdout
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
