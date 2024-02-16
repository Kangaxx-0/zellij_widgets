//! This module contains commands that are used to control the terminal with ANSI escape codes, it
//! is pub(crate) and should not be used directly.
//!
//! It contains commands for setting colors, attributes, and printing text.

use std::fmt::{self, Debug, Display};

use crate::{core::command::Command, csi};

pub use super::{
    attributes::Attributes,
    types::{Attribute, Color, Colored},
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_foreground_color() {
        let command = SetForegroundColor(Color::Red);
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[38;5;9m");
    }

    #[test]
    fn test_set_background_color() {
        let command = SetBackgroundColor(Color::Blue);
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[48;5;12m");
    }

    #[test]
    fn test_set_underline_color() {
        let command = SetUnderlineColor(Color::Green);
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[58;5;10m");
    }

    #[test]
    fn test_set_attribute() {
        let command = SetAttribute(Attribute::Bold);
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[1m");
    }

    #[test]
    fn test_reset_color() {
        let command = ResetColor;
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        assert_eq!(output, "\x1b[0m");
    }

    #[test]
    fn test_print() {
        let command = Print("Hello, world!");
        let mut output = String::new();
        command.write_ansi(&mut output).unwrap();
        assert_eq!(output, "Hello, world!");
    }
}
