use std::collections::BTreeMap;

use zellij_tile::prelude::*;
use zellij_widgets::prelude::{Color as WColor, Style as WStyle, *};

use loading::LoadingDialog;
use session::{Session, SessionList};

mod loading;
mod session;

#[derive(Default)]
struct State {
    sessions: SessionList,
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
            Event::ModeUpdate(_) => {
                should_render = true;
            }
            Event::Key(key) => {
                should_render = self.handle_key(key);
            }
            Event::PermissionRequestResult(_result) => {
                should_render = true;
            }
            Event::SessionUpdate(session_infos, _) => {
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
        let _ = pane.draw(|frame| ui(frame, &self.sessions, self.is_loading, self.tab_locked));
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
                if self.tab_locked {
                    self.sessions.next_pane();
                } else {
                    self.sessions.next_tab();
                    self.tab_locked = false;
                }
                should_render = true;
            }
            Key::Up => {
                if self.tab_locked {
                    self.sessions.previous_pane();
                } else {
                    self.sessions.previous_tab();
                    self.tab_locked = false;
                }
                should_render = true;
            }
            Key::Char('\n') => {
                self.tab_locked = !self.tab_locked;
                should_render = true;
            }
            Key::Ctrl('s') => {
                self.switch_session();
                should_render = true;
            }
            _ => (),
        }

        should_render
    }

    fn update_session_infos(&mut self, session_infos: Vec<SessionInfo>) {
        let session_infos: Vec<Session> = session_infos
            .iter()
            .map(Session::from_session_info)
            .collect();
        self.sessions.set_sessions(session_infos);
    }

    fn switch_session(&mut self) {
        let is_current_session = self.sessions.selected_is_current_session();
        if is_current_session {
            if let Some(pane_id) = self.sessions.selected_pane_index {
                focus_terminal_pane(pane_id.try_into().unwrap(), true);
            } else if let Some(tab_position) = self.sessions.selected_tab_index {
                go_to_tab(tab_position as u32);
            }
        } else {
            let session_name = self.sessions.get_selected_session_name().unwrap();
            let pane_id = self.sessions.selected_pane_index.unwrap_or(0);
            switch_session_with_focus(
                &session_name,
                self.sessions.selected_tab_index,
                Some((pane_id.try_into().unwrap(), false)),
            );
        }
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
                .get(session_list.selected_tab_index.unwrap_or(0))
                .map_or_else(Vec::new, |tab| {
                    tab.panes.iter().map(|pane| pane.name.clone()).collect()
                })
        });

    (session_names, tab_names, pane_names)
}

fn ui(frame: &mut Frame, sessions: &SessionList, is_loading: bool, tab_locked: bool) {
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

        handle_session(
            layouts[0],
            frame,
            session_names,
            sessions.selected_session_index,
        );

        let mut tab_state = ListState::new(sessions.selected_tab_index, 0);

        handle_session_tabs(sub_layout[0], frame, tab_names, &mut tab_state, tab_locked);
        if tab_locked {
            let mut pane_state = ListState::new(sessions.selected_pane_index, 0);
            handle_session_pane(sub_layout[1], frame, pane_names, &mut pane_state);
        } else {
            handle_session_pane_hint(sub_layout[1], frame);
        }

        handle_status_bar(layouts[2], frame);
    }
}

fn handle_status_bar(layout: Geometry, frame: &mut Frame) {
    let parah = "<TAB> to switch session, <UP/DOWN> to switch tab / switch pane if tab is locked, <ENTER> to lock/unlock tab, <CTRL + S> to switch pane";
    let status_bar = Paragraph::new(parah)
        .style(
            WStyle::default()
                .fg(WColor::White)
                .bg(Color::Black)
                .slow_blink(),
        )
        .alignment(Alignment::Center);

    frame.render_widget(status_bar, layout);
}

fn handle_session_pane_hint(layout: Geometry, frame: &mut Frame) {
    let hint = Paragraph::new("Press Enter to lock the tab")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        )
        .style(WStyle::default().fg(WColor::White).bg(Color::Black))
        .alignment(Alignment::Center);

    frame.render_widget(hint, layout);
}

fn handle_session_pane(
    layout: Geometry,
    frame: &mut Frame,
    pane_names: Vec<String>,
    pane_state: &mut ListState,
) {
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

    frame.render_state_widget(list, layout, pane_state);
}

fn handle_session_tabs(
    layout: Geometry,
    frame: &mut Frame,
    session_tabs: Vec<String>,
    tab_state: &mut ListState,
    tab_locked: bool,
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

    let block = if tab_locked {
        Block::default()
            .borders(Borders::ALL)
            .title("Tabs(locked)")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Thick)
    } else {
        Block::default()
            .borders(Borders::ALL)
            .title("Tabs")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Thick)
    };

    let mut list = List::new_with_items(tabs)
        .block(block)
        .highlight_style(highlight_style);

    list = if tab_locked {
        list.block_style(WStyle::default().fg(Color::Red))
    } else {
        list.block_style(WStyle::default().fg(Color::Green))
    };

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
