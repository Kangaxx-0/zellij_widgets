use crate::prelude::*;

#[derive(Debug, PartialEq, Hash)]
pub struct HighlightItem<'a> {
    symbol: &'a str,
    style: Style,
    icon: Option<&'a str>,
}

impl Default for HighlightItem<'_> {
    fn default() -> Self {
        Self {
            symbol: "-❯",
            style: Style::default(),
            icon: None,
        }
    }
}

impl<'a> HighlightItem<'a> {
    pub fn new(symbol: &'a str, style: Style, icon: Option<&'a str>) -> Self {
        Self {
            symbol,
            style,
            icon,
        }
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = symbol;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_item() {
        let highlight_item = HighlightItem::default();
        assert_eq!(highlight_item.symbol, "-❯");
        assert_eq!(highlight_item.style, Style::default());
        assert_eq!(highlight_item.icon, None);

        let highlight_item = HighlightItem::new(">", Style::default(), Some("*"));
        assert_eq!(highlight_item.symbol, ">");
        assert_eq!(highlight_item.style, Style::default());
        assert_eq!(highlight_item.icon, Some("*"));

        let highlight_item = HighlightItem::default().icon("icon");
        assert_eq!(highlight_item.icon, Some("icon"));

        let highlight_item = HighlightItem::default().style(Style::default().fg(Color::Red));
        assert_eq!(highlight_item.style, Style::default().fg(Color::Red));

        let highlight_item = HighlightItem::default().symbol(">");
        assert_eq!(highlight_item.symbol, ">");
    }
}
