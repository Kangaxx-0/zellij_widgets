#![allow(unused_imports)]

//! Elements related to the `Block` base widget.
//!
//! This holds everything needed to display and configure a [`Block`].

use crate::{
    buffer::Buffer,
    layout::{Alignment, Geometry},
    style::{symbols::border, Style, Styled},
    title::{Position, Title},
    widget::Widget,
};

pub use border_option::BorderOptions;
pub use border_type::BorderType;
pub use borders::Borders;
pub use padding::Padding;

mod border_option;
mod border_type;
mod borders;
mod padding;

/// Base widget to be used to display a box border around all [`Widget`]
///
/// The borders can be configured with [`Block::borders`] and others. A block can have multiple
/// [`Title`] using [`Block::title`]. It can also be [styled](Block::style) and
/// [padded](Block::padding).
///
/// # Examples
///
/// ```
/// # use zellij_widgets::prelude::*;
///
/// Block::default()
///     .title("Block")
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .border_type(BorderType::Rounded)
///     .style(Style::default().bg(Color::Black));
/// ```
///
/// You may also use multiple titles like in the following:
/// ```
/// use zellij_widgets::prelude::*;
///
/// Block::default()
///     .title("Title 1")
///     .title(Title::from("Title 2").position(Position::Bottom));
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Block<'a> {
    /// List of titles
    titles: Vec<Title<'a>>,
    /// The style to be patched to all titles of the block
    titles_style: Style,
    /// The default alignment of the titles that don't have one
    titles_alignment: Alignment,
    /// The default position of the titles that don't have one
    titles_position: Position,

    /// Border options
    border_option: BorderOptions,

    /// Widget style
    style: Style,
    /// Block padding
    padding: Padding,
}

impl<'a> Block<'a> {
    /// Creates a new block with no [`Borders`] or [`Padding`].
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            titles_style: Style::new(),
            titles_alignment: Alignment::Left,
            titles_position: Position::Top,
            border_option: BorderOptions::default(),
            style: Style::new(),
            padding: Padding::zero(),
        }
    }

    /// Adds a title to the block.
    ///
    /// The `title` function allows you to add a title to the block. You can call this function
    /// multiple times to add multiple titles.
    ///
    /// Each title will be rendered with a single space separating titles that are in the same
    /// position or alignment. When both centered and non-centered titles are rendered, the centered
    /// space is calculated based on the full cols of the block, rather than the leftover width.
    ///
    /// You can provide any type that can be converted into [`Title`] including: strings, string
    /// slices (`&str`), borrowed strings (`Cow<str>`), [spans](crate::text::Span), or vectors of
    /// [spans](crate::text::Span) (`Vec<Span>`).
    ///
    /// By default, the titles will avoid being rendered in the corners of the block but will align
    /// against the left or right edge of the block if there is no border on that edge.  
    /// The following demonstrates this behavior, notice the second title is one character off to
    /// the left.
    ///
    /// ```plain
    /// ┌With at least a left border───
    ///
    /// Without left border───
    /// ```
    ///
    /// Note: If the block is too small and multiple titles overlap, the border might get cut off at
    /// a corner.
    ///
    /// # Example
    ///
    /// The following example demonstrates:
    /// - Default title alignment
    /// - Multiple titles (notice "Center" is centered according to the full with of the block, not
    /// the leftover space)
    /// - Two titles with the same alignment (notice the left titles are separated)
    /// ```
    /// use zellij_widgets::prelude::*;
    ///
    /// Block::default()
    ///     .title("Title") // By default in the top left corner
    ///     .title(Title::from("Left").alignment(Alignment::Left)) // also on the left
    ///     .title(Title::from("Right").alignment(Alignment::Right))
    ///     .title(Title::from("Center").alignment(Alignment::Center));
    /// // Renders
    /// // ┌Title─Left────Center─────────Right┐
    /// ```
    ///
    /// # See also
    ///
    /// Titles attached to a block can have default behaviors. See
    /// - [`Block::title_style`]
    /// - [`Block::title_alignment`]
    /// - [`Block::title_position`]
    pub fn title<T>(mut self, title: T) -> Block<'a>
    where
        T: Into<Title<'a>>,
    {
        self.titles.push(title.into());
        self
    }

    /// Applies the style to all titles.
    ///
    /// If a [`Title`] already has a style, the title's style will add on top of this one.
    pub const fn title_style(mut self, style: Style) -> Block<'a> {
        self.titles_style = style;
        self
    }

    /// Sets the default [`Alignment`] for all block titles.
    ///
    /// Titles that explicitly set an [`Alignment`] will ignore this.
    ///
    /// # Example
    ///
    /// This example aligns all titles in the center except the "right" title which explicitly sets
    /// [`Alignment::Right`].
    /// ```
    /// use zellij_widgets::prelude::*;
    ///
    /// Block::default()
    ///     // This title won't be aligned in the center
    ///     .title(Title::from("right").alignment(Alignment::Right))
    ///     .title("foo")
    ///     .title("bar")
    ///     .title_alignment(Alignment::Center);
    /// ```
    pub const fn title_alignment(mut self, alignment: Alignment) -> Block<'a> {
        self.titles_alignment = alignment;
        self
    }

    /// Sets the default [`Position`] for all block [titles](Title).
    ///
    /// Titles that explicitly set a [`Position`] will ignore this.
    ///
    /// # Example
    ///
    /// This example positions all titles on the bottom except the "top" title which explicitly sets
    /// [`Position::Top`].
    /// ```
    /// use zellij_widgets::prelude::*;
    ///
    /// Block::default()
    ///     // This title won't be aligned in the center
    ///     .title(Title::from("top").position(Position::Top))
    ///     .title("foo")
    ///     .title("bar")
    ///     .title_position(Position::Bottom);
    /// ```
    pub const fn title_position(mut self, position: Position) -> Block<'a> {
        self.titles_position = position;
        self
    }

    /// Defines the options of the borders.
    ///
    /// # Example
    ///
    /// This example shows a `Block` with blue borders.
    /// ```
    /// use zellij_widgets::prelude::*;
    /// let mut border_option = BorderOptions::default();
    /// border_option.set_border_style(Style::new().blue());
    /// Block::default().border_option(border_option);
    /// ```
    pub fn border_option(mut self, border_option: BorderOptions) -> Block<'a> {
        self.border_option = border_option;
        self
    }

    /// Defines the options with the given borders, border style and border set.
    pub fn border_option_with_attrs(
        mut self,
        borders: Borders,
        border_style: Style,
        border_set: border::Set,
    ) -> Block<'a> {
        self.border_option = BorderOptions::new(borders, border_style, border_set);
        self
    }

    /// Defines the style of the borders.
    ///
    /// If a [`Block::style`] is defined, `border_style` will be applied on top of it.
    ///
    /// # Example
    ///
    /// This example shows a `Block` with blue borders.
    /// ```
    /// use zellij_widgets::prelude::*;
    /// Block::default()
    ///     .borders(Borders::ALL)
    ///     .border_style(Style::new().blue());
    /// ```
    pub fn border_style(mut self, style: Style) -> Block<'a> {
        self.border_option.set_border_style(style);
        self
    }

    /// Defines the block style.
    ///
    /// This is the most generic [`Style`] a block can receive, it will be merged with any other
    /// more specific style. Elements can be styled further with [`Block::title_style`] and
    /// [`Block::border_style`].
    ///
    /// This will also apply to the widget inside that block, unless the inner widget is styled.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Block<'a> {
        self.style = style.into();
        self
    }

    /// Defines which borders to display.
    ///
    /// [`Borders`] can also be styled with [`Block::border_style`] and [`Block::border_type`].
    ///
    /// # Examples
    ///
    /// Simply show all borders.
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// Block::default().borders(Borders::ALL);
    /// ```
    ///
    /// Display left and right borders.
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// Block::default().borders(Borders::LEFT | Borders::RIGHT);
    /// ```
    pub fn borders(mut self, flag: Borders) -> Block<'a> {
        self.border_option.set_borders(flag);
        self
    }

    /// Sets the symbols used to display the border (e.g. single line, double line, thick or
    /// rounded borders).
    ///
    /// Setting this overwrites any custom [`border_set`](Block::border_set) that was set.
    ///
    /// See [`BorderType`] for the full list of available symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// Block::default().title("Block").borders(Borders::ALL).border_type(BorderType::Rounded);
    /// // Renders
    /// // ╭Block╮
    /// // │     │
    /// // ╰─────╯
    /// ```
    pub fn border_type(mut self, border_type: BorderType) -> Block<'a> {
        let set = border_type.to_border_set();
        self.border_option.set_border_set(set);
        self
    }

    /// Sets the symbols used to display the border as a [`crate::style::symbols::border::Set`].
    ///
    /// Setting this overwrites any [`border_type`](Block::border_type) that was set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// Block::default().title("Block").borders(Borders::ALL).border_set(symbols::border::DOUBLE);
    /// // Renders
    /// // ╔Block╗
    /// // ║     ║
    /// // ╚═════╝
    pub fn border_set(mut self, border_set: border::Set) -> Block<'a> {
        self.border_option.set_border_set(border_set);
        self
    }

    /// Compute the inner area of a block based on its border visibility rules.
    ///
    /// # Examples
    ///
    /// Draw a block nested within another block
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// # fn render_nested_block(frame: &mut Frame) {
    /// let outer_block = Block::default().title("Outer").borders(Borders::ALL);
    /// let inner_block = Block::default().title("Inner").borders(Borders::ALL);
    ///
    /// let outer_area = frame.size();
    /// let inner_area = outer_block.inner(outer_area);
    ///
    /// frame.render_widget(outer_block, outer_area);
    /// frame.render_widget(inner_block, inner_area);
    /// # }
    /// // Renders
    /// // ┌Outer────────┐
    /// // │┌Inner──────┐│
    /// // ││           ││
    /// // │└───────────┘│
    /// // └─────────────┘
    /// ```
    pub fn inner(&self, area: Geometry) -> Geometry {
        let mut inner = area;
        let border = self.border_option.borders;
        if border.intersects(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1).min(inner.right());
            inner.cols = inner.cols.saturating_sub(1);
        }
        if border.intersects(Borders::TOP) || !self.titles.is_empty() {
            inner.y = inner.y.saturating_add(1).min(inner.bottom());
            inner.rows = inner.rows.saturating_sub(1);
        }
        if border.intersects(Borders::RIGHT) {
            inner.cols = inner.cols.saturating_sub(1);
        }
        if border.intersects(Borders::BOTTOM) {
            inner.rows = inner.rows.saturating_sub(1);
        }

        inner.x = inner.x.saturating_add(self.padding.left);
        inner.y = inner.y.saturating_add(self.padding.top);

        inner.cols = inner
            .cols
            .saturating_sub(self.padding.left + self.padding.right);
        inner.rows = inner
            .rows
            .saturating_sub(self.padding.top + self.padding.bottom);

        inner
    }

    /// Defines the padding inside a `Block`.
    ///
    /// See [`Padding`] for more information.
    ///
    /// # Examples
    ///
    /// This renders a `Block` with no padding (the default).
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// Block::default()
    ///     .borders(Borders::ALL)
    ///     .padding(Padding::zero());
    /// // Renders
    /// // ┌───────┐
    /// // │content│
    /// // └───────┘
    /// ```
    ///
    /// This example shows a `Block` with padding left and right ([`Padding::horizontal`]).
    /// Notice the two spaces before and after the content.
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// Block::default()
    ///     .borders(Borders::ALL)
    ///     .padding(Padding::horizontal(2));
    /// // Renders
    /// // ┌───────────┐
    /// // │  content  │
    /// // └───────────┘
    /// ```
    pub const fn padding(mut self, padding: Padding) -> Block<'a> {
        self.padding = padding;
        self
    }

    fn render_borders(&self, area: Geometry, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let BorderOptions {
            borders,
            border_style,
            border_set: symbols,
        } = self.border_option;

        // Sides
        if borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                    .set_symbol(symbols.vertical_left)
                    .set_style(border_style);
            }
        }
        if borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                    .set_symbol(symbols.horizontal_top)
                    .set_style(border_style);
            }
        }
        if borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.vertical_right)
                    .set_style(border_style);
            }
        }
        if borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols.horizontal_bottom)
                    .set_style(border_style);
            }
        }

        // Corners
        if borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
                .set_symbol(symbols.bottom_right)
                .set_style(border_style);
        }
        if borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
                .set_symbol(symbols.top_right)
                .set_style(border_style);
        }
        if borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
                .set_symbol(symbols.bottom_left)
                .set_style(border_style);
        }
        if borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(symbols.top_left)
                .set_style(border_style);
        }
    }

    /* Titles Rendering */
    fn get_title_y(&self, position: Position, area: Geometry) -> u16 {
        match position {
            Position::Bottom => area.bottom() - 1,
            Position::Top => area.top(),
        }
    }

    fn title_filter(&self, title: &Title, alignment: Alignment, position: Position) -> bool {
        title.alignment.unwrap_or(self.titles_alignment) == alignment
            && title.position.unwrap_or(self.titles_position) == position
    }

    fn calculate_title_area_offsets(&self, area: Geometry) -> (u16, u16, u16) {
        let borders = self.border_option.borders;
        let left_border_dx = u16::from(borders.intersects(Borders::LEFT));
        let right_border_dx = u16::from(borders.intersects(Borders::RIGHT));

        let title_area_cols = area
            .cols
            .saturating_sub(left_border_dx)
            .saturating_sub(right_border_dx);

        (left_border_dx, right_border_dx, title_area_cols)
    }

    fn render_left_titles(&self, position: Position, area: Geometry, buf: &mut Buffer) {
        let (left_border_dx, _, title_area_cols) = self.calculate_title_area_offsets(area);

        let mut current_offset = left_border_dx;
        self.titles
            .iter()
            .filter(|title| self.title_filter(title, Alignment::Left, position))
            .for_each(|title| {
                let title_x = current_offset;
                current_offset += title.content.width() as u16 + 1;

                // Clone the title's content, applying block title style then the title style
                let mut content = title.content.clone();
                for span in content.spans.iter_mut() {
                    span.style = self.titles_style.patch(span.style);
                }

                buf.set_line(
                    title_x + area.left(),
                    self.get_title_y(position, area),
                    &content,
                    title_area_cols,
                );
            });
    }

    fn render_center_titles(&self, position: Position, area: Geometry, buf: &mut Buffer) {
        let (_, _, title_area_cols) = self.calculate_title_area_offsets(area);

        let titles = self
            .titles
            .iter()
            .filter(|title| self.title_filter(title, Alignment::Center, position));

        let titles_sum = titles
            .clone()
            .fold(-1, |acc, f| acc + f.content.width() as i16 + 1); // First element isn't spaced

        let mut current_offset = area.cols.saturating_sub(titles_sum as u16) / 2;
        titles.for_each(|title| {
            let title_x = current_offset;
            current_offset += title.content.width() as u16 + 1;

            // Clone the title's content, applying block title style then the title style
            let mut content = title.content.clone();
            for span in content.spans.iter_mut() {
                span.style = self.titles_style.patch(span.style);
            }

            buf.set_line(
                title_x + area.left(),
                self.get_title_y(position, area),
                &content,
                title_area_cols,
            );
        });
    }

    fn render_right_titles(&self, position: Position, area: Geometry, buf: &mut Buffer) {
        let (_, right_border_dx, title_area_cols) = self.calculate_title_area_offsets(area);

        let mut current_offset = right_border_dx;
        self.titles
            .iter()
            .filter(|title| self.title_filter(title, Alignment::Right, position))
            .rev() // so that the titles appear in the order they have been set
            .for_each(|title| {
                current_offset += title.content.width() as u16 + 1;
                let title_x = current_offset - 1; // First element isn't spaced

                // Clone the title's content, applying block title style then the title style
                let mut content = title.content.clone();
                for span in content.spans.iter_mut() {
                    span.style = self.titles_style.patch(span.style);
                }

                buf.set_line(
                    area.cols.saturating_sub(title_x) + area.left(),
                    self.get_title_y(position, area),
                    &content,
                    title_area_cols,
                );
            });
    }

    fn render_title_position(&self, position: Position, area: Geometry, buf: &mut Buffer) {
        // Note: the order in which these functions are called define the overlapping behavior
        self.render_right_titles(position, area, buf);
        self.render_center_titles(position, area, buf);
        self.render_left_titles(position, area, buf);
    }

    fn render_titles(&self, area: Geometry, buf: &mut Buffer) {
        self.render_title_position(Position::Top, area, buf);
        self.render_title_position(Position::Bottom, area, buf);
    }
}

impl<'a> Widget for Block<'a> {
    fn render(self, area: Geometry, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }
        self.render_borders(area, buf);
        self.render_titles(area, buf);
    }
}

impl<'a> Styled for Block<'a> {
    type Item = Block<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        layout::Geometry,
        style::{Color, Modifier, Stylize},
        text::Line,
    };

    #[test]
    fn inner_takes_into_account_the_borders() {
        // No borders
        assert_eq!(
            Block::default().inner(Geometry::default()),
            Geometry::new(0, 0),
            "no borders, cols=0, rows=0"
        );
        assert_eq!(
            Block::default().inner(Geometry::new(1, 1)),
            Geometry::new(1, 1),
            "no borders, cols=1, rows=1"
        );

        // Left border
        assert_eq!(
            Block::default()
                .borders(Borders::LEFT)
                .inner(Geometry::new(10, 10)),
            Geometry {
                x: 1,
                y: 0,
                cols: 9,
                rows: 10,
            },
            "left, cols=0"
        );
        assert_eq!(
            Block::default()
                .borders(Borders::LEFT)
                .inner(Geometry::new(10, 10)),
            Geometry {
                x: 1,
                y: 0,
                cols: 9,
                rows: 10,
            },
            "left, cols=1"
        );
        assert_eq!(
            Block::default().borders(Borders::LEFT).inner(Geometry {
                x: 0,
                y: 0,
                rows: 2,
                cols: 1
            }),
            Geometry {
                x: 1,
                y: 0,
                cols: 0,
                rows: 2,
            },
            "left, cols=2"
        );

        // Top border
        assert_eq!(
            Block::default().borders(Borders::TOP).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 0
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 0,
            },
            "top, rows=0"
        );
        assert_eq!(
            Block::default().borders(Borders::TOP).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 1
            }),
            Geometry {
                x: 0,
                y: 1,
                cols: 1,
                rows: 0,
            },
            "top, rows=1"
        );
        assert_eq!(
            Block::default().borders(Borders::TOP).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 2
            }),
            Geometry {
                x: 0,
                y: 1,
                cols: 1,
                rows: 1,
            },
            "top, rows=2"
        );

        // Right border
        assert_eq!(
            Block::default().borders(Borders::RIGHT).inner(Geometry {
                x: 0,
                y: 0,
                cols: 0,
                rows: 1
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 0,
                rows: 1,
            },
            "right, cols=0"
        );
        assert_eq!(
            Block::default().borders(Borders::RIGHT).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 1
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 0,
                rows: 1,
            },
            "right, cols=1"
        );
        assert_eq!(
            Block::default().borders(Borders::RIGHT).inner(Geometry {
                x: 0,
                y: 0,
                cols: 2,
                rows: 1
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 1
            },
            "right, cols=2"
        );

        // Bottom border
        assert_eq!(
            Block::default().borders(Borders::BOTTOM).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 0
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 0,
            },
            "bottom, rows=0"
        );
        assert_eq!(
            Block::default().borders(Borders::BOTTOM).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 1
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 0,
            },
            "bottom, rows=1"
        );
        assert_eq!(
            Block::default().borders(Borders::BOTTOM).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 2
            }),
            Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 1,
            },
            "bottom, rows=2"
        );

        // All borders
        assert_eq!(
            Block::default()
                .borders(Borders::ALL)
                .inner(Geometry::default()),
            Geometry::new(0, 0),
            "all borders, cols=0, rows=0"
        );
        assert_eq!(
            Block::default().borders(Borders::ALL).inner(Geometry {
                x: 0,
                y: 0,
                cols: 1,
                rows: 1
            }),
            Geometry {
                x: 1,
                y: 1,
                cols: 0,
                rows: 0
            },
            "all borders, cols=1, rows=1"
        );
        assert_eq!(
            Block::default().borders(Borders::ALL).inner(Geometry {
                x: 0,
                y: 0,
                cols: 2,
                rows: 2
            }),
            Geometry {
                x: 1,
                y: 1,
                cols: 0,
                rows: 0
            },
            "all borders, cols=2, rows=2"
        );
        assert_eq!(
            Block::default().borders(Borders::ALL).inner(Geometry {
                x: 0,
                y: 0,
                cols: 3,
                rows: 3
            }),
            Geometry {
                x: 1,
                y: 1,
                cols: 1,
                rows: 1
            },
            "all borders, cols=3, rows=3"
        );
    }
    //
    #[test]
    fn inner_takes_into_account_the_title() {
        assert_eq!(
            Block::default().title("Test").inner(Geometry {
                x: 0,
                y: 0,
                cols: 0,
                rows: 1
            }),
            Geometry {
                x: 0,
                y: 1,
                cols: 0,
                rows: 0
            },
        );
        assert_eq!(
            Block::default()
                .title(Title::from("Test").alignment(Alignment::Center))
                .inner(Geometry {
                    x: 0,
                    y: 0,
                    cols: 0,
                    rows: 1
                }),
            Geometry {
                x: 0,
                y: 1,
                cols: 0,
                rows: 0
            },
        );
        assert_eq!(
            Block::default()
                .title(Title::from("Test").alignment(Alignment::Right))
                .inner(Geometry {
                    x: 0,
                    y: 0,
                    cols: 0,
                    rows: 1
                }),
            Geometry {
                x: 0,
                y: 1,
                cols: 0,
                rows: 0
            },
        );
    }

    #[test]
    fn border_type_can_be_const() {
        const _PLAIN: border::Set = BorderType::border_symbols(BorderType::Plain);
    }

    #[test]
    fn padding_new() {
        assert_eq!(
            Padding::new(1, 2, 3, 4),
            Padding {
                left: 1,
                right: 2,
                top: 3,
                bottom: 4
            }
        )
    }

    #[test]
    fn padding_constructors() {
        assert_eq!(Padding::zero(), Padding::new(0, 0, 0, 0));
        assert_eq!(Padding::horizontal(1), Padding::new(1, 1, 0, 0));
        assert_eq!(Padding::vertical(1), Padding::new(0, 0, 1, 1));
        assert_eq!(Padding::uniform(1), Padding::new(1, 1, 1, 1));
    }

    #[test]
    fn padding_can_be_const() {
        const _PADDING: Padding = Padding::new(1, 1, 1, 1);
        const _UNI_PADDING: Padding = Padding::uniform(1);
        const _NO_PADDING: Padding = Padding::zero();
        const _HORIZONTAL: Padding = Padding::horizontal(1);
        const _VERTICAL: Padding = Padding::vertical(1);
    }

    #[test]
    fn block_new() {
        assert_eq!(
            Block::new(),
            Block {
                titles: Vec::new(),
                titles_style: Style::new(),
                titles_alignment: Alignment::Left,
                titles_position: Position::Top,
                border_option: BorderOptions::default(),
                style: Style::new(),
                padding: Padding::zero(),
            }
        )
    }

    #[test]
    fn can_be_stylized() {
        let block = Block::default().black().on_white().bold().not_dim();
        assert_eq!(
            block.style,
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::DIM)
        )
    }

    #[test]
    fn block_style() {
        // nominal style
        let block = Block::default().style(Style::new().red());
        assert_eq!(block.style, Style::new().red());

        // auto-convert from Color
        let block = Block::default().style(Color::Red);
        assert_eq!(block.style, Style::new().red());

        // auto-convert from (Color, Color)
        let block = Block::default().style((Color::Red, Color::Blue));
        assert_eq!(block.style, Style::new().red().on_blue());

        // auto-convert from Modifier
        let block = Block::default().style(Modifier::BOLD | Modifier::ITALIC);
        assert_eq!(block.style, Style::new().bold().italic());

        // auto-convert from (Color, Modifier)
        let block = Block::default().style((Color::Red, Modifier::BOLD));
        assert_eq!(block.style, Style::new().red().bold());

        // auto-convert from (Color, Color, Modifier)
        let block = Block::default().style((Color::Red, Color::Blue, Modifier::BOLD));
        assert_eq!(block.style, Style::new().red().on_blue().bold());
    }

    #[test]
    fn block_padding() {
        let block = Block::default().padding(Padding::new(1, 2, 3, 4));
        assert_eq!(block.padding, Padding::new(1, 2, 3, 4));
    }

    #[test]
    fn block_borders() {
        let block = Block::default().borders(Borders::ALL);
        assert_eq!(block.border_option.borders, Borders::ALL);
    }

    #[test]
    fn block_border_type() {
        let block = Block::default().border_type(BorderType::Rounded);
        assert_eq!(
            block.border_option.border_set,
            BorderType::Rounded.to_border_set()
        );
    }

    #[test]
    fn title() {
        let block = Block::default().title("Title");
        assert_eq!(block.titles.len(), 1);
        assert_eq!(block.titles[0].content, Line::raw("Title"));
    }

    #[test]
    fn title_style() {
        let block = Block::default()
            .title("Title")
            .title_style(Style::new().red());
        assert_eq!(block.titles_style, Style::new().red());
    }

    #[test]
    fn title_alignment() {
        let block_center = Block::default()
            .title("Title")
            .title_alignment(Alignment::Center);
        assert_eq!(block_center.titles_alignment, Alignment::Center);

        let block_left = Block::default()
            .title("Title")
            .title_alignment(Alignment::Left);
        assert_eq!(block_left.titles_alignment, Alignment::Left);

        let block_right = Block::default()
            .title("Title")
            .title_alignment(Alignment::Right);
        assert_eq!(block_right.titles_alignment, Alignment::Right);
    }
}
