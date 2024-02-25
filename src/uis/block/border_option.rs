use crate::prelude::symbols::border;

use super::{BorderType, Borders, Style};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BorderOptions {
    /// Visible borders
    pub borders: Borders,
    /// Border style
    pub border_style: Style,
    /// The symbols used to render the border
    pub border_set: border::Set,
}

impl Default for BorderOptions {
    fn default() -> Self {
        Self {
            borders: Borders::NONE,
            border_style: Style::default(),
            border_set: BorderType::Plain.to_border_set(),
        }
    }
}

impl BorderOptions {
    pub fn set_borders(&mut self, borders: Borders) {
        self.borders = borders;
    }

    pub fn set_border_style(&mut self, border_style: Style) {
        self.border_style = border_style;
    }

    pub fn set_border_set(&mut self, border_set: border::Set) {
        self.border_set = border_set;
    }
}
