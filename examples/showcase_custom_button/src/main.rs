use std::collections::BTreeMap;

use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Style, *};

use component::{Button, ButtonState, BLUE, GREEN, ORANGE, PURPLE, RED};

mod component;

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    pressed_key: char,
    buttons: Vec<ButtonState>,
    selected_button: usize,
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
        self.buttons = vec![
            ButtonState::Selected,
            ButtonState::Normal,
            ButtonState::Normal,
            ButtonState::Normal,
            ButtonState::Normal,
        ];
        self.selected_button = 0;
    }

    fn update(&mut self, event: Event) -> bool {
        matches!(event, Event::Key(c) if {
            self.handle_key(c);
            true
        })
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // setup terminal
        let stdout = std::io::stdout();

        // setup terminal pane
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);

        // draw the UI
        if self.pressed_key == 'c' {
            // no loop for testing
            let _ = pane.draw(|frame| ui(frame, self.selected_button, &self.buttons));
        }
    }
}

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) if c == 'c' => self.pressed_key = c,
            Key::Char('q') => hide_self(),
            Key::Right => {
                self.buttons[self.selected_button] = ButtonState::Normal;
                self.selected_button = self.selected_button.saturating_add(1) % self.buttons.len();
                self.buttons[self.selected_button] = ButtonState::Selected;
            }
            Key::Left => {
                self.buttons[self.selected_button] = ButtonState::Normal;
                self.selected_button =
                    (self.selected_button.saturating_sub(1) + self.buttons.len() - 1)
                        % self.buttons.len();
                self.buttons[self.selected_button] = ButtonState::Selected;
            }
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
        }
    }
}

fn ui(frame: &mut Frame, index: usize, states: &[ButtonState]) {
    let layout = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Max(5),
            Constraint::Ratio(1, 3),
            Constraint::Length(1),
            Constraint::Min(0), // ignore remaining space
        ])
        .split(frame.size());
    frame.render_widget(
        Paragraph::new("Custom Widget Example (mouse enabled)"),
        layout[0],
    );
    render_buttons(frame, layout[1], states);
    render_block(frame, layout[2], index);
    frame.render_widget(
        Paragraph::new("←/→: select, Space: toggle, q: quit"),
        layout[3],
    );
}

fn render_block(frame: &mut Frame<'_>, area: Geometry, index: usize) {
    let block = Block::default()
        .title("Block")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let text = format!(
        "The picked button is {}",
        match index {
            0 => "Red",
            1 => "Green",
            2 => "Blue",
            3 => "Orange",
            4 => "Purple",
            _ => "None",
        }
    );
    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn render_buttons(frame: &mut Frame<'_>, area: Geometry, states: &[ButtonState]) {
    assert!(states.len() == 5, "This example only supports 3 buttons");
    let layout = Layout::default()
        .direction(Orientation::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Min(0), // ignore remaining space
        ])
        .split(area);
    frame.render_widget(Button::new("Red").theme(RED).state(states[0]), layout[0]);
    frame.render_widget(
        Button::new("Green").theme(GREEN).state(states[1]),
        layout[1],
    );
    frame.render_widget(Button::new("Blue").theme(BLUE).state(states[2]), layout[2]);
    frame.render_widget(
        Button::new("Orange").theme(ORANGE).state(states[2]),
        layout[3],
    );
    frame.render_widget(
        Button::new("Purple").theme(PURPLE).state(states[2]),
        layout[4],
    );
}
