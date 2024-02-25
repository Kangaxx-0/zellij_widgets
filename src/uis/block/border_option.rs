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
    pub fn new(borders: Borders, border_style: Style, border_set: border::Set) -> Self {
        Self {
            borders,
            border_style,
            border_set,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_options_default() {
        let border_options = BorderOptions::default();
        assert_eq!(border_options.borders, Borders::NONE);
        assert_eq!(border_options.border_style, Style::default());
        assert_eq!(border_options.border_set, BorderType::Plain.to_border_set());
    }

    #[test]
    fn test_border_options_new() {
        let border_options = BorderOptions::new(
            Borders::ALL,
            Style::default(),
            BorderType::Plain.to_border_set(),
        );
        assert_eq!(border_options.borders, Borders::ALL);
        assert_eq!(border_options.border_style, Style::default());
        assert_eq!(border_options.border_set, BorderType::Plain.to_border_set());
    }

    #[test]
    fn test_border_options_set_borders() {
        let mut border_options = BorderOptions::default();
        border_options.set_borders(Borders::ALL);
        assert_eq!(border_options.borders, Borders::ALL);
    }

    #[test]
    fn test_border_options_set_border_style() {
        let mut border_options = BorderOptions::default();
        border_options.set_border_style(Style::default());
        assert_eq!(border_options.border_style, Style::default());
    }

    #[test]
    fn test_border_options_set_border_set() {
        let mut border_options = BorderOptions::default();
        border_options.set_border_set(BorderType::Plain.to_border_set());
        assert_eq!(border_options.border_set, BorderType::Plain.to_border_set());
    }
}
