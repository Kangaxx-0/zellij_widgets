use super::Tab;

#[derive(Debug, Default)]
pub struct SessionList {
    pub session_ui_infos: Vec<Session>,
    pub selected_index: SelectedIndex,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub tabs: Vec<Tab>,
    pub connected_users: usize,
    pub is_current_session: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SelectedIndex(pub Option<usize>, pub Option<usize>, pub Option<usize>);
