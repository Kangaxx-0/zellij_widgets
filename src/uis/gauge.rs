use crate::prelude::{Span, Style};

use super::Block;

/// A widget to display a progress bar.
///
/// A `Gauge` renders a bar filled according to the value given to [`Gauge::percent`] or
/// [`Gauge::ratio`]. The bar width and height are defined by the [`Geometry`] it is
/// [rendered](Widget::render) in.
/// The associated label is always centered horizontally and vertically. If not set with
/// [`Gauge::label`], the label is the percentage of the bar filled.
///
/// # Example
///
pub struct Gauge<'a> {
    block: Block<'a>,
    ratio: f64,
    label: Option<Span<'a>>,
    style: Style,
}

impl<'a> Gauge<'a> {
    pub fn new(block: Block<'a>) -> Self {
        Self {
            block,
            ratio: 0.0,
            label: None,
            style: Style::default(),
        }
    }

    /// Sets the ratio of the filled part of the gauge.
    ///
    /// Note that the ratio should be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use zellij_widgets::prelude::*;
    /// let gauge = Gauge::new(Block::default()).ratio(0.5);
    /// ```
    ///
    #[must_use = "function consumes self and returns a new instance"]
    pub fn ratio(mut self, ratio: f64) -> Self {
        assert!(
            ratio >= 0.0 && ratio <= 1.0,
            "ratio should be between 0.0 and 1.0"
        );
        self.ratio = ratio;
        self
    }

    /// Sets the label of the gauge. The associated label is always centered horizontally and vertically.
    ///
    /// # Example
    /// ```
    /// use zellij_widgets::prelude::*;
    /// let gauge = Gauge::new(Block::default()).label("progress");
    /// ```
    ///
    #[must_use = "function consumes self and returns a new instance"]
    pub fn label(mut self, label: impl Into<Span<'a>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the style of the gauge.
    ///
    /// # Example
    /// ```
    /// use zellij_widgets::prelude::*;
    /// let gauge = Gauge::new(Block::default()).style(Style::default().fg(Color::Red));
    /// ```
    ///
    #[must_use = "function consumes self and returns a new instance"]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Color;

    use super::*;

    #[test]
    fn gauge_new() {
        let gauge = Gauge::new(Block::default());
        assert_eq!(gauge.ratio, 0.0);
        assert_eq!(gauge.label, None);
        assert_eq!(gauge.style, Style::default());
    }

    #[test]
    fn gauge_ratio() {
        let gauge = Gauge::new(Block::default()).ratio(0.5);
        assert_eq!(gauge.ratio, 0.5);
    }

    #[test]
    #[should_panic]
    fn gauge_ratio_out_of_bounds() {
        let gauge1 = Gauge::new(Block::default()).ratio(2.0);
        assert_eq!(gauge1.ratio, 0.0);

        let gauge2 = Gauge::new(Block::default()).ratio(-1.0);
        assert_eq!(gauge2.ratio, 0.0);
    }

    #[test]
    fn gauge_label() {
        let gauge = Gauge::new(Block::default()).label("label");
        assert_eq!(gauge.label, Some(Span::raw("label")));
    }

    #[test]
    fn gauge_style() {
        let gauge = Gauge::new(Block::default()).style(Style::default().fg(Color::Red));
        assert_eq!(gauge.style, Style::default().fg(Color::Red));
    }
}
