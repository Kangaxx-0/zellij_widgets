use encoding::bool;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Style as WStyle, *};

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    ratio: f64,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::SessionUpdate]);
        self.is_loading = true;
        self.ratio = 0.1;
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::SessionUpdate(_, _) => {
                self.ratio += 0.01_f64;
                if self.ratio >= 1.0 {
                    self.ratio = 0.0;
                }
                true
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let stdout = std::io::stdout();
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);

        let _ = pane.draw(|frame| ui(frame, self.ratio));
    }
}

fn ui(frame: &mut Frame, ratio: f64) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)].as_ref())
        .split(frame.size());

    render_gauge(frame, layouts[0], ratio, "/100");
}

fn render_gauge(frame: &mut Frame, area: Geometry, ratio: f64, text: &str) {
    let gauge = Gauge::new(Block::new().title("Gauge").border_option(BorderOptions {
        borders: Borders::ALL,
        border_set: BorderType::Rounded.to_border_set(),
        border_style: WStyle::default(),
    }))
    .label(text)
    .ratio(ratio)
    .style(WStyle::default().fg(Color::Yellow).bg(Color::Red));
    frame.render_widget(gauge, area);
}
