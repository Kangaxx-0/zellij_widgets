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
    pub fn new(highlight_index: Option<usize>, start_pos: usize) -> Self {
        self::ListState {
            highlight_index,
            start_pos_to_display: start_pos,
        }
    }

    pub fn set_highlight_index(&mut self, index: usize) {
        self.highlight_index = Some(index);
    }

    pub fn highlight_index(&self) -> Option<usize> {
        self.highlight_index
    }

    pub fn set_start_position(&mut self, start_pos: usize) {
        self.start_pos_to_display = start_pos;
    }

    pub fn start_position(&self) -> usize {
        self.start_pos_to_display
    }
}
