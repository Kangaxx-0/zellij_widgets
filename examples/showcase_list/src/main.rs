use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::{
    prelude::{Style as WStyle, Text, *},
    uis::HighlightStyle,
};

#[derive(Default)]
struct State {
    is_loading: bool,
    pressed_key: char,
    list: Vec<(String, usize)>,
    pub(crate) list_state: ListState,
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
            ("Item5".to_string(), 4),
            ("Item6".to_string(), 1),
            ("Item7".to_string(), 3),
            ("Item8".to_string(), 1),
            ("Item9".to_string(), 6),
            ("Item10".to_string(), 1),
            ("Item11".to_string(), 3),
            ("Item12".to_string(), 1),
            ("Item13".to_string(), 2),
            ("Item14".to_string(), 1),
            ("Item15".to_string(), 1),
            ("Item16".to_string(), 4),
            ("Item17".to_string(), 1),
            ("Item18".to_string(), 5),
            ("Item19".to_string(), 4),
            ("Item20".to_string(), 1),
            ("Item21".to_string(), 2),
            ("Item22".to_string(), 1),
            ("Item23".to_string(), 3),
            ("Item24".to_string(), 1),
        ];
        self.list_state = ListState::new(Some(2), 1);
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

        let highlight_style =
            HighlightStyle::new("->".to_string(), WStyle::default().fg(Color::Yellow), None);
        let block = Block::default().borders(Borders::ALL).title("List").red();
        let list = List::new_with_items(item_list)
            .highlight_style(highlight_style)
            .block(block);

        match self.pressed_key {
            'c' => {
                // no loop for testing
                let _ = pane.draw(|frame| ui(frame, list, &mut self.list_state));
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame, list: List, state: &mut ListState) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(frame.size());

    render_title(frame, layouts[0]);

    render_list(frame, layouts[1], list, state);
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("List example".slow_blink())
            .red()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_list(frame: &mut Frame, area: Geometry, list: List, state: &mut ListState) {
    frame.render_state_widget(list, area, state);
}

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) => {
                if c == 'c' {
                    self.pressed_key = c;
                }
            }
            Key::Up => {
                let current_highlight_index = self.list_state.highlight_index();
                if let Some(current_highlight_index) = current_highlight_index {
                    self.list_state
                        .set_highlight_index(current_highlight_index.saturating_sub(1));
                } else {
                    self.list_state.set_highlight_index(0);
                }
            }
            Key::Down => {
                let current_highlight_index = self.list_state.highlight_index();
                // If index is greater than the length of the list, we need to reset the index to
                // the last item in the list
                if let Some(current_highlight_index) = current_highlight_index {
                    self.list_state.set_highlight_index(
                        current_highlight_index
                            .saturating_add(1)
                            .min(self.list.len() - 1),
                    );
                } else {
                    self.list_state.set_highlight_index(0);
                }
            }
            _ => {}
        }
    }
}
