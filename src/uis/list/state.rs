#[derive(Debug, Default, PartialEq, Hash)]
pub struct ListState {
    /// The index of the currently selected / highlighted item index
    pub highlight_index: Option<usize>,

    /// Sets the index of the first item to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    pub start_pos_to_display: usize,
}

impl ListState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn highlight_index(mut self, index: Option<usize>) -> Self {
        self.highlight_index = index;
        self
    }

    pub fn set_highlight_index(&self) -> Option<usize> {
        self.highlight_index
    }

    pub fn set_start_pos(mut self, start_pos: usize) -> Self {
        self.start_pos_to_display = start_pos;
        self
    }

    pub fn start_pos_to_display(&self) -> usize {
        self.start_pos_to_display
    }
}
