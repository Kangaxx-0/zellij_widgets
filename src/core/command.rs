//! An interface for a command that performs an action on the terminal.
//!

use std::fmt;
use std::io::{self, Write};

pub trait Command {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result;
}

/// An interface for types that can queue commands for further execution.
pub trait QueueableCommand {
    /// Queues the given command for further execution.
    fn queue(&mut self, command: impl Command) -> io::Result<&mut Self>;
}

impl<T: Write + ?Sized> QueueableCommand for T {
    fn queue(&mut self, command: impl Command) -> io::Result<&mut Self> {
        write_command_ansi(self, command)?;
        Ok(self)
    }
}

/// Writes the ANSI representation of a command to the given writer.
///
/// # Under the hood
/// `Adapter` is introduced as a bridge between [`std::fmt::Write`] and [`std::io::Write`].
fn write_command_ansi<C: Command>(
    io: &mut (impl io::Write + ?Sized),
    command: C,
) -> io::Result<()> {
    struct Adapter<T> {
        inner: T,
        res: io::Result<()>,
    }

    impl<T: Write> fmt::Write for Adapter<T> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.inner.write_all(s.as_bytes()).map_err(|e| {
                self.res = Err(e);
                fmt::Error
            })
        }
    }

    let mut adapter = Adapter {
        inner: io,
        res: Ok(()),
    };

    command
        .write_ansi(&mut adapter)
        .map_err(|fmt::Error| match adapter.res {
            Ok(()) => panic!(
                "<{}>::write_ansi incorrectly errored",
                std::any::type_name::<C>()
            ),
            Err(e) => e,
        })
}

/// Executes the ANSI representation of a command, using the given `fmt::Write`.
pub(crate) fn execute_fmt(f: &mut impl fmt::Write, command: impl Command) -> fmt::Result {
    command.write_ansi(f)
}
