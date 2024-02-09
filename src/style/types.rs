pub use self::{attribute::Attribute, color::Color, colored::Colored};

mod attribute;
mod color;
mod colored;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colors {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl Colors {
    /// Returns a new `Color` which, when applied, has the same effect as applying `self` and *then*
    /// `other`.
    pub fn then(&self, other: &Colors) -> Colors {
        Colors {
            foreground: other.foreground.or(self.foreground),
            background: other.background.or(self.background),
        }
    }
}

impl Colors {
    pub fn new(foreground: Color, background: Color) -> Colors {
        Colors {
            foreground: Some(foreground),
            background: Some(background),
        }
    }
}

impl From<Colored> for Colors {
    fn from(colored: Colored) -> Colors {
        match colored {
            Colored::ForegroundColor(color) => Colors {
                foreground: Some(color),
                background: None,
            },
            Colored::BackgroundColor(color) => Colors {
                foreground: None,
                background: Some(color),
            },
            Colored::UnderlineColor(color) => Colors {
                foreground: None,
                background: Some(color),
            },
        }
    }
}
