use session::SessionList;
use std::collections::BTreeMap;

use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Color as WColor, Style as WStyle, *};

mod session;

#[derive(Default)]
struct State {
    sessions: SessionList,
    error: Option<String>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[
            EventType::ModeUpdate,
            EventType::SessionUpdate,
            EventType::Key,
            EventType::RunCommandResult,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                should_render = true;
            }
            Event::Key(key) => {
                // should_render = self.handle_key(key);
                should_render = true;
            }
            Event::PermissionRequestResult(_result) => {
                should_render = true;
            }
            Event::SessionUpdate(session_infos, resurrectable_session_list) => {
                should_render = true;
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let stdout = std::io::stdout();
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);
        let _ = pane.draw(|frame| ui(frame));
    }
}

impl State {
    fn handle_key(&mut self, key: Key) -> bool {
        todo!()
    }
}

fn ui(frame: &mut Frame) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(70),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let sub_layout = Layout::default()
        .direction(Orientation::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(layouts[1]);

    handle_session(layouts[0], frame);

    handle_session_tabs(sub_layout[0], frame);
    handle_session_pane(sub_layout[1], frame);

    handle_status_bar(layouts[2], frame);
}

fn handle_status_bar(layout: Geometry, frame: &mut Frame) {
    let status_bar = Paragraph::new("Status Bar")
        .style(
            WStyle::default()
                .fg(WColor::White)
                .bg(Color::Black)
                .slow_blink(),
        )
        .alignment(Alignment::Center);

    frame.render_widget(status_bar, layout);
}

fn handle_session_pane(layout: Geometry, frame: &mut Frame) {
    let mut list_state = ListState::new(Some(3), 2);
    let list = List::new_with_items(vec![
        ListItem::new("Pane 1"),
        ListItem::new("Pane 2"),
        ListItem::new("Pane 3"),
        ListItem::new("Pane 4"),
        ListItem::new("Pane 5"),
        ListItem::new("Pane 6"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Panes")
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded),
    );

    frame.render_state_widget(list, layout, &mut list_state);
}

fn handle_session_tabs(layout: Geometry, frame: &mut Frame) {
    let mut list_state = ListState::new(Some(1), 1);
    let list = List::new_with_items(vec![
        ListItem::new("Tab 1"),
        ListItem::new("Tab 2"),
        ListItem::new("Tab 3"),
        ListItem::new("Tab 4"),
        ListItem::new("Tab 5"),
        ListItem::new("Tab 6"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Tabs")
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded),
    );

    frame.render_state_widget(list, layout, &mut list_state);
}

fn handle_session(layout: Geometry, frame: &mut Frame) {
    let mut tab_state = TabState::new(3);
    let tabs = Tab::new(vec![
        "Session 1",
        "Session 2",
        "Session 3",
        "Session 4",
        "Session 5",
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Sessions")
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded),
    )
    .style(WStyle::default().fg(WColor::White).bg(Color::Black))
    .highlight_style(
        WStyle::default()
            .fg(WColor::Green)
            .bg(WColor::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_state_widget(tabs, layout, &mut tab_state);
}
