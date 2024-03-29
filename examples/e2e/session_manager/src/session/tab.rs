use zellij_tile::prelude::*;

use super::Pane;

#[derive(Debug, Clone)]
pub struct Tab {
    pub name: String,
    pub panes: Vec<Pane>,
    pub position: usize,
}

impl Tab {
    pub fn new(tab_info: &TabInfo, pane_manifest: &PaneManifest) -> Self {
        let panes = pane_manifest
            .panes
            .get(&tab_info.position)
            .map(|p| {
                p.iter()
                    .filter_map(|pane_info| {
                        if pane_info.is_selectable {
                            Some(Pane {
                                name: pane_info.title.clone(),
                                exit_code: pane_info.exit_status.clone(),
                                pane_id: pane_info.id,
                                is_plugin: pane_info.is_plugin,
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();
        Tab {
            name: tab_info.name.clone(),
            panes,
            position: tab_info.position,
        }
    }
}
