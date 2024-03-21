#[derive(Debug, Default, PartialEq, Hash)]
pub struct ListState {
    /// The index of the currently selected / highlighted item index
    highlight_index: Option<usize>,

    /// Sets the index of the first item to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    start_pos_to_display: usize,
}

impl ListState {
    /// Create a new list state
    ///
    /// Example:
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let list_state = ListState::new(None, 0);
    /// ```
    pub fn new(highlight_index: Option<usize>, start_pos: usize) -> Self {
        self::ListState {
            highlight_index,
            start_pos_to_display: start_pos,
        }
    }

    /// Set the index of the currently selected / highlighted item index
    ///
    /// Example:
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let mut list_state = ListState::new(None, 0);
    /// list_state.set_highlight_index(5);
    /// assert_eq!(list_state.highlight_index(), Some(5));
    /// ```
    pub fn set_highlight_index(&mut self, index: usize) {
        self.highlight_index = Some(index);
    }

    /// Get the index of the currently selected / highlighted item index
    ///
    /// Example:
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let list_state = ListState::new(Some(0), 0);
    /// assert_eq!(list_state.highlight_index(), Some(0));
    /// ```
    pub fn highlight_index(&self) -> Option<usize> {
        self.highlight_index
    }

    /// Set the index of the first item to be displayed
    ///
    /// Example:
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let mut list_state = ListState::new(None, 0);
    /// list_state.set_start_position(5);
    /// assert_eq!(list_state.start_position(), 5);
    /// ```
    pub fn set_start_position(&mut self, start_pos: usize) {
        self.start_pos_to_display = start_pos;
    }

    /// Get the index of the first item to be displayed
    ///
    /// Example:
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let list_state = ListState::new(None, 0);
    /// assert_eq!(list_state.start_position(), 0);
    /// ```
    pub fn start_position(&self) -> usize {
        self.start_pos_to_display
    }
}
