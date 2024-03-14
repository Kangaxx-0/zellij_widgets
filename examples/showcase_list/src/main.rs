use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Style as WStyle, Text, *};

#[derive(Default)]
struct State {
    is_loading: bool,
    pressed_key: char,
    list: Vec<(String, usize)>,
    list_state: ListState,
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
        self.list = vec![
            ("Item0".to_string(), 1),
            ("Item1".to_string(), 2),
            ("Item2".to_string(), 1),
            ("Item3".to_string(), 3),
            ("Item4".to_string(), 1),
            // ("Item5".to_string(), 4),
            // ("Item6".to_string(), 1),
            // ("Item7".to_string(), 3),
            // ("Item8".to_string(), 1),
            // ("Item9".to_string(), 6),
            // ("Item10".to_string(), 1),
            // ("Item11".to_string(), 3),
            // ("Item12".to_string(), 1),
            // ("Item13".to_string(), 2),
            // ("Item14".to_string(), 1),
            // ("Item15".to_string(), 1),
            // ("Item16".to_string(), 4),
            // ("Item17".to_string(), 1),
            // ("Item18".to_string(), 5),
            // ("Item19".to_string(), 4),
            // ("Item20".to_string(), 1),
            // ("Item21".to_string(), 2),
            // ("Item22".to_string(), 1),
            // ("Item23".to_string(), 3),
            // ("Item24".to_string(), 1),
        ]
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
        let item_list: Vec<_> = self
            .list
            .iter()
            .map(|(s, count)| {
                let item = format!("{}\n{} ", s, "This is a test\n".repeat(*count));
                ListItem::new(Text::from(item))
            })
            .collect();

        let list = List::new_with_items(item_list);

        match self.pressed_key {
            'c' | 'p' => {
                // no loop for testing
                let _ = pane.draw(|frame| ui(frame, list));
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame, list: List) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(frame.size());

    render_title(frame, layouts[0]);

    render_list(frame, layouts[1], list);
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("List example".slow_blink())
            .red()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_list(frame: &mut Frame, area: Geometry, list: List) {
    frame.render_state_widget(
        list,
        area,
        &mut ListState {
            highlight_index: Some(0),
            start_pos_to_display: 0,
        },
    );
}

impl State {
    fn handle_key(&mut self, e: Key) {
        if let Key::Char(c) = e {
            if c == 'c' {
                self.pressed_key = c;
            } else if c == 'p' {
                todo!()
            }
        }
    }
}
