use crate::prelude::*;

#[derive(Debug, PartialEq, Hash)]
pub struct HighlightStyle {
    pub symbol: String,
    pub style: Style,
    pub icon: Option<String>,
}

impl Default for HighlightStyle {
    fn default() -> Self {
        Self {
            symbol: "-❯".to_string(),
            style: Style::default(),
            icon: None,
        }
    }
}

impl HighlightStyle {
    pub fn new(symbol: String, style: Style, icon: Option<String>) -> Self {
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

    pub fn symbol(mut self, symbol: String) -> Self {
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
        assert_eq!(highlight_item.symbol, "-❯");
        assert_eq!(highlight_item.style, Style::default());
        assert_eq!(highlight_item.icon, None);

        let highlight_item =
            HighlightStyle::new(">".to_string(), Style::default(), Some("*".to_string()));
        assert_eq!(highlight_item.symbol, ">");
        assert_eq!(highlight_item.style, Style::default());
        assert_eq!(highlight_item.icon, Some("*".to_string()));

        let highlight_item = HighlightStyle::default().icon("icon".to_string());
        assert_eq!(highlight_item.icon, Some("icon".to_string()));

        let highlight_item = HighlightStyle::default().style(Style::default().fg(Color::Red));
        assert_eq!(highlight_item.style, Style::default().fg(Color::Red));

        let highlight_item = HighlightStyle::default().symbol(">".to_string());
        assert_eq!(highlight_item.symbol, ">");
    }
}
