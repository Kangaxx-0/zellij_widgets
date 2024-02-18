use zellij_widgets::prelude::*;

/// A custom widget that renders a button with a label, theme and state.
#[derive(Debug, Clone)]
pub(crate) struct Button<'a> {
    pub label: Line<'a>,
    pub theme: Theme,
    pub state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ButtonState {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Theme {
    pub text: Color,
    pub background: Color,
    pub highlight: Color,
    pub shadow: Color,
}

pub const BLUE: Theme = Theme {
    text: Color::Rgb {
        r: 16,
        g: 48,
        b: 48,
    },
    background: Color::Rgb {
        r: 48,
        g: 144,
        b: 144,
    },
    highlight: Color::Rgb {
        r: 64,
        g: 192,
        b: 192,
    },
    shadow: Color::Rgb {
        r: 32,
        g: 96,
        b: 96,
    },
};

pub const RED: Theme = Theme {
    text: Color::Rgb {
        r: 48,
        g: 16,
        b: 16,
    },

    background: Color::Rgb {
        r: 144,
        g: 48,
        b: 48,
    },
    highlight: Color::Rgb {
        r: 192,
        g: 64,
        b: 64,
    },
    shadow: Color::Rgb {
        r: 96,
        g: 32,
        b: 32,
    },
};

pub const GREEN: Theme = Theme {
    text: Color::Rgb {
        r: 16,
        g: 48,
        b: 16,
    },
    background: Color::Rgb {
        r: 48,
        g: 144,
        b: 48,
    },
    highlight: Color::Rgb {
        r: 64,
        g: 192,
        b: 64,
    },
    shadow: Color::Rgb {
        r: 32,
        g: 96,
        b: 32,
    },
};

/// A button with a label that can be themed.
impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Button<'a> {
        Button {
            label: label.into(),
            theme: BLUE,
            state: ButtonState::Normal,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Button<'a> {
        self.theme = theme;
        self
    }

    pub fn state(mut self, state: ButtonState) -> Button<'a> {
        self.state = state;
        self
    }
}
