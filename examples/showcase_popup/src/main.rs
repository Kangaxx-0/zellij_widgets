use itertools::Itertools;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Style as WStyle, *};

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
        let stdout = std::io::stdout();
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);

        match self.pressed_key {
            'c' => {
                // no loop for testing
                let _ = pane.draw(ui);
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(frame.size());

    render_title(frame, layouts[0]);

    render_content(frame, layouts[1]);
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit".slow_blink())
            .red()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_content(frame: &mut Frame, area: Geometry) {
    let block = Block::default()
        .title("Block")
        .style(WStyle::default().bg(Color::Blue))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(block, area);
}

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) => self.pressed_key = c,
            _ => {}
        }
    }
}
