use std::fmt;
use strum::EnumMessage;

use crate::prelude::*;

/// HighlightSymbol is an enum that represents the different symbols that can be used to highlight
///
/// - SingleArrow: ->,
/// - DoubleArrow: =>,
/// - Checkmark: ✓,
/// - Asterisk: *,
/// - Custom: Custom(String) where String is the custom symbol, e.g. "-->"
///
/// The default value is SingleArrow
///
/// # Example
/// ``` rust
/// use zellij_widgets::prelude::*;
/// let highlight_symbol_single = HighlightSymbol::SingleArrow;
/// assert_eq!(format!("{}", highlight_symbol_single), String::from("->"));
/// ```
#[derive(Default, EnumMessage, Debug, PartialEq, Hash)]
pub enum HighlightSymbol {
    #[strum(message = "->")]
    #[default]
    SingleArrow,
    #[strum(message = "=>")]
    DoubleArrow,
    #[strum(message = "✓")]
    Checkmark,
    #[strum(message = "*")]
    Asterisk,
    Custom(String),
}

/// Implement Display for HighlightSymbol, this is mainly for `Custom` variant
///
/// Example:
/// ``` rust
/// use zellij_widgets::prelude::*;
/// let highlight_symbol_custom = HighlightSymbol::Custom("--->".to_string());
/// assert_eq!(format!("{}", highlight_symbol_custom), String::from("--->"));
/// ```
impl fmt::Display for HighlightSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightSymbol::Custom(s) => write!(f, "{}", s),
            _ => write!(
                f,
                "{}",
                self.get_message().expect("Invalid HighlightSymbol")
            ),
        }
    }
}

impl HighlightSymbol {
    /// Return the length of the symbol
    pub fn len(&self) -> usize {
        match self {
            HighlightSymbol::SingleArrow => 2,
            HighlightSymbol::DoubleArrow => 2,
            HighlightSymbol::Checkmark => 1,
            HighlightSymbol::Asterisk => 1,
            HighlightSymbol::Custom(s) => s.len(),
        }
    }

    /// Convenient method to check if the symbol is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// HighlightStyle is a struct that represents the style of the highlight
///
/// - symbol: HighlightSymbol,
/// - style: Style,
///
/// Example:
/// ``` rust
/// use zellij_widgets::prelude::*;
/// let highlight_item = HighlightStyle::default();
/// assert_eq!(highlight_item.symbol, HighlightSymbol::SingleArrow);
/// assert_eq!(highlight_item.style, Style::default());
/// ```
#[derive(Default, Debug, PartialEq, Hash)]
pub struct HighlightStyle {
    pub symbol: HighlightSymbol,
    pub style: Style,
}

impl HighlightStyle {
    pub fn new(symbol: HighlightSymbol, style: Style) -> Self {
        Self { symbol, style }
    }

    /// Set the style of the highlight
    ///
    /// Example:
    /// ``` rust
    /// use zellij_widgets::prelude::*;
    /// let highlight_item = HighlightStyle::default().style(Style::default().fg(Color::Red));
    /// assert_eq!(highlight_item.style, Style::default().fg(Color::Red));
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the symbol of the highlight
    ///
    /// Example:
    /// ``` rust
    /// use zellij_widgets::prelude::*;
    /// let highlight_item = HighlightStyle::default().symbol(HighlightSymbol::Asterisk);
    /// assert_eq!(highlight_item.symbol, HighlightSymbol::Asterisk);
    /// ```
    pub fn symbol(mut self, symbol: HighlightSymbol) -> Self {
        self.symbol = symbol;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_symbol() {
        let highlight_symbol_single = HighlightSymbol::SingleArrow;
        assert_eq!(format!("{}", highlight_symbol_single), String::from("->"));

        let highlight_symbol_double = HighlightSymbol::DoubleArrow;
        assert_eq!(format!("{}", highlight_symbol_double), String::from("=>"));

        let highlight_symbol_checkmark = HighlightSymbol::Checkmark;
        assert_eq!(format!("{}", highlight_symbol_checkmark), String::from("✓"));

        let highlight_symbol_asterisk = HighlightSymbol::Asterisk;
        assert_eq!(format!("{}", highlight_symbol_asterisk), String::from("*"));

        let highlight_symbol_custom = HighlightSymbol::Custom("--->".to_string());
        assert_eq!(format!("{}", highlight_symbol_custom), String::from("--->"));
    }

    #[test]
    fn test_highlight_symbol_len() {
        let highlight_symbol_single = HighlightSymbol::SingleArrow;
        assert_eq!(highlight_symbol_single.len(), 2);

        let highlight_symbol_double = HighlightSymbol::DoubleArrow;
        assert_eq!(highlight_symbol_double.len(), 2);

        let highlight_symbol_checkmark = HighlightSymbol::Checkmark;
        assert_eq!(highlight_symbol_checkmark.len(), 1);

        let highlight_symbol_asterisk = HighlightSymbol::Asterisk;
        assert_eq!(highlight_symbol_asterisk.len(), 1);

        let highlight_symbol_custom = HighlightSymbol::Custom("--->".to_string());
        assert_eq!(highlight_symbol_custom.len(), 4);
    }

    #[test]
    fn test_highlight_item() {
        let highlight_item = HighlightStyle::default();
        assert_eq!(highlight_item.symbol, HighlightSymbol::SingleArrow);
        assert_eq!(highlight_item.style, Style::default());

        let highlight_item = HighlightStyle::new(HighlightSymbol::DoubleArrow, Style::default());
        assert_eq!(highlight_item.symbol, HighlightSymbol::DoubleArrow);
        assert_eq!(highlight_item.style, Style::default());

        let highlight_item = HighlightStyle::default().style(Style::default().fg(Color::Red));
        assert_eq!(highlight_item.style, Style::default().fg(Color::Red));

        let highlight_item =
            HighlightStyle::default().symbol(HighlightSymbol::Custom("--->".to_string()));
        assert_eq!(
            highlight_item.symbol,
            HighlightSymbol::Custom("--->".to_string())
        );
    }
}
