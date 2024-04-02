use super::Tab;
use crate::SessionInfo;

#[derive(Debug, Default, Clone)]
pub struct SessionList {
    pub is_searching: bool,
    pub sessions: Vec<Session>,
    pub selected_search_index: Option<usize>,
    pub selected_session_index: Option<usize>,
    pub selected_tab_index: Option<usize>,
    pub selected_pane_index: Option<usize>,
}

#[allow(dead_code)]
impl SessionList {
    pub fn set_sessions(&mut self, mut session: Vec<Session>) {
        session.sort_unstable_by(|a, b| {
            if a.is_current_session {
                std::cmp::Ordering::Less
            } else if b.is_current_session {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });
        self.sessions = session;
    }

    pub fn next_session(&mut self) {
        if let Some(selected_session) = self.selected_session_index {
            if selected_session < self.sessions.len() - 1 {
                self.selected_session_index = Some(selected_session + 1);
            } else {
                self.selected_session_index = Some(0);
            }
        } else {
            self.selected_session_index = Some(0);
        }

        self.selected_tab_index = Some(0);
    }

    pub fn previous_session(&mut self) {
        if let Some(selected_session) = self.selected_session_index {
            if selected_session > 0 {
                self.selected_session_index = Some(selected_session - 1);
            } else {
                self.selected_session_index = Some(self.sessions.len() - 1);
            }
        } else {
            self.selected_session_index = Some(self.sessions.len() - 1);
        }

        self.selected_tab_index = Some(0);
    }

    pub fn next_tab(&mut self) {
        if let Some(selected_tab) = self.selected_tab_index {
            if selected_tab
                < self.sessions[self.selected_session_index.unwrap()]
                    .tabs
                    .len()
                    - 1
            {
                self.selected_tab_index = Some(selected_tab + 1);
            } else {
                self.selected_tab_index = Some(0);
            }
        } else {
            self.selected_tab_index = Some(0);
        }
    }

    pub fn previous_tab(&mut self) {
        if let Some(selected_tab) = self.selected_tab_index {
            if selected_tab > 0 {
                self.selected_tab_index = Some(selected_tab - 1);
            } else {
                self.selected_tab_index = Some(
                    self.sessions[self.selected_session_index.unwrap()]
                        .tabs
                        .len()
                        - 1,
                );
            }
        } else {
            self.selected_tab_index = Some(
                self.sessions[self.selected_session_index.unwrap()]
                    .tabs
                    .len()
                    - 1,
            );
        }
    }

    pub fn next_pane(&mut self) {
        if let Some(selected_pane) = self.selected_pane_index {
            if selected_pane
                < self.sessions[self.selected_session_index.unwrap()].tabs
                    [self.selected_tab_index.unwrap()]
                .panes
                .len()
                    - 1
            {
                self.selected_pane_index = Some(selected_pane + 1);
            } else {
                self.selected_pane_index = Some(0);
            }
        } else {
            self.selected_pane_index = Some(0);
        }
    }

    pub fn previous_pane(&mut self) {
        if let Some(selected_pane) = self.selected_pane_index {
            if selected_pane > 0 {
                self.selected_pane_index = Some(selected_pane - 1);
            } else {
                self.selected_pane_index = Some(
                    self.sessions[self.selected_session_index.unwrap()].tabs
                        [self.selected_tab_index.unwrap()]
                    .panes
                    .len()
                        - 1,
                );
            }
        } else {
            self.selected_pane_index = Some(
                self.sessions[self.selected_session_index.unwrap()].tabs
                    [self.selected_tab_index.unwrap()]
                .panes
                .len()
                    - 1,
            );
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn has_session(&self, session_name: &str) -> bool {
        self.sessions.iter().any(|s| s.name == session_name)
    }

    pub fn selected_is_current_session(&self) -> bool {
        self.sessions
            .get(self.selected_session_index.unwrap())
            .map(|s| s.is_current_session)
            .unwrap_or(false)
    }

    pub fn get_selected_session_name(&self) -> Option<String> {
        self.selected_session_index
            .and_then(|i| self.sessions.get(i))
            .map(|s_i| s_i.name.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub tabs: Vec<Tab>,
    pub connected_users: usize,
    pub is_current_session: bool,
}

impl Session {
    pub fn from_session_info(session_info: &SessionInfo) -> Self {
        Session {
            name: session_info.name.clone(),
            tabs: session_info
                .tabs
                .iter()
                .map(|t| Tab::new(t, &session_info.panes))
                .collect(),
            connected_users: session_info.connected_clients,
            is_current_session: session_info.is_current_session,
        }
    }
}
