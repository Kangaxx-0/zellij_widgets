use unicode_width::UnicodeWidthStr;

use super::{
    reflow::{LineComposer, LineTruncator, WordWrapper},
    Block,
};

use crate::prelude::*;

fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Center => (text_area_width / 2).saturating_sub(line_width / 2),
        Alignment::Right => text_area_width.saturating_sub(line_width),
        Alignment::Left => 0,
    }
}

/// A widget to display some text.
///
/// # Example
///
/// ```
/// use zellij_widgets::{prelude::*};
///
/// let text = vec![
///     Line::from(vec![
///         Span::raw("First"),
///         Span::styled("line",Style::new().green().italic()),
///         ".".into(),
///     ]),
///     Line::from("Second line".red()),
///     "Third line".into(),
/// ];
/// Paragraph::new(text)
///     .block(Block::new()
///         .title("Paragraph")
///         .borders(Borders::ALL))
///     .style(Style::new().white().on_black())
///     .alignment(Alignment::Center)
///     .wrap(Wrap { trim: true });
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Paragraph<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// How to wrap the text
    wrap: Option<Wrap>,
    /// The text to display
    text: Text<'a>,
    /// Scroll
    scroll: (u16, u16),
    /// Alignment of the text
    alignment: Alignment,
}

/// Describes how to wrap text across lines.
///
/// ## Examples
///
/// ```
/// use zellij_widgets::{prelude::*};
///
/// let bullet_points = Text::from(r#"Some indented points:
///     - First thing goes here and is long so that it wraps
///     - Here is another point that is long enough to wrap"#);
///
/// // With leading spaces trimmed (window width of 30 chars):
/// Paragraph::new(bullet_points.clone()).wrap(Wrap { trim: true });
/// // Some indented points:
/// // - First thing goes here and is
/// // long so that it wraps
/// // - Here is another point that
/// // is long enough to wrap
///
/// // But without trimming, indentation is preserved:
/// Paragraph::new(bullet_points).wrap(Wrap { trim: false });
/// // Some indented points:
/// //     - First thing goes here
/// // and is long so that it wraps
/// //     - Here is another point
/// // that is long enough to wrap
/// ```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Wrap {
    /// Should leading whitespace be trimmed
    pub trim: bool,
}

type Horizontal = u16;
type Vertical = u16;

impl<'a> Paragraph<'a> {
    /// Creates a new [`Paragraph`] widget with the given text.
    ///
    /// The `text` parameter can be a [`Text`] or any type that can be converted into a [`Text`]. By
    /// default, the text is styled with [`Style::default()`], not wrapped, and aligned to the left.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zellij_widgets::{prelude::*};
    ///
    /// let paragraph = Paragraph::new("Hello, world!");
    /// let paragraph = Paragraph::new(String::from("Hello, world!"));
    /// let paragraph = Paragraph::new(Text::raw("Hello, world!"));
    /// let paragraph = Paragraph::new(
    ///     Text::styled("Hello, world!", Style::default()));
    /// let paragraph = Paragraph::new(
    ///     Line::from(vec!["Hello, ".into(), "world!".red()]));
    /// ```
    pub fn new<T>(text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
    {
        Paragraph {
            block: None,
            style: Style::default(),
            wrap: None,
            text: text.into(),
            scroll: (0, 0),
            alignment: Alignment::Left,
        }
    }

    /// Surrounds the [`Paragraph`] widget with a [`Block`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use zellij_widgets::{prelude::*};
    /// let paragraph = Paragraph::new("Hello, world!")
    ///    .block(Block::default()
    ///         .title("Paragraph")
    ///         .borders(Borders::ALL));
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Paragraph<'a> {
        self.block = Some(block);
        self
    }

    /// Sets the style of the entire widget.
    ///
    /// This applies to the entire widget, including the block if one is present. Any style set on
    /// the block or text will be added to this style.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let paragraph = Paragraph::new("Hello, world!")
    ///    .style(Style::new().red().on_white());
    /// ```
    pub fn style(mut self, style: Style) -> Paragraph<'a> {
        self.style = style;
        self
    }

    /// Sets the wrapping configuration for the widget.
    ///
    /// See [`Wrap`] for more information on the different options.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use zellij_widgets::{prelude::*};
    ///
    /// let paragraph = Paragraph::new("Hello, world!")
    ///   .wrap(Wrap { trim: true });
    /// ```
    pub fn wrap(mut self, wrap: Wrap) -> Paragraph<'a> {
        self.wrap = Some(wrap);
        self
    }

    /// Set the scroll offset for the given paragraph
    ///
    /// The scroll offset is a tuple of (y, x) offset. The y offset is the number of lines to
    /// scroll, and the x offset is the number of characters to scroll. The scroll offset is applied
    /// after the text is wrapped and aligned.
    ///
    /// Note: the order of the tuple is (y, x) instead of (x, y), which is different from general
    /// convention across the crate.
    ///
    /// For more information about future scrolling design and concerns, see [RFC: Design of
    /// Scrollable Widgets](https://github.com/widgets-org/widgets/issues/174) on GitHub.
    pub fn scroll(mut self, offset: (Vertical, Horizontal)) -> Paragraph<'a> {
        self.scroll = offset;
        self
    }

    /// Set the text alignment for the given paragraph
    ///
    /// The alignment is a variant of the [`Alignment`] enum which can be one of Left, Right, or
    /// Center.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// # use zellij_widgets::{prelude::*};
    /// let paragraph = Paragraph::new("Hello World")
    ///     .alignment(Alignment::Center);
    /// ```
    pub fn alignment(mut self, alignment: Alignment) -> Paragraph<'a> {
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for Paragraph<'a> {
    fn render(mut self, area: Geometry, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if text_area.rows < 1 {
            return;
        }

        let styled = self.text.lines.iter().map(|line| {
            let graphemes = line
                .spans
                .iter()
                .flat_map(|span| span.styled_graphemes(self.style));
            let alignment = line.alignment.unwrap_or(self.alignment);
            (graphemes, alignment)
        });

        if let Some(Wrap { trim }) = self.wrap {
            let line_composer = WordWrapper::new(styled, text_area.cols, trim);
            self.render_text(line_composer, text_area, buf);
        } else {
            let mut line_composer = LineTruncator::new(styled, text_area.cols);
            line_composer.set_horizontal_offset(self.scroll.1);
            self.render_text(line_composer, text_area, buf);
        }
    }
}

impl<'a> Paragraph<'a> {
    fn render_text<C: LineComposer<'a>>(&self, mut composer: C, area: Geometry, buf: &mut Buffer) {
        let mut y = 0;
        while let Some((current_line, current_line_width, current_line_alignment)) =
            composer.next_line()
        {
            if y >= self.scroll.0 {
                let mut x = get_line_offset(current_line_width, area.cols, current_line_alignment);
                for StyledGrapheme { symbol, style } in current_line {
                    let width = symbol.width();
                    if width == 0 {
                        continue;
                    }
                    // If the symbol is empty, the last char which rendered last time will
                    // leave on the line. It's a quick fix.
                    let symbol = if symbol.is_empty() { " " } else { symbol };
                    buf.get_mut(area.left() + x, area.top() + y - self.scroll.0)
                        .set_symbol(symbol)
                        .set_style(*style);
                    x += width as u16;
                }
            }
            y += 1;
            if y >= area.rows + self.scroll.0 {
                break;
            }
        }
    }
}

impl<'a> Styled for Paragraph<'a> {
    type Item = Paragraph<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}
