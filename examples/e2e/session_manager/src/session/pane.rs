#[derive(Debug, Clone)]
pub struct Pane {
    pub name: String,
    pub exit_code: Option<i32>,
    pub pane_id: u32,
    pub is_plugin: bool,
}
