use itertools::Itertools;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::*;

use zellij_widgets::prelude::Style;

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    pressed_key: char,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::SessionUpdate,
            EventType::Key,
            EventType::ModeUpdate,
        ]);
        self.is_loading = true;
    }

    fn update(&mut self, event: Event) -> bool {
        matches!(event, Event::Key(c) if {
            self.handle_key(c);
            true
        })
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // setup terminal
        let mut stdout = std::io::stdout();

        // setup terminal pane
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);
        let mut selected_button: usize = 0;
        let button_states = &[
            ButtonState::Selected,
            ButtonState::Normal,
            ButtonState::Normal,
        ];

        // draw the UI

        match self.pressed_key {
            'c' => {
                // no loop for testing
                let _ = pane.draw(|frame| ui(frame, button_states));
            }
            _ => {}
        }
    }
}

// fn ui(frame: &mut Frame) {
//     let (title_area, layout) = calculate_layout(frame.size());
//
//     render_title(frame, title_area);
//
//     //Render blocks
//     render_borders(frame, Borders::ALL, layout[0][0]);
//     render_borders(frame, Borders::NONE, layout[0][1]);
//     render_borders(frame, Borders::LEFT, layout[1][0]);
//     render_borders(frame, Borders::RIGHT, layout[1][1]);
//     render_borders(frame, Borders::TOP, layout[2][0]);
//     render_borders(frame, Borders::BOTTOM, layout[2][1]);
//
//     render_border_type(frame, BorderType::Plain, layout[3][0]);
//     render_border_type(frame, BorderType::Thick, layout[3][1]);
//     render_border_type(frame, BorderType::Double, layout[4][0]);
//     render_border_type(frame, BorderType::Rounded, layout[4][1]);
// }

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) => self.pressed_key = c,
            _ => {}
        }
    }
}

/// A custom widget that renders a button with a label, theme and state.
#[derive(Debug, Clone)]
struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

const BLUE: Theme = Theme {
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

const RED: Theme = Theme {
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

const GREEN: Theme = Theme {
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

impl<'a> Widget for Button<'a> {
    fn render(self, area: Geometry, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.colors();
        buf.set_style(area, Style::new().bg(background).fg(text));

        // render top line if there's enough space
        if area.rows > 2 {
            buf.set_string(
                area.x,
                area.y,
                "╬ô├╗├╢".repeat(area.cols as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.rows > 1 {
            buf.set_string(
                area.x,
                area.y + area.rows - 1,
                "╬ô├╗├╝".repeat(area.cols as usize),
                Style::new().fg(shadow).bg(background),
            );
        }
        // render label centered
        buf.set_line(
            area.x + (area.cols.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.rows.saturating_sub(1)) / 2,
            &self.label,
            area.cols,
        );
    }
}

impl Button<'_> {
    fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            ButtonState::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
            ButtonState::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
            ButtonState::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
        }
    }
}

// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     let mut selected_button: usize = 0;
//     let button_states = &mut [State::Selected, State::Normal, State::Normal];
//     loop {
//         terminal.draw(|frame| ui(frame, button_states))?;
//         if !event::poll(Duration::from_millis(100))? {
//             continue;
//         }
//         match event::read()? {
//             Event::Key(key) => {
//                 if key.kind != event::KeyEventKind::Press {
//                     continue;
//                 }
//                 if handle_key_event(key, button_states, &mut selected_button).is_break() {
//                     break;
//                 }
//             }
//             Event::Mouse(mouse) => handle_mouse_event(mouse, button_states, &mut selected_button),
//             _ => (),
//         }
//     }
//     Ok(())
// }

fn ui(frame: &mut Frame, states: &[ButtonState; 3]) {
    let layout = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Max(3),
            Constraint::Length(1),
            Constraint::Min(0), // ignore remaining space
        ])
        .split(frame.size());
    frame.render_widget(
        Paragraph::new("Custom Widget Example (mouse enabled)"),
        layout[0],
    );
    render_buttons(frame, layout[1], states);
    frame.render_widget(
        Paragraph::new("╬ô├Ñ├ë/╬ô├Ñ├å: select, Space: toggle, q: quit"),
        layout[2],
    );
}

fn render_buttons(frame: &mut Frame<'_>, area: Geometry, states: &[ButtonState; 3]) {
    let layout = Layout::default()
        .direction(Orientation::Horizontal)
        .constraints([
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Min(0), // ignore remaining space
        ])
        .split(area);
    frame.render_widget(Button::new("Red").theme(RED).state(states[0]), layout[0]);
    frame.render_widget(
        Button::new("Green").theme(GREEN).state(states[1]),
        layout[1],
    );
    frame.render_widget(Button::new("Blue").theme(BLUE).state(states[2]), layout[2]);
}

// fn handle_key_event(
//     key: event::KeyEvent,
//     button_states: &mut [State; 3],
//     selected_button: &mut usize,
// ) -> ControlFlow<()> {
//     match key.code {
//         KeyCode::Char('q') => return ControlFlow::Break(()),
//         KeyCode::Left | KeyCode::Char('h') => {
//             button_states[*selected_button] = State::Normal;
//             *selected_button = selected_button.saturating_sub(1);
//             button_states[*selected_button] = State::Selected;
//         }
//         KeyCode::Right | KeyCode::Char('l') => {
//             button_states[*selected_button] = State::Normal;
//             *selected_button = selected_button.saturating_add(1).min(2);
//             button_states[*selected_button] = State::Selected;
//         }
//         KeyCode::Char(' ') => {
//             if button_states[*selected_button] == State::Active {
//                 button_states[*selected_button] = State::Normal;
//             } else {
//                 button_states[*selected_button] = State::Active;
//             }
//         }
//         _ => (),
//     }
//     ControlFlow::Continue(())
// }

// fn handle_mouse_event(
//     mouse: MouseEvent,
//     button_states: &mut [State; 3],
//     selected_button: &mut usize,
// ) {
//     match mouse.kind {
//         MouseEventKind::Moved => {
//             let old_selected_button = *selected_button;
//             *selected_button = match mouse.column {
//                 x if x < 15 => 0,
//                 x if x < 30 => 1,
//                 _ => 2,
//             };
//             if old_selected_button != *selected_button {
//                 if button_states[old_selected_button] != State::Active {
//                     button_states[old_selected_button] = State::Normal;
//                 }
//                 if button_states[*selected_button] != State::Active {
//                     button_states[*selected_button] = State::Selected;
//                 }
//             }
//         }
//         MouseEventKind::Down(MouseButton::Left) => {
//             if button_states[*selected_button] == State::Active {
//                 button_states[*selected_button] = State::Normal;
//             } else {
//                 button_states[*selected_button] = State::Active;
//             }
//         }
//         _ => (),
//     }
// }
