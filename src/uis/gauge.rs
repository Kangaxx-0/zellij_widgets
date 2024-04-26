use core::f64;

use crate::prelude::{Block, Buffer, Color, Geometry, Span, Style, Widget};

/// A widget to display a progress bar.
///
/// A `Gauge` renders a bar filled according to the value given to [`Gauge::percent`] or
/// [`Gauge::ratio`]. The bar width and height are defined by the [`Geometry`] it is
/// [rendered](Widget::render) in.
/// The associated label is always centered horizontally and vertically. If not set with
/// [`Gauge::label`], the label is the percentage of the bar filled.
///
/// Below is how the ratio/label is rendered:
///
///  ┌block─────────────────────────────┐
///  │███████     ratio/label           │
///  └──────────────────────────────────┘
///
/// # Example
///
pub struct Gauge<'a> {
    block: Block<'a>,
    ratio: f64,
    label: Option<Span<'a>>,
    style: Style,
}

impl<'a> Default for Gauge<'a> {
    fn default() -> Self {
        Self {
            block: Default::default(),
            ratio: 0.0,
            label: None,
            style: Style::default(),
        }
    }
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
    /// NOTE
    ///
    /// The ratio should be between 0.0 and 1.0.
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

    /// Sets the percentage of the filled part of the gauge.
    ///
    /// NOTE
    ///
    /// The percentage should be between 0 and 100.
    ///
    /// # Example
    /// ```
    /// use zellij_widgets::prelude::*;
    /// let gauge = Gauge::new(Block::default()).ratio(0.5);
    /// ```
    ///
    #[must_use = "function consumes self and returns a new instance"]
    pub fn percent(mut self, percent: u8) -> Self {
        assert!(
            percent > 0 && percent <= 100,
            "percent should be between 0 and 100"
        );

        self.ratio = f64::from(percent) / 100 as f64;
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

impl<'a> Widget for Gauge<'a> {
    fn render(self, area: Geometry, buf: &mut Buffer) {
        // Render Block
        let inner_area = self.block.inner(area);
        self.block.render(area, buf);

        // Render ratio/label. We begin with the ratio and label position in the inner_area,
        // we know this is because the label is always centered horizontally and vertically
        if let Some(label) = self.label {
            let label_len = label.width();
            // We format the ratio to have one decimal place
            let ratio = format!("{:.1}", self.ratio * 100.0);
            let ratio_len = ratio.len();
            let total_len = label_len + ratio_len;

            let x = inner_area.left() + (inner_area.cols - total_len as u16) / 2;
            let y = inner_area.top() + inner_area.rows / 2;

            // Render the label
            buf.set_span(x, y, &label, inner_area.cols);
            // Render the ratio
            buf.set_span(x - label_len as u16, y, &Span::raw(ratio), inner_area.cols);
        } else {
            // If the label is not set, then we display "xx%" where xx is the ratio
            let ratio = format!("{:.0}%", self.ratio * 100.0);
            let x = inner_area.left() + (inner_area.cols - ratio.len() as u16) / 2;
            let y = inner_area.top() + inner_area.rows / 2;

            buf.set_span(x, y, &Span::raw(ratio), inner_area.cols);
        }

        buf.set_style(inner_area, self.style);
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
