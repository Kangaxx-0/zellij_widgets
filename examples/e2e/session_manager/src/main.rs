use std::collections::BTreeMap;

use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Color as WColor, Style as WStyle, *};

use loading::LoadingDialog;
use session::{Session, SessionList};

mod loading;
mod session;

#[derive(Default)]
struct State {
    session_name: Option<String>,
    sessions: SessionList,
    error: Option<String>,
    is_loading: bool,
    tab_locked: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        subscribe(&[
            EventType::ModeUpdate,
            EventType::SessionUpdate,
            EventType::Key,
            EventType::RunCommandResult,
        ]);

        self.is_loading = true;
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                should_render = true;
            }
            Event::Key(key) => {
                should_render = self.handle_key(key);
                should_render = true;
            }
            Event::PermissionRequestResult(_result) => {
                should_render = true;
            }
            Event::SessionUpdate(session_infos, resurrectable_session_list) => {
                self.update_session_infos(session_infos);
                if !self.sessions.is_empty() {
                    self.is_loading = false;
                }
                should_render = true;
            }

            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let stdout = std::io::stdout();
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);
        let _ = pane.draw(|frame| ui(frame, &self.sessions, self.is_loading));
    }
}

impl State {
    fn handle_key(&mut self, key: Key) -> bool {
        let mut should_render = false;
        match key {
            Key::BackTab => {
                self.sessions.next_session();
                self.tab_locked = false;
                should_render = true;
            }
            Key::Down => {
                self.sessions.next_tab();
                self.tab_locked = false;
                should_render = true;
            }
            Key::Up => {
                self.sessions.previous_tab();
                self.tab_locked = false;
                should_render = true;
            }
            Key::Char('\n') => {
                self.tab_locked = !self.tab_locked;
                should_render = true;
            }
            _ => (),
        }

        should_render
    }

    fn update_session_infos(&mut self, session_infos: Vec<SessionInfo>) {
        let session_infos: Vec<Session> = session_infos
            .iter()
            .map(|s| Session::from_session_info(s))
            .collect();
        // let current_session_name = session_infos.iter().find_map(|s| {
        //     if s.is_current_session {
        //         Some(s.name.clone())
        //     } else {
        //         None
        //     }
        // });
        // if let Some(current_session_name) = current_session_name {
        //     self.session_name = Some(current_session_name);
        // }
        self.sessions.set_sessions(session_infos);
    }
}

fn break_down_session(session_list: &SessionList) -> (Vec<String>, Vec<String>, Vec<String>) {
    let session_names = session_list
        .sessions
        .iter()
        .map(|session| session.name.clone())
        .collect::<Vec<String>>();

    let tab_names = session_list
        .sessions
        .get(session_list.selected_session_index.unwrap_or(0))
        .map_or_else(Vec::new, |session| {
            session.tabs.iter().map(|tab| tab.name.clone()).collect()
        });
    let pane_names = session_list
        .sessions
        .get(session_list.selected_session_index.unwrap_or(0))
        .map_or_else(Vec::new, |session| {
            session
                .tabs
                .get(session_list.selected_pane_index.unwrap_or(0))
                .map_or_else(Vec::new, |tab| {
                    tab.panes.iter().map(|pane| pane.name.clone()).collect()
                })
        });

    (session_names, tab_names, pane_names)
}

fn ui(frame: &mut Frame, sessions: &SessionList, is_loading: bool) {
    if is_loading {
        let loading =
            LoadingDialog::new("Please wait, we are loading session information...".to_string())
                .with_block(Block::default().borders(Borders::ALL).bg(Color::Green))
                .with_style(WStyle::default().fg(Color::Yellow).bg(Color::Black))
                .with_label_style(WStyle::default().fg(Color::Blue).bg(Color::Black));
        frame.render_widget(loading, frame.size());
    } else {
        let (session_names, tab_names, pane_names) = break_down_session(sessions);

        let layouts = Layout::default()
            .direction(Orientation::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(80),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let sub_layout = Layout::default()
            .direction(Orientation::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(layouts[1]);
        // let paragraph = Paragraph::new(format!(
        //     "session x{}, tab x{}, pane x{}",
        //     session_names.len(),
        //     tab_names.len(),
        //     pane_names.len()
        // ))
        // .style(
        //     WStyle::default()
        //         .fg(WColor::White)
        //         .bg(Color::Black)
        //         .slow_blink(),
        // )
        // .alignment(Alignment::Center);
        //
        // frame.render_widget(paragraph, layouts[0]);

        handle_session(
            layouts[0],
            frame,
            session_names,
            sessions.selected_session_index,
        );

        let mut tab_state = ListState::new(sessions.selected_tab_index, 0);

        handle_session_tabs(sub_layout[0], frame, tab_names, &mut tab_state);
        handle_session_pane(sub_layout[1], frame, pane_names);

        handle_status_bar(layouts[2], frame);
    }
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

fn handle_session_pane(layout: Geometry, frame: &mut Frame, pane_names: Vec<String>) {
    let mut list_state = ListState::new(Some(3), 2);
    let highlight_style = HighlightStyle::default().style(WStyle::default().fg(WColor::Rgb {
        r: 255,
        g: 255,
        b: 153,
    }));

    let panes: Vec<ListItem> = pane_names
        .iter()
        .map(|name| ListItem::new(name.clone()))
        .collect();

    let list = List::new_with_items(panes)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Panes")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Thick),
        )
        .block_style(WStyle::default().fg(Color::Green))
        .highlight_style(highlight_style);

    frame.render_state_widget(list, layout, &mut list_state);
}

fn handle_session_tabs(
    layout: Geometry,
    frame: &mut Frame,
    session_tabs: Vec<String>,
    tab_state: &mut ListState,
) {
    let highlight_style = HighlightStyle::default().style(WStyle::default().fg(WColor::Rgb {
        r: 255,
        g: 255,
        b: 153,
    }));
    let tabs: Vec<ListItem> = session_tabs
        .iter()
        .map(|name| ListItem::new(name.clone()))
        .collect();

    let list = List::new_with_items(tabs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tabs")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Thick),
        )
        .block_style(WStyle::default().fg(Color::Green))
        .highlight_style(highlight_style);

    frame.render_state_widget(list, layout, tab_state);
}

fn handle_session(
    layout: Geometry,
    frame: &mut Frame,
    session_names: Vec<String>,
    selected_index: Option<usize>,
) {
    let mut tab_state = if session_names.is_empty() {
        TabState::new(session_names.len())
    } else {
        TabState {
            selected: selected_index.unwrap_or(0),
            len: session_names.len(),
        }
    };

    let tabs = Tab::new(session_names)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sessions")
                .title_alignment(Alignment::Center)
                .padding(Padding::horizontal(1))
                .border_type(BorderType::Rounded),
        )
        .style(WStyle::default().fg(WColor::Green).bg(Color::Black))
        .highlight_style(
            WStyle::default()
                .fg(WColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 153,
                })
                .add_modifier(Modifier::BOLD),
        );

    frame.render_state_widget(tabs, layout, &mut tab_state);
}
