use itertools::Itertools;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Style as WStyle, *};

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    pressed_key: char,
    is_popup: bool,
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
        self.is_popup = false;
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
            'c' | 'p' => {
                // no loop for testing
                let _ = pane.draw(|frame| ui(frame, self.is_popup));
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame, is_popup: bool) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(frame.size());

    render_title(frame, layouts[0]);

    let erase_part = calculate_area(layouts[1], 70, 30);

    render_content(frame, layouts[1]);
    if is_popup {
        render_popup(frame, erase_part);
    }
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("Popup example. Press p to toggle".slow_blink())
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

fn render_popup(frame: &mut Frame, area: Geometry) {
    let popup = Paragraph::new("Popup").style(WStyle::default().bg(Color::Red));
    frame.render_widget(popup, area);
}

fn calculate_area(area: Geometry, percent_x: u16, percent_y: u16) -> Geometry {
    let popup_layout = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Orientation::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

impl State {
    fn handle_key(&mut self, e: Key) {
        if let Key::Char(c) = e {
            if c == 'c' {
                self.pressed_key = c;
            } else if c == 'p' {
                self.is_popup = !self.is_popup;
            }
        }
    }
}
