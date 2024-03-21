use crate::prelude::*;
use strum::Display;

#[derive(Default, Display, Debug, PartialEq, Hash)]
pub enum HighlightSymbol {
    #[strum(serialize = "->")]
    #[default]
    SingleArrow,
    #[strum(serialize = "=>")]
    DoubleArrow,
    #[strum(serialize = "✓")]
    Checkmark,
    #[strum(serialize = "*")]
    Asterisk,
    Custom(String),
}

impl HighlightSymbol {
    pub fn len(&self) -> usize {
        match self {
            HighlightSymbol::SingleArrow => 2,
            HighlightSymbol::DoubleArrow => 2,
            HighlightSymbol::Checkmark => 1,
            HighlightSymbol::Asterisk => 1,
            HighlightSymbol::Custom(s) => s.len(),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct HighlightStyle {
    pub symbol: HighlightSymbol,
    pub style: Style,
    pub icon: Option<String>,
}

impl Default for HighlightStyle {
    fn default() -> Self {
        Self {
            symbol: HighlightSymbol::default(),
            style: Style::default(),
            icon: None,
        }
    }
}

impl HighlightStyle {
    pub fn new(symbol: HighlightSymbol, style: Style, icon: Option<String>) -> Self {
        Self {
            symbol,
            style,
            icon,
        }
    }

    pub fn icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn symbol(mut self, symbol: HighlightSymbol) -> Self {
        self.symbol = symbol;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_item() {
        let highlight_item = HighlightStyle::default();
        assert_eq!(highlight_item.symbol, HighlightSymbol::SingleArrow);
        assert_eq!(highlight_item.style, Style::default());
        assert_eq!(highlight_item.icon, None);

        let highlight_item = HighlightStyle::new(
            HighlightSymbol::DoubleArrow,
            Style::default(),
            Some("*".to_string()),
        );
        assert_eq!(highlight_item.symbol, HighlightSymbol::DoubleArrow);
        assert_eq!(highlight_item.style, Style::default());
        assert_eq!(highlight_item.icon, Some("*".to_string()));

        let highlight_item = HighlightStyle::default().icon("icon".to_string());
        assert_eq!(highlight_item.icon, Some("icon".to_string()));

        let highlight_item = HighlightStyle::default().style(Style::default().fg(Color::Red));
        assert_eq!(highlight_item.style, Style::default().fg(Color::Red));

        let highlight_item = HighlightStyle::default().symbol(HighlightSymbol::Asterisk);
        assert_eq!(highlight_item.symbol, HighlightSymbol::Asterisk);
    }
}
