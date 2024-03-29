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
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }
    // pub fn get_selected_session_name(&self) -> Option<String> {
    //     self.selected_index
    //         .0
    //         .and_then(|i| self.sessions.get(i))
    //         .map(|s_i| s_i.name.clone())
    //     //}
    // }
    // pub fn selected_is_current_session(&self) -> bool {
    //     self.selected_index
    //         .0
    //         .and_then(|i| self.sessions.get(i))
    //         .map(|s_i| s_i.is_current_session)
    //         .unwrap_or(false)
    //     //}
    // }
    // pub fn get_selected_tab_position(&self) -> Option<usize> {
    //     self.selected_index
    //         .0
    //         .and_then(|i| self.sessions.get(i))
    //         .and_then(|s_i| {
    //             self.selected_index
    //                 .1
    //                 .and_then(|i| s_i.tabs.get(i))
    //                 .map(|t| t.position)
    //         })
    //     //}
    // }
    // pub fn get_selected_pane_id(&self) -> Option<(u32, bool)> {
    //     self.selected_index
    //         .0
    //         .and_then(|i| self.sessions.get(i))
    //         .and_then(|s_i| {
    //             self.selected_index
    //                 .1
    //                 .and_then(|i| s_i.tabs.get(i))
    //                 .and_then(|t| {
    //                     self.selected_index
    //                         .2
    //                         .and_then(|i| t.panes.get(i))
    //                         .map(|p| (p.pane_id, p.is_plugin))
    //                 })
    //         })
    //     //}
    // }
    // pub fn move_selection_down(&mut self) {
    //     match self.selected_index {
    //         SelectedIndex(None, None, None) => {
    //             if !self.sessions.is_empty() {
    //                 self.selected_index.0 = Some(0);
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), None, None) => {
    //             if self.sessions.len() > selected_session + 1 {
    //                 self.selected_index.0 = Some(selected_session + 1);
    //             } else {
    //                 self.selected_index.0 = None;
    //                 self.selected_index.1 = None;
    //                 self.selected_index.2 = None;
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), Some(selected_tab), None) => {
    //             if self
    //                 .get_session(selected_session)
    //                 .map(|s| s.tabs.len() > selected_tab + 1)
    //                 .unwrap_or(false)
    //             {
    //                 self.selected_index.1 = Some(selected_tab + 1);
    //             } else {
    //                 self.selected_index.1 = Some(0);
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), Some(selected_tab), Some(selected_pane)) => {
    //             if self
    //                 .get_session(selected_session)
    //                 .and_then(|s| s.tabs.get(selected_tab))
    //                 .map(|t| t.panes.len() > selected_pane + 1)
    //                 .unwrap_or(false)
    //             {
    //                 self.selected_index.2 = Some(selected_pane + 1);
    //             } else {
    //                 self.selected_index.2 = Some(0);
    //             }
    //         }
    //         _ => {} //}
    //     }
    // }
    // pub fn move_selection_up(&mut self) {
    //     // if self.is_searching {
    //     //     match self.selected_search_index.as_mut() {
    //     //         Some(search_index) => {
    //     //             *search_index = search_index.saturating_sub(1);
    //     //         }
    //     //         None => {
    //     //             if !self.search_results.is_empty() {
    //     //                 self.selected_search_index = Some(0);
    //     //             }
    //     //         }
    //     //     }
    //     // } else {
    //     match self.selected_index {
    //         SelectedIndex(None, None, None) => {
    //             if !self.sessions.is_empty() {
    //                 self.selected_index.0 = Some(self.sessions.len().saturating_sub(1))
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), None, None) => {
    //             if selected_session > 0 {
    //                 self.selected_index.0 = Some(selected_session - 1);
    //             } else {
    //                 self.selected_index.0 = None;
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), Some(selected_tab), None) => {
    //             if selected_tab > 0 {
    //                 self.selected_index.1 = Some(selected_tab - 1);
    //             } else {
    //                 let tab_count = self
    //                     .get_session(selected_session)
    //                     .map(|s| s.tabs.len())
    //                     .unwrap_or(0);
    //                 self.selected_index.1 = Some(tab_count.saturating_sub(1))
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), Some(selected_tab), Some(selected_pane)) => {
    //             if selected_pane > 0 {
    //                 self.selected_index.2 = Some(selected_pane - 1);
    //             } else {
    //                 let pane_count = self
    //                     .get_session(selected_session)
    //                     .and_then(|s| s.tabs.get(selected_tab))
    //                     .map(|t| t.panes.len())
    //                     .unwrap_or(0);
    //                 self.selected_index.2 = Some(pane_count.saturating_sub(1))
    //             }
    //         }
    //         _ => {}
    //     }
    //     //}
    // }
    // fn get_session(&self, index: usize) -> Option<&Session> {
    //     self.sessions.get(index)
    // }
    // pub fn result_expand(&mut self) {
    //     // we can't move this to SelectedIndex because the borrow checker is mean
    //     match self.selected_index {
    //         SelectedIndex(Some(selected_session), None, None) => {
    //             let selected_session_has_tabs = self
    //                 .get_session(selected_session)
    //                 .map(|s| !s.tabs.is_empty())
    //                 .unwrap_or(false);
    //             if selected_session_has_tabs {
    //                 self.selected_index.1 = Some(0);
    //             }
    //         }
    //         SelectedIndex(Some(selected_session), Some(selected_tab), None) => {
    //             let selected_tab_has_panes = self
    //                 .get_session(selected_session)
    //                 .and_then(|s| s.tabs.get(selected_tab))
    //                 .map(|t| !t.panes.is_empty())
    //                 .unwrap_or(false);
    //             if selected_tab_has_panes {
    //                 self.selected_index.2 = Some(0);
    //             }
    //         }
    //         _ => {}
    //     }
    // }
    // pub fn result_shrink(&mut self) {
    //     self.selected_index.result_shrink();
    // }
    // pub fn update_rows(&mut self, rows: usize) {
    //     if let Some(search_result_rows_until_selected) = self.selected_search_index.map(|i| {
    //         self.search_results
    //             .iter()
    //             .enumerate()
    //             .take(i + 1)
    //             .fold(0, |acc, s| acc + s.1.lines_to_render())
    //     }) {
    //         if search_result_rows_until_selected > rows
    //             || self.selected_search_index >= Some(self.search_results.len())
    //         {
    //             self.selected_search_index = None;
    //         }
    //     }
    // }
    pub fn has_session(&self, session_name: &str) -> bool {
        self.sessions.iter().any(|s| s.name == session_name)
    }
    pub fn update_session_name(&mut self, old_name: &str, new_name: &str) {
        self.sessions
            .iter_mut()
            .find(|s| s.name == old_name)
            .map(|s| s.name = new_name.to_owned());
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
