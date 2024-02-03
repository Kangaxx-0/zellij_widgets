use zellij_widgets::prelude::*;

use itertools::Itertools;

use std::collections::BTreeMap;
use zellij_tile::prelude::*;

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
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);

    //Render blocks
    render_borders(frame, Borders::ALL, layout[0][0]);
    render_borders(frame, Borders::NONE, layout[0][1]);
    render_borders(frame, Borders::LEFT, layout[1][0]);
    render_borders(frame, Borders::RIGHT, layout[1][1]);
    render_borders(frame, Borders::TOP, layout[2][0]);
    render_borders(frame, Borders::BOTTOM, layout[2][1]);

    render_border_type(frame, BorderType::Plain, layout[3][0]);
    render_border_type(frame, BorderType::Thick, layout[3][1]);
    render_border_type(frame, BorderType::Double, layout[4][0]);
    render_border_type(frame, BorderType::Rounded, layout[4][1]);
}

fn calculate_layout(area: Geometry) -> (Geometry, Vec<Vec<Geometry>>) {
    let layout = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Percentage(70),
            Constraint::Min(0),
        ])
        .split(area);
    let title_area = layout[0];
    let main_areas = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([Constraint::Max(4); 9])
        .split(layout[1])
        .iter()
        .map(|&area| {
            Layout::default()
                .direction(Orientation::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect_vec();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .red()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_borders(frame: &mut Frame, border: Borders, area: Geometry) {
    let block = Block::new()
        .borders(border)
        .title(format!("Borders::{border:#?}", border = border));

    let text = "Block";
    let parah = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });

    frame.render_widget(parah.clone().block(block), area);
}

fn render_border_type(frame: &mut Frame, border_type: BorderType, area: Geometry) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(border_type)
        .title(format!("BorderType::{border_type:#?}"));

    let text = "Block";
    let parah = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });

    frame.render_widget(parah.clone().block(block), area);
}

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) => self.pressed_key = c,
            _ => {}
        }
    }
}
