use bitflags::bitflags;

use std::fmt;

bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    ///
    /// They are bitflags so they can easily be composed.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use zellij_widgets::prelude::*;
    ///
    /// let m = Modifier::BOLD | Modifier::ITALIC;
    /// ```
    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

/// Implement the `Debug` trait for `Modifier` manually.
///
/// This will avoid printing the empty modifier as 'Borders(0x0)' and instead print it as 'NONE'.
impl fmt::Debug for Modifier {
    /// Format the modifier as `NONE` if the modifier is empty or as a list of flags separated by
    /// `|` otherwise.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        fmt::Debug::fmt(&self.0, f)
    }
}
