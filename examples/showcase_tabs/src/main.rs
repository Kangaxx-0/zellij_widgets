use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Style as WStyle, *};

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    pressed_key: char,
    selected_tab_1: TabState,
    selected_tab_2: TabState,
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
        self.selected_tab_1 = TabState::new(3);
        self.selected_tab_2 = TabState::new(3);
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
                let _ = pane.draw(|frame| ui(frame, &self.selected_tab_1, &self.selected_tab_2));
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame, selected_tab_1: &TabState, selected_tab_2: &TabState) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(4),
                Constraint::Length(2),
                Constraint::Length(4),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(frame.size());

    render_paragraph(frame, layouts[0], "The 1st tab");
    render_tabs(frame, layouts[1], selected_tab_1);
    render_paragraph(frame, layouts[2], "The 2nd tab(arrow key is not enabled))");
    render_tabs_without_block(frame, layouts[3], selected_tab_2);
}

fn render_paragraph(frame: &mut Frame, area: Geometry, text: &str) {
    let paragraph = Paragraph::new(text)
        .style(WStyle::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_tabs(frame: &mut Frame, area: Geometry, selected_tab: &TabState) {
    let tabs = vec!["Tab1", "Tab2", "Tab3"];
    let tabs = tabs.iter().map(|t| Span::from(*t)).collect::<Vec<Span>>();

    let block = Block::default().borders(Borders::ALL).title("Tabs");

    let tab = Tab::new(tabs)
        .block(block)
        .style(WStyle::default().fg(Color::White).bg(Color::Blue))
        .divider(Span::raw("|"))
        .highlight_style(WStyle::default().fg(Color::Black).bg(Color::White));

    frame.render_state_widget(tab, area, selected_tab);
}

fn render_tabs_without_block(frame: &mut Frame, area: Geometry, selected_tab: &TabState) {
    let tabs = vec!["Tab1", "Tab2", "Tab3"];
    let tabs = tabs.iter().map(|t| Span::from(*t)).collect::<Vec<Span>>();

    let tab = Tab::new(tabs)
        .style(WStyle::default().fg(Color::White).bg(Color::Black))
        .divider(Span::raw("||"))
        .highlight_style(WStyle::default().fg(Color::Black).bg(Color::White));

    frame.render_state_widget(tab, area, selected_tab);
}

impl State {
    fn handle_key(&mut self, e: Key) {
        match e {
            Key::Char(c) if c == 'c' => {
                self.pressed_key = c;
            }
            Key::Char(c) if c == 'r' => {
                self.selected_tab_1.reset_index();
            }
            Key::Right => {
                self.selected_tab_1.next();
            }
            Key::Left => {
                self.selected_tab_1.previous();
            }
            _ => {}
        }
    }
}
