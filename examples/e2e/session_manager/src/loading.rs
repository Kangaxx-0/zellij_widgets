use zellij_widgets::prelude::*;

pub struct LoadingDialog<'a> {
    block: Option<Block<'a>>,
    style: Style,
    label: String,
    label_style: Style,
}

impl<'a> LoadingDialog<'a> {
    pub fn new(label: String) -> Self {
        LoadingDialog {
            block: None,
            style: Style::default(),
            label,
            label_style: Style::default(),
        }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_label_style(mut self, label_style: Style) -> Self {
        self.label_style = label_style;
        self
    }
}

impl<'a> Widget for LoadingDialog<'a> {
    fn render(mut self, area: Geometry, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        // calculate the center of the inner_area
        let x = text_area.x + text_area.cols / 2 - self.label.len() as u16 / 2;
        let y = text_area.y + text_area.rows / 2;
        buf.set_string(x, y, self.label, self.label_style);
        buf.set_style(text_area, self.style);
    }
}
