use itertools::Itertools;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

use zellij_widgets::prelude::{Style, *};

use component::{Button, ButtonState, BLUE, GREEN, RED};

mod component;

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

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) => self.pressed_key = c,
            _ => {}
        }
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
                "▔".repeat(area.cols as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.rows > 1 {
            buf.set_string(
                area.x,
                area.y + area.rows - 1,
                "▁".repeat(area.cols as usize),
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
        Paragraph::new("←/→: select, Space: toggle, q: quit"),
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
