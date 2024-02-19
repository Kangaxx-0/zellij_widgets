use crate::{buffer::Buffer, layout::Geometry, widget::Widget};

/// A widget to erase/reset a certain area to allow overdrawing (e.g. for popups).
///
/// # Examples
///
/// ```
/// use zellij_widgets::prelude::*;
///
/// fn draw_on_clear(f: &mut Frame, area: Geometry) {
///     let block = Block::default().title("Block").borders(Borders::ALL);
///     f.render_widget(Erase, area); // <- this will clear/reset the area first
///     f.render_widget(block, area); // now render the block widget
/// }
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Erase;

impl Widget for Erase {
    fn render(self, area: Geometry, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).reset();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_buffer_eq;

    #[test]
    fn render() {
        let mut buf = Buffer::with_lines(vec!["xxxxxxxxxxxxxxx"; 7]);
        let erase = Erase;
        erase.render(
            Geometry {
                x: 1,
                y: 2,
                cols: 3,
                rows: 4,
            },
            &mut buf,
        );
        assert_buffer_eq!(
            buf,
            Buffer::with_lines(vec![
                "xxxxxxxxxxxxxxx",
                "xxxxxxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "x   xxxxxxxxxxx",
                "xxxxxxxxxxxxxxx",
            ])
        );
    }
}
