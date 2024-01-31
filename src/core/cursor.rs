//! # Cursor
//!
//! The `cursor` module provides limited functionality to work with the terminal cursor, you can
//! move the cursor position but you would not be able to see it since zellij plugin does not show cursor as
//! a stylistic choice :)

use std::fmt;

use crate::{core::command::Command, csi, impl_display};

/// A command that moves the terminal cursor to the given position (column, row).
///
/// # Notes
/// * Top left cell is represented as `0,0`.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveTo(pub u16, pub u16);

impl Command for MoveTo {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{};{}H"), self.1 + 1, self.0 + 1)
    }
}

impl Command for &MoveTo {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{};{}H"), self.1 + 1, self.0 + 1)
    }
}

/// A command that moves the terminal cursor down the given number of lines,
/// and moves it to the first column.
///
/// # Notes
/// * This command is 1 based, meaning `MoveToNextLine(1)` moves to the next line.
/// * Most terminals default 0 argument to 1.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToNextLine(pub u16);

impl Command for MoveToNextLine {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}E"), self.0)?;
        Ok(())
    }
}

impl Command for &MoveToNextLine {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}E"), self.0)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor up the given number of lines,
/// and moves it to the first column.
///
/// # Notes
/// * This command is 1 based, meaning `MoveToPreviousLine(1)` moves to the previous line.
/// * Most terminals default 0 argument to 1.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToPreviousLine(pub u16);

impl Command for MoveToPreviousLine {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}F"), self.0)?;
        Ok(())
    }
}

impl Command for &MoveToPreviousLine {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}F"), self.0)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor to the given column on the current row.
///
/// # Notes
/// * This command is 0 based, meaning 0 is the leftmost column.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToColumn(pub u16);

impl Command for MoveToColumn {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}G"), self.0 + 1)?;
        Ok(())
    }
}

impl Command for &MoveToColumn {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}G"), self.0 + 1)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor to the given row on the current column.
///
/// # Notes
/// * This command is 0 based, meaning 0 is the topmost row.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveToRow(pub u16);

impl Command for MoveToRow {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}d"), self.0 + 1)?;
        Ok(())
    }
}

impl Command for &MoveToRow {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}d"), self.0 + 1)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor a given number of rows up.
///
/// # Notes
/// * This command is 1 based, meaning `MoveUp(1)` moves the cursor up one cell.
/// * Most terminals default 0 argument to 1.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveUp(pub u16);

impl Command for MoveUp {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}A"), self.0)?;
        Ok(())
    }
}

impl Command for &MoveUp {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}A"), self.0)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor a given number of columns to the right.
///
/// # Notes
/// * This command is 1 based, meaning `MoveRight(1)` moves the cursor right one cell.
/// * Most terminals default 0 argument to 1.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveRight(pub u16);

impl Command for MoveRight {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}C"), self.0)?;
        Ok(())
    }
}

impl Command for &MoveRight {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}C"), self.0)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor a given number of rows down.
///
/// # Notes
/// * This command is 1 based, meaning `MoveDown(1)` moves the cursor down one cell.
/// * Most terminals default 0 argument to 1.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDown(pub u16);

impl Command for MoveDown {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}B"), self.0)?;
        Ok(())
    }
}

impl Command for &MoveDown {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}B"), self.0)?;
        Ok(())
    }
}

/// A command that moves the terminal cursor a given number of columns to the left.
///
/// # Notes
/// * This command is 1 based, meaning `MoveLeft(1)` moves the cursor left one cell.
/// * Most terminals default 0 argument to 1.
/// * Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLeft(pub u16);

impl Command for MoveLeft {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}D"), self.0)?;
        Ok(())
    }
}

impl Command for &MoveLeft {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{}D"), self.0)?;
        Ok(())
    }
}

/// A command that saves the current terminal cursor position.
///
/// See the [RestorePosition](./struct.RestorePosition.html) command.
///
/// # Notes
///
/// - The cursor position is stored globally.
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavePosition;

impl Command for SavePosition {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str("\x1B7")
    }
}

impl Command for &SavePosition {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str("\x1B7")
    }
}

/// A command that restores the saved terminal cursor position.
///
/// See the [SavePosition](./struct.SavePosition.html) command.
///
/// # Notes
///
/// - The cursor position is stored globally.
/// - Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestorePosition;

impl Command for RestorePosition {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str("\x1B8")
    }
}

impl Command for &RestorePosition {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str("\x1B8")
    }
}

impl_display!(for MoveTo);
impl_display!(for MoveToColumn);
impl_display!(for MoveToRow);
impl_display!(for MoveToNextLine);
impl_display!(for MoveToPreviousLine);
impl_display!(for MoveUp);
impl_display!(for MoveDown);
impl_display!(for MoveLeft);
impl_display!(for MoveRight);
impl_display!(for SavePosition);
impl_display!(for RestorePosition);
