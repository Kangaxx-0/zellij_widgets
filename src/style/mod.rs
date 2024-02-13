use crate::text::Span;
use std::fmt::{self, Debug, Display};

use crate::{
    core::command::{execute_fmt, Command},
    csi, impl_display,
};

pub use self::{
    attributes::Attributes,
    content_style::ContentStyle,
    modifier::Modifier,
    styled_content::StyledContent,
    stylize::Stylize,
    types::{Attribute, Color, Colored, Colors},
};

mod attributes;
mod content_style;
mod modifier;
mod styled_content;
mod stylize;
mod types;

/// Style lets you control the main characteristics of the displayed elements.
///
/// ```rust
/// use zellij_widgets::prelude::*;
///
/// Style::default()
///     .fg(Color::Black)
///     .bg(Color::Green)
///     .add_modifier(Modifier::ITALIC | Modifier::BOLD);
/// ```
///
/// ```rust
/// # use zellij_widgets::prelude::*;
/// Style::new().black().on_green().italic().bold();
/// ```
///
/// For more information about the style shorthands, see the [`Stylize`] trait.
///
/// Styles represents an incremental change. If you apply the styles S1, S2, S3 to a cell of the
/// terminal buffer, the style of this cell will be the result of the merge of S1, S2 and S3, not
/// just S3.
///
/// ```rust
/// use zellij_widgets::prelude::*;
///
/// let styles = [
///     Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::default().bg(Color::Red).add_modifier(Modifier::UNDERLINED),
///     Style::default().fg(Color::Yellow).remove_modifier(Modifier::ITALIC),
/// ];
/// let mut buffer = Buffer::empty(Geometry::new(10, 10));
/// for style in &styles {
///   buffer.get_mut(0, 0).set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Red),
///         add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
///         sub_modifier: Modifier::empty(),
///     },
///     buffer.get(0, 0).style(),
/// );
/// ```
///
/// The default implementation returns a `Style` that does not modify anything. If you wish to
/// reset all properties until that point use [`Style::reset`].
///
/// ```
/// use zellij_widgets::prelude::*;
///
/// let styles = [
///     Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD | Modifier::ITALIC),
///     Style::reset().fg(Color::Yellow),
/// ];
/// let mut buffer = Buffer::empty(Geometry::new(10, 10));
/// for style in &styles {
///   buffer.get_mut(0, 0).set_style(*style);
/// }
/// assert_eq!(
///     Style {
///         fg: Some(Color::Yellow),
///         bg: Some(Color::Reset),
///         add_modifier: Modifier::empty(),
///         sub_modifier: Modifier::empty(),
///     },
///     buffer.get(0, 0).style(),
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

impl From<Color> for Style {
    /// Creates a new `Style` with the given foreground color.
    fn from(color: Color) -> Style {
        Style::default().fg(color)
    }
}

impl From<(Color, Color)> for Style {
    /// Creates a new `Style` with the given foreground and background colors.
    fn from((fg, bg): (Color, Color)) -> Style {
        Style::default().fg(fg).bg(bg)
    }
}

impl From<Modifier> for Style {
    /// Creates a new `Style` with the given modifier.
    fn from(modifier: Modifier) -> Style {
        Style::default().add_modifier(modifier)
    }
}

impl From<(Color, Modifier)> for Style {
    /// Creates a new `Style` with the given foreground color and modifier.
    fn from((fg, modifier): (Color, Modifier)) -> Style {
        Style::default().fg(fg).add_modifier(modifier)
    }
}

impl From<(Color, Color, Modifier)> for Style {
    /// Creates a new `Style` with the given foreground and background colors and modifier.
    fn from((fg, bg, modifier): (Color, Color, Modifier)) -> Style {
        Style::default().fg(fg).bg(bg).add_modifier(modifier)
    }
}

/// A trait for objects that have a `Style`.
///
/// This trait enables generic code to be written that can interact with any object that has a
/// `Style`. This is used by the `Stylize` trait to allow generic code to be written that can
/// interact with any object that can be styled.
pub trait Styled {
    type Item;

    fn style(&self) -> Style;
    fn set_style(self, style: Style) -> Self::Item;
}

impl<'a> Styled for &'a str {
    type Item = Span<'a>;

    fn style(&self) -> Style {
        Style::default()
    }

    fn set_style(self, style: Style) -> Self::Item {
        Span::styled(self, style)
    }
}

impl Styled for String {
    type Item = Span<'static>;

    fn style(&self) -> Style {
        Style::default()
    }

    fn set_style(self, style: Style) -> Self::Item {
        Span::styled(self, style)
    }
}

impl Styled for Style {
    type Item = Style;

    fn style(&self) -> Style {
        *self
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.patch(style)
    }
}

impl Style {
    pub const fn new() -> Style {
        Style {
            fg: None,
            bg: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    /// Returns a `Style` resetting all properties.
    pub const fn reset() -> Style {
        Style {
            fg: Some(Color::Reset),
            bg: Some(Color::Reset),
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::all(),
        }
    }

    /// Changes the foreground color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    ///
    /// let style = Style::default().fg(Color::Blue);
    /// let diff = Style::default().fg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().fg(Color::Red));
    /// ```
    #[must_use = "`fg` returns the modified style without modifying the original"]
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    ///
    /// let style = Style::default().bg(Color::Blue);
    /// let diff = Style::default().bg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().bg(Color::Red));
    /// ```
    #[must_use = "`bg` returns the modified style without modifying the original"]
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it adds the given modifier to the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    ///
    /// let style = Style::default().add_modifier(Modifier::BOLD);
    /// let diff = Style::default().add_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(patched.sub_modifier, Modifier::empty());
    /// ```
    #[must_use = "`add_modifier` returns the modified style without modifying the original"]
    pub const fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.sub_modifier = self.sub_modifier.difference(modifier);
        self.add_modifier = self.add_modifier.union(modifier);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it removes the given modifier from the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    ///
    /// let style = Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC);
    /// let diff = Style::default().remove_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD);
    /// assert_eq!(patched.sub_modifier, Modifier::ITALIC);
    /// ```
    #[must_use = "`remove_modifier` returns the modified style without modifying the original"]
    pub const fn remove_modifier(mut self, modifier: Modifier) -> Style {
        self.add_modifier = self.add_modifier.difference(modifier);
        self.sub_modifier = self.sub_modifier.union(modifier);
        self
    }

    /// Results in a combined style that is equivalent to applying the two individual styles to
    /// a style one after the other.
    ///
    /// ## Examples
    /// ```
    /// # use zellij_widgets::prelude::*;
    ///
    /// let style_1 = Style::default().fg(Color::Yellow);
    /// let style_2 = Style::default().bg(Color::Red);
    /// let combined = style_1.patch(style_2);
    /// assert_eq!(
    ///     Style::default().patch(style_1).patch(style_2),
    ///     Style::default().patch(combined));
    /// ```
    #[must_use = "`patch` returns the modified style without modifying the original"]
    pub fn patch(mut self, other: Style) -> Style {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        self.add_modifier.remove(other.sub_modifier);
        self.add_modifier.insert(other.add_modifier);
        self.sub_modifier.remove(other.add_modifier);
        self.sub_modifier.insert(other.sub_modifier);

        self
    }
}

/// A command that sets the foreground color with ANSI code directly.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// [`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
/// color in one command.
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetForegroundColor(pub Color);

impl Command for SetForegroundColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::ForegroundColor(self.0))
    }
}

impl Command for &SetForegroundColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::ForegroundColor(self.0))
    }
}

/// A command that sets the background color with ANSI code directly.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// [`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
/// color with one command.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetBackgroundColor(pub Color);

impl Command for SetBackgroundColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::BackgroundColor(self.0))
    }
}

impl Command for &SetBackgroundColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::BackgroundColor(self.0))
    }
}

/// A command that sets the underline color with ANSI code directly.
///
/// See [`Color`](enum.Color.html) for more info.
///
/// [`SetColors`](struct.SetColors.html) can also be used to set both the foreground and background
/// color with one command.
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetUnderlineColor(pub Color);

impl Command for SetUnderlineColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::UnderlineColor(self.0))
    }
}

impl Command for &SetUnderlineColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), Colored::UnderlineColor(self.0))
    }
}

/// A command that sets an attribute with ANSI code directly.
///
/// See [`Attribute`](enum.Attribute.html) for more info.
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAttribute(pub Attribute);

impl Command for SetAttribute {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), self.0.sgr())
    }
}
impl Command for &SetAttribute {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}m"), self.0.sgr())
    }
}

/// A command that sets several attributes.
///
/// See [`Attributes`](struct.Attributes.html) for more info.
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAttributes(pub Attributes);

impl Command for SetAttributes {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for attr in Attribute::iterator() {
            if self.0.has(attr) {
                SetAttribute(attr).write_ansi(f)?;
            }
        }
        Ok(())
    }
}
impl Command for &SetAttributes {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        for attr in Attribute::iterator() {
            if self.0.has(attr) {
                SetAttribute(attr).write_ansi(f)?;
            }
        }
        Ok(())
    }
}

/// A command that sets a style (colors and attributes).
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetStyle(pub ContentStyle);

impl Command for SetStyle {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if let Some(bg) = self.0.background_color {
            execute_fmt(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
        }
        if let Some(fg) = self.0.foreground_color {
            execute_fmt(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
        }
        if let Some(ul) = self.0.underline_color {
            execute_fmt(f, SetUnderlineColor(ul)).map_err(|_| fmt::Error)?;
        }
        if !self.0.attributes.is_empty() {
            execute_fmt(f, SetAttributes(self.0.attributes)).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}
impl Command for &SetStyle {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if let Some(bg) = self.0.background_color {
            execute_fmt(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
        }
        if let Some(fg) = self.0.foreground_color {
            execute_fmt(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
        }
        if let Some(ul) = self.0.underline_color {
            execute_fmt(f, SetUnderlineColor(ul)).map_err(|_| fmt::Error)?;
        }
        if !self.0.attributes.is_empty() {
            execute_fmt(f, SetAttributes(self.0.attributes)).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}

/// A command that prints styled content.
///
/// See [`StyledContent`](struct.StyledContent.html) for more info.
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Copy, Clone)]
pub struct PrintStyledContent<D: Display>(pub StyledContent<D>);

impl<D: Display> Command for PrintStyledContent<D> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let style = self.0.style();

        let mut reset_background = false;
        let mut reset_foreground = false;
        let mut reset = false;

        if let Some(bg) = style.background_color {
            execute_fmt(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
            reset_background = true;
        }
        if let Some(fg) = style.foreground_color {
            execute_fmt(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
            reset_foreground = true;
        }
        if let Some(ul) = style.underline_color {
            execute_fmt(f, SetUnderlineColor(ul)).map_err(|_| fmt::Error)?;
            reset_foreground = true;
        }

        if !style.attributes.is_empty() {
            execute_fmt(f, SetAttributes(style.attributes)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        write!(f, "{}", self.0.content())?;

        if reset {
            // NOTE: This will reset colors even though self has no colors, hence produce unexpected
            // resets.
            // TODO: reset the set attributes only.
            execute_fmt(f, ResetColor).map_err(|_| fmt::Error)?;
        } else {
            // NOTE: Since the above bug, we do not need to reset colors when we reset attributes.
            if reset_background {
                execute_fmt(f, SetBackgroundColor(Color::Reset)).map_err(|_| fmt::Error)?;
            }
            if reset_foreground {
                execute_fmt(f, SetForegroundColor(Color::Reset)).map_err(|_| fmt::Error)?;
            }
        }

        Ok(())
    }
}
impl<D: Display> Command for &PrintStyledContent<D> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let style = self.0.style();

        let mut reset_background = false;
        let mut reset_foreground = false;
        let mut reset = false;

        if let Some(bg) = style.background_color {
            execute_fmt(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
            reset_background = true;
        }
        if let Some(fg) = style.foreground_color {
            execute_fmt(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
            reset_foreground = true;
        }
        if let Some(ul) = style.underline_color {
            execute_fmt(f, SetUnderlineColor(ul)).map_err(|_| fmt::Error)?;
            reset_foreground = true;
        }

        if !style.attributes.is_empty() {
            execute_fmt(f, SetAttributes(style.attributes)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        write!(f, "{}", self.0.content())?;

        if reset {
            // NOTE: This will reset colors even though self has no colors, hence produce unexpected
            // resets.
            // TODO: reset the set attributes only.
            execute_fmt(f, ResetColor).map_err(|_| fmt::Error)?;
        } else {
            // NOTE: Since the above bug, we do not need to reset colors when we reset attributes.
            if reset_background {
                execute_fmt(f, SetBackgroundColor(Color::Reset)).map_err(|_| fmt::Error)?;
            }
            if reset_foreground {
                execute_fmt(f, SetForegroundColor(Color::Reset)).map_err(|_| fmt::Error)?;
            }
        }

        Ok(())
    }
}

/// A command that resets the colors back to default.
///
/// # Notes
///
/// Commands must be queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetColor;

impl Command for ResetColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("0m"))
    }
}
impl Command for &ResetColor {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(csi!("0m"))
    }
}

/// A command that prints the given displayable type.
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Print<T: Display>(pub T);

impl<T: Display> Command for Print<T> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Display> Display for Print<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl_display!(for SetForegroundColor);
impl_display!(for SetBackgroundColor);
impl_display!(for SetAttribute);
impl_display!(for PrintStyledContent<String>);
impl_display!(for PrintStyledContent<&'static str>);
impl_display!(for ResetColor);

/// Utility function for ANSI parsing in Color and Colored.
/// Gets the next element of `iter` and tries to parse it as a `u8`.
pub fn parse_next_u8<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<u8> {
    iter.next().and_then(|s| s.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn styles() -> Vec<Style> {
        vec![
            Style::default(),
            Style::default().fg(Color::Yellow),
            Style::default().bg(Color::Yellow),
            Style::default().add_modifier(Modifier::BOLD),
            Style::default().remove_modifier(Modifier::BOLD),
            Style::default().add_modifier(Modifier::ITALIC),
            Style::default().remove_modifier(Modifier::ITALIC),
            Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD),
            Style::default().remove_modifier(Modifier::ITALIC | Modifier::BOLD),
        ]
    }

    #[test]
    fn combined_patch_gives_same_result_as_individual_patch() {
        let styles = styles();
        for &a in &styles {
            for &b in &styles {
                for &c in &styles {
                    for &d in &styles {
                        let combined = a.patch(b.patch(c.patch(d)));

                        assert_eq!(
                            Style::default().patch(a).patch(b).patch(c).patch(d),
                            Style::default().patch(combined)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn combine_individual_modifiers() {
        use crate::{buffer::Buffer, layout::Geometry};

        let mods = vec![
            Modifier::BOLD,
            Modifier::DIM,
            Modifier::ITALIC,
            Modifier::UNDERLINED,
            Modifier::SLOW_BLINK,
            Modifier::RAPID_BLINK,
            Modifier::REVERSED,
            Modifier::HIDDEN,
            Modifier::CROSSED_OUT,
        ];

        let mut buffer = Buffer::empty(Geometry::new(37, 240));

        for m in &mods {
            println!("try to get mut");
            buffer.get_mut(0, 0).set_style(Style::reset());
            println!("set reset");
            buffer
                .get_mut(0, 0)
                .set_style(Style::default().add_modifier(*m));
            let style = buffer.get(0, 0).style();
            assert!(style.add_modifier.contains(*m));
            assert!(!style.sub_modifier.contains(*m));
        }
    }

    #[test]
    fn modifier_debug() {
        assert_eq!(format!("{:?}", Modifier::empty()), "NONE");
        assert_eq!(format!("{:?}", Modifier::BOLD), "BOLD");
        assert_eq!(format!("{:?}", Modifier::DIM), "DIM");
        assert_eq!(format!("{:?}", Modifier::ITALIC), "ITALIC");
        assert_eq!(format!("{:?}", Modifier::UNDERLINED), "UNDERLINED");
        assert_eq!(format!("{:?}", Modifier::SLOW_BLINK), "SLOW_BLINK");
        assert_eq!(format!("{:?}", Modifier::RAPID_BLINK), "RAPID_BLINK");
        assert_eq!(format!("{:?}", Modifier::REVERSED), "REVERSED");
        assert_eq!(format!("{:?}", Modifier::HIDDEN), "HIDDEN");
        assert_eq!(format!("{:?}", Modifier::CROSSED_OUT), "CROSSED_OUT");
        assert_eq!(
            format!("{:?}", Modifier::BOLD | Modifier::DIM),
            "BOLD | DIM"
        );
        assert_eq!(
            format!("{:?}", Modifier::all()),
            "BOLD | DIM | ITALIC | UNDERLINED | SLOW_BLINK | RAPID_BLINK | REVERSED | HIDDEN | CROSSED_OUT"
        );
    }

    #[test]
    fn style_can_be_const() {
        const RED: Color = Color::Red;
        const BLACK: Color = Color::Black;
        const BOLD: Modifier = Modifier::BOLD;
        const ITALIC: Modifier = Modifier::ITALIC;

        const _RESET: Style = Style::reset();
        const _RED_FG: Style = Style::new().fg(RED);
        const _BLACK_BG: Style = Style::new().bg(BLACK);
        const _ADD_BOLD: Style = Style::new().add_modifier(BOLD);
        const _REMOVE_ITALIC: Style = Style::new().remove_modifier(ITALIC);
        const ALL: Style = Style::new()
            .fg(RED)
            .bg(BLACK)
            .add_modifier(BOLD)
            .remove_modifier(ITALIC);
        assert_eq!(
            ALL,
            Style::new()
                .fg(Color::Red)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD)
                .remove_modifier(Modifier::ITALIC)
        )
    }

    #[test]
    fn style_can_be_stylized() {
        // foreground colors
        assert_eq!(Style::new().black(), Style::new().fg(Color::Black));
        assert_eq!(Style::new().red(), Style::new().fg(Color::Red));
        assert_eq!(Style::new().green(), Style::new().fg(Color::Green));
        assert_eq!(Style::new().yellow(), Style::new().fg(Color::Yellow));
        assert_eq!(Style::new().blue(), Style::new().fg(Color::Blue));
        assert_eq!(Style::new().magenta(), Style::new().fg(Color::Magenta));
        assert_eq!(Style::new().cyan(), Style::new().fg(Color::Cyan));
        assert_eq!(Style::new().white(), Style::new().fg(Color::White));
        assert_eq!(Style::new().gray(), Style::new().fg(Color::Gray));
        assert_eq!(Style::new().dark_gray(), Style::new().fg(Color::DarkGray));
        assert_eq!(Style::new().white(), Style::new().fg(Color::White));

        // Background colors
        assert_eq!(Style::new().on_black(), Style::new().bg(Color::Black));
        assert_eq!(Style::new().on_red(), Style::new().bg(Color::Red));
        assert_eq!(Style::new().on_green(), Style::new().bg(Color::Green));
        assert_eq!(Style::new().on_yellow(), Style::new().bg(Color::Yellow));
        assert_eq!(Style::new().on_blue(), Style::new().bg(Color::Blue));
        assert_eq!(Style::new().on_magenta(), Style::new().bg(Color::Magenta));
        assert_eq!(Style::new().on_cyan(), Style::new().bg(Color::Cyan));
        assert_eq!(Style::new().on_white(), Style::new().bg(Color::White));
        assert_eq!(Style::new().on_gray(), Style::new().bg(Color::Gray));
        assert_eq!(
            Style::new().on_dark_gray(),
            Style::new().bg(Color::DarkGray)
        );
        assert_eq!(Style::new().on_white(), Style::new().bg(Color::White));

        // Add Modifiers
        assert_eq!(
            Style::new().bold(),
            Style::new().add_modifier(Modifier::BOLD)
        );
        assert_eq!(Style::new().dim(), Style::new().add_modifier(Modifier::DIM));
        assert_eq!(
            Style::new().italic(),
            Style::new().add_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::new().underlined(),
            Style::new().add_modifier(Modifier::UNDERLINED)
        );
        assert_eq!(
            Style::new().slow_blink(),
            Style::new().add_modifier(Modifier::SLOW_BLINK)
        );
        assert_eq!(
            Style::new().rapid_blink(),
            Style::new().add_modifier(Modifier::RAPID_BLINK)
        );
        assert_eq!(
            Style::new().reversed(),
            Style::new().add_modifier(Modifier::REVERSED)
        );
        assert_eq!(
            Style::new().hidden(),
            Style::new().add_modifier(Modifier::HIDDEN)
        );
        assert_eq!(
            Style::new().crossed_out(),
            Style::new().add_modifier(Modifier::CROSSED_OUT)
        );

        // Remove Modifiers
        assert_eq!(
            Style::new().not_bold(),
            Style::new().remove_modifier(Modifier::BOLD)
        );
        assert_eq!(
            Style::new().not_dim(),
            Style::new().remove_modifier(Modifier::DIM)
        );
        assert_eq!(
            Style::new().not_italic(),
            Style::new().remove_modifier(Modifier::ITALIC)
        );
        assert_eq!(
            Style::new().not_underlined(),
            Style::new().remove_modifier(Modifier::UNDERLINED)
        );
        assert_eq!(
            Style::new().not_slow_blink(),
            Style::new().remove_modifier(Modifier::SLOW_BLINK)
        );
        assert_eq!(
            Style::new().not_rapid_blink(),
            Style::new().remove_modifier(Modifier::RAPID_BLINK)
        );
        assert_eq!(
            Style::new().not_reversed(),
            Style::new().remove_modifier(Modifier::REVERSED)
        );
        assert_eq!(
            Style::new().not_hidden(),
            Style::new().remove_modifier(Modifier::HIDDEN)
        );
        assert_eq!(
            Style::new().not_crossed_out(),
            Style::new().remove_modifier(Modifier::CROSSED_OUT)
        );

        // reset
        assert_eq!(Style::new().reset(), Style::reset());
    }
}
