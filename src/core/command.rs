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
/// This call would not panic, but instead return an error if the underlying writer fails.
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
            Ok(()) => io::Error::new(io::ErrorKind::Other, "write_ansi failed"),
            Err(e) => e,
        })
}

/// Executes the ANSI representation of a command, using the given `fmt::Write`.
pub(crate) fn execute_fmt(f: &mut impl fmt::Write, command: impl Command) -> fmt::Result {
    command.write_ansi(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            // Flushing is a no-op here
            Ok(())
        }
    }

    struct MockCommand;

    impl Command for MockCommand {
        fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
            write!(f, "MockCommand")?;
            Ok(())
        }
    }

    #[test]
    fn test_queueable_command() {
        let mut writer = MockWriter::new();
        let command = MockCommand;
        let result = writer.queue(command);

        assert!(result.is_ok());
        assert_eq!(writer.buffer, b"MockCommand");
    }

    #[test]
    fn test_execute_fmt() {
        let mut writer = MockWriter::new();
        let command = MockCommand;
        let result = execute_fmt(&mut writer, command);

        assert!(result.is_ok());
        assert_eq!(writer.buffer, b"MockCommand");
    }

    // A command that always fails
    struct FailingCommand;

    impl Command for FailingCommand {
        fn write_ansi(&self, _f: &mut impl fmt::Write) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    #[test]
    fn test_failing_command() {
        let mut writer = MockWriter::new();
        let command = FailingCommand;
        let result = writer.queue(command);

        assert!(result.is_err());
    }
}
