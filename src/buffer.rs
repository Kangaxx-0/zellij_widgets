use std::{
    cmp::min,
    fmt::{Debug, Formatter, Result},
};

use crate::core::style::Color;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::layout::Geometry;
use crate::styles::{Modifier, Style};
use crate::text::{Line, Span};

/// A buffer cell
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub symbol: String,
    pub modifier: Modifier,
    pub skip: bool,
}

#[allow(deprecated)] // For Cell::symbol
impl Cell {
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push_str(symbol);
        self
    }

    pub fn set_char(&mut self, ch: char) -> &mut Cell {
        self.symbol.clear();
        self.symbol.push(ch);
        self
    }

    pub fn set_fg(&mut self, color: Color) -> &mut Cell {
        self.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: Color) -> &mut Cell {
        self.bg = color;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Cell {
        if let Some(c) = style.fg {
            self.fg = c;
        }
        if let Some(c) = style.bg {
            self.bg = c;
        }
        self.modifier.insert(style.add_modifier);
        self.modifier.remove(style.sub_modifier);
        self
    }

    pub fn style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .bg(self.bg)
            .add_modifier(self.modifier)
    }

    /// Sets the cell to be skipped when copying (diffing) the buffer to the screen.
    ///
    /// This is helpful when it is necessary to prevent the buffer from overwriting a cell that is
    /// covered by an image from some terminal graphics protocol (Sixel / iTerm / Kitty ...).
    pub fn set_skip(&mut self, skip: bool) -> &mut Cell {
        self.skip = skip;
        self
    }

    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
        self.skip = false;
    }
}

impl Default for Cell {
    fn default() -> Cell {
        #[allow(deprecated)] // For Cell::symbol
        Cell {
            symbol: " ".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
        }
    }
}

/// A buffer that maps to the desired content of the terminal after the draw call
///
/// No widget in the library interacts directly with the terminal. Instead each of them is required
/// to draw their state to an intermediate buffer. It is basically a grid where each cell contains
/// a grapheme, a foreground color and a background color. This grid will then be used to output
/// the appropriate escape sequences and characters to draw the UI as the user has defined it.
///
/// # Examples:
///
/// ```
/// use zellij_widgets::prelude::*;
/// use zellij_widgets::core::style::Color;
///
/// let mut buf = Buffer::empty(Geometry{x: 0, y: 0, cols: 10, rows: 5});
/// buf.get_mut(0, 2).set_symbol("x");
/// assert_eq!(buf.get(0, 2).symbol(), "x");
///
/// buf.set_string(3, 0, "string", Style::default().fg(Color::Red).bg(Color::White));
/// let cell = buf.get_mut(5, 0);
/// assert_eq!(cell.symbol(), "r");
/// assert_eq!(cell.fg, Color::Red);
/// assert_eq!(cell.bg, Color::White);
///
/// buf.get_mut(5, 0).set_char('x');
/// assert_eq!(buf.get(5, 0).symbol(), "x");
/// ```
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Buffer {
    /// The area represented by this buffer
    pub area: Geometry,
    /// The content of the buffer. The length of this Vec should always be equal to area.cols *
    /// area.rows
    pub content: Vec<Cell>,
}

impl Buffer {
    /// Returns a Buffer with all cells set to the default one
    pub fn empty(area: Geometry) -> Buffer {
        let cell = Cell::default();
        Buffer::filled(area, &cell)
    }

    /// Returns a Buffer with all cells initialized with the attributes of the given Cell
    pub fn filled(area: Geometry, cell: &Cell) -> Buffer {
        let size = area.area() as usize;
        let mut content = Vec::with_capacity(size);
        for _ in 0..size {
            content.push(cell.clone());
        }
        Buffer { area, content }
    }

    /// Returns a Buffer containing the given lines
    pub fn with_lines<'a, S>(lines: Vec<S>) -> Buffer
    where
        S: Into<Line<'a>>,
    {
        let lines = lines.into_iter().map(Into::into).collect::<Vec<_>>();
        let rows = lines.len() as u16;
        let cols = lines.iter().map(Line::width).max().unwrap_or_default() as u16;
        let mut buffer = Buffer::empty(Geometry::new(rows, cols));
        for (y, line) in lines.iter().enumerate() {
            buffer.set_line(0, y as u16, line, cols);
        }
        buffer
    }

    /// Returns the content of the buffer as a slice
    pub fn content(&self) -> &[Cell] {
        &self.content
    }

    /// Returns the area covered by this buffer
    pub fn area(&self) -> &Geometry {
        &self.area
    }

    /// Returns a reference to Cell at the given coordinates
    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let i = self.index_of(x, y);
        &self.content[i]
    }

    /// Returns a mutable reference to Cell at the given coordinates
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let i = self.index_of(x, y);
        &mut self.content[i]
    }

    /// Returns the index in the `Vec<Cell>` for the given global (x, y) coordinates.
    ///
    /// Global coordinates are offset by the Buffer's area offset (`x`/`y`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// let rect = Geometry::new(10, 10);
    /// let buffer = Buffer::empty(rect);
    /// // Global coordinates to the top corner of this buffer's area
    /// assert_eq!(buffer.index_of(0, 0), 0);
    /// ```
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        debug_assert!(
            x >= self.area.left()
                && x < self.area.right()
                && y >= self.area.top()
                && y < self.area.bottom(),
            "Trying to access position outside the buffer: x={x}, y={y}, area={:?}",
            self.area
        );
        ((y - self.area.y) * self.area.cols + (x - self.area.x)) as usize
    }

    /// Returns the (global) coordinates of a cell given its index
    ///
    /// Global coordinates are offset by the Buffer's area offset (`x`/`y`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// let rect = Geometry::new(10, 10);
    /// let buffer = Buffer::empty(rect);
    /// assert_eq!(buffer.pos_of(0), (0, 0));
    /// assert_eq!(buffer.pos_of(14), (4, 1));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when given an index that is outside the Buffer's content.
    ///
    /// ```should_panic
    /// # use zellij_widgets::prelude::*;
    /// let rect = Geometry::new(10, 10); // 100 cells in total
    /// let buffer = Buffer::empty(rect);
    /// // Index 100 is the 101th cell, which lies outside of the area of this Buffer.
    /// buffer.pos_of(100); // Panics
    /// ```
    pub fn pos_of(&self, i: usize) -> (u16, u16) {
        debug_assert!(
            i < self.content.len(),
            "Trying to get the coords of a cell outside the buffer: i={i} len={}",
            self.content.len()
        );
        (
            self.area.x + (i as u16) % self.area.cols,
            self.area.y + (i as u16) / self.area.cols,
        )
    }

    /// Print a string, starting at the position (x, y)
    pub fn set_string<S>(&mut self, x: u16, y: u16, string: S, style: Style)
    where
        S: AsRef<str>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    /// Print at most the first n characters of a string if enough space is available
    /// until the end of the line
    pub fn set_stringn<S>(
        &mut self,
        x: u16,
        y: u16,
        string: S,
        cols: usize,
        style: Style,
    ) -> (u16, u16)
    where
        S: AsRef<str>,
    {
        let mut index = self.index_of(x, y);
        let mut x_offset = x as usize;
        let graphemes = UnicodeSegmentation::graphemes(string.as_ref(), true);
        let max_offset = min(self.area.right() as usize, cols.saturating_add(x as usize));
        for s in graphemes {
            let cols = s.width();
            if cols == 0 {
                continue;
            }
            // `x_offset + cols > max_offset` could be integer overflow on 32-bit machines if we
            // change dimensions to usize or u32 and someone resizes the terminal to 1x2^32.
            if cols > max_offset.saturating_sub(x_offset) {
                break;
            }

            self.content[index].set_symbol(s);
            self.content[index].set_style(style);
            // Reset following cells if multi-cols (they would be hidden by the grapheme),
            for i in index + 1..index + cols {
                self.content[i].reset();
            }
            index += cols;
            x_offset += cols;
        }
        (x_offset as u16, y)
    }

    pub fn set_line(&mut self, x: u16, y: u16, line: &Line<'_>, cols: u16) -> (u16, u16) {
        let mut remaining_cols = cols;
        let mut x = x;
        for span in &line.spans {
            if remaining_cols == 0 {
                break;
            }
            let pos = self.set_stringn(
                x,
                y,
                span.content.as_ref(),
                remaining_cols as usize,
                span.style,
            );
            let w = pos.0.saturating_sub(x);
            x = pos.0;
            remaining_cols = remaining_cols.saturating_sub(w);
        }
        (x, y)
    }

    pub fn set_span(&mut self, x: u16, y: u16, span: &Span<'_>, cols: u16) -> (u16, u16) {
        self.set_stringn(x, y, span.content.as_ref(), cols as usize, span.style)
    }

    pub fn set_style(&mut self, area: Geometry, style: Style) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                self.get_mut(x, y).set_style(style);
            }
        }
    }

    /// Resize the buffer so that the mapped area matches the given area and that the buffer
    /// length is equal to area.cols * area.rows
    pub fn resize(&mut self, area: Geometry) {
        let length = area.area() as usize;
        if self.content.len() > length {
            self.content.truncate(length);
        } else {
            self.content.resize(length, Cell::default());
        }
        self.area = area;
    }

    /// Reset all cells in the buffer
    pub fn reset(&mut self) {
        for c in &mut self.content {
            c.reset();
        }
    }
}

impl Debug for Buffer {
    /// Writes a debug representation of the buffer to the given formatter.
    ///
    /// The format is like a pretty printed struct, with the following fields:
    /// * `area`: displayed as `Geometry { x: 1, y: 2, cols: 3, rows: 4 }`
    /// * `content`: displayed as a list of strings representing the content of the buffer
    /// * `styles`: displayed as a list of: `{ x: 1, y: 2, fg: Color::Red, bg: Color::Blue,
    ///   modifier: Modifier::BOLD }` only showing a value when there is a change in style.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "Buffer {{\n    area: {:?},\n    content: [\n",
            &self.area
        ))?;
        let mut last_style = None;
        let mut styles = vec![];
        for (y, line) in self.content.chunks(self.area.cols as usize).enumerate() {
            let mut overwritten = vec![];
            let mut skip: usize = 0;
            f.write_str("        \"")?;
            for (x, c) in line.iter().enumerate() {
                if skip == 0 {
                    f.write_str(c.symbol())?;
                } else {
                    overwritten.push((x, c.symbol()));
                }
                skip = std::cmp::max(skip, c.symbol().width()).saturating_sub(1);
                {
                    let style = (c.fg, c.bg, c.modifier);
                    if last_style != Some(style) {
                        last_style = Some(style);
                        styles.push((x, y, c.fg, c.bg, c.modifier));
                    }
                }
            }
            if !overwritten.is_empty() {
                f.write_fmt(format_args!(
                    "// hidden by multi-cols symbols: {overwritten:?}"
                ))?;
            }
            f.write_str("\",\n")?;
        }
        f.write_str("    ],\n    styles: [\n")?;
        for s in styles {
            f.write_fmt(format_args!(
                "        x: {}, y: {}, fg: {:?}, bg: {:?}, modifier: {:?},\n",
                s.0, s.1, s.2, s.3, s.4
            ))?;
        }
        f.write_str("    ]\n}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_buffer_eq;

    fn cell(s: &str) -> Cell {
        let mut cell = Cell::default();
        cell.set_symbol(s);
        cell
    }

    #[test]
    fn it_implements_debug() {
        let mut buf = Buffer::empty(Geometry::new(2, 12));
        buf.set_string(0, 0, "Hello World!", Style::default());
        buf.set_string(
            0,
            1,
            "G'day World!",
            Style::default()
                .fg(Color::Green)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
        assert_eq!(
            format!("{buf:?}"),
            indoc::indoc!(
                "
                Buffer {
                    area: Geometry { x: 0, y: 0, rows: 2, cols: 12 },
                    content: [
                        \"Hello World!\",
                        \"G'day World!\",
                    ],
                    styles: [
                        x: 0, y: 0, fg: Reset, bg: Reset, modifier: NONE,
                        x: 0, y: 1, fg: Green, bg: Yellow, modifier: BOLD,
                    ]
                }"
            )
        );
    }

    #[test]
    fn it_translates_to_and_from_coordinates() {
        let rect = Geometry::new(50, 80);
        let buf = Buffer::empty(rect);

        // First cell is at the upper left corner.
        assert_eq!(buf.pos_of(0), (0, 0));
        assert_eq!(buf.index_of(0, 0), 0);

        // Last cell is in the lower right.
        assert_eq!(buf.pos_of(buf.content.len() - 1), (79, 49));
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn pos_of_panics_on_out_of_bounds() {
        let rect = Geometry::new(10, 10);
        let buf = Buffer::empty(rect);

        // There are a total of 100 cells; zero-indexed means that 100 would be the 101st cell.
        buf.pos_of(100);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn index_of_panics_on_out_of_bounds() {
        let rect = Geometry::new(10, 10);
        let buf = Buffer::empty(rect);

        // cols is 10; zero-indexed means that 10 would be the 11th cell.
        buf.index_of(10, 0);
    }

    #[test]
    fn cell_symbol_field() {
        let mut cell = Cell::default();
        assert_eq!(cell.symbol(), " ");
        cell.set_symbol("あ"); // Multi-byte character
        assert_eq!(cell.symbol(), "あ");
        cell.set_symbol("👨‍👩‍👧‍👦"); // Multiple code units combined with ZWJ
        assert_eq!(cell.symbol(), "👨‍👩‍👧‍👦");
    }

    #[test]
    fn with_lines_accepts_into_lines() {
        use crate::styles::Stylize;
        let mut buf = Buffer::empty(Geometry::new(2, 3));
        buf.set_string(0, 0, "foo", Style::new().red());
        buf.set_string(0, 1, "bar", Style::new().blue());
        assert_eq!(buf, Buffer::with_lines(vec!["foo".red(), "bar".blue()]));
    }

    #[test]
    fn buffer_set_string_multi_width_overwrite() {
        let area = Geometry::new(1, 5);
        let mut buffer = Buffer::empty(area);

        // multi-width overwrite
        buffer.set_string(0, 0, "aaaaa", Style::default());
        buffer.set_string(0, 0, "称号", Style::default());
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["称号a"]));
    }

    #[test]
    fn buffer_set_string_zero_width() {
        let area = Geometry::new(1, 1);
        let mut buffer = Buffer::empty(area);

        // Leading grapheme with zero width
        let s = "\u{1}a";
        buffer.set_stringn(0, 0, s, 1, Style::default());
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["a"]));

        // Trailing grapheme with zero with
        let s = "a\u{1}";
        buffer.set_stringn(0, 0, s, 1, Style::default());
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["a"]));
    }

    #[test]
    fn buffer_set_string_double_width() {
        let area = Geometry::new(1, 5);
        let mut buffer = Buffer::empty(area);
        buffer.set_string(0, 0, "コン", Style::default());
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["コン "]));

        // Only 1 space left.
        buffer.set_string(0, 0, "コンピ", Style::default());
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["コン "]));
    }

    #[test]
    fn buffer_with_lines() {
        let buffer =
            Buffer::with_lines(vec!["┌────────┐", "│コンピュ│", "│ーa 上で│", "└────────┘"]);
        assert_eq!(buffer.area.x, 0);
        assert_eq!(buffer.area.y, 0);
        assert_eq!(buffer.area.cols, 10);
        assert_eq!(buffer.area.rows, 4);
    }
}
