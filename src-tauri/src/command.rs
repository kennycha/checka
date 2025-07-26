use std::sync::Once;

use tauri_nspanel::ManagerExt;

use crate::fns::{
    setup_menubar_panel_listeners, swizzle_to_menubar_panel, update_menubar_appearance,
};
use crate::agent_manager::{AgentManager, AgentSummary};

static INIT: Once = Once::new();

#[tauri::command]
pub fn init(app_handle: tauri::AppHandle) {
    INIT.call_once(|| {
        swizzle_to_menubar_panel(&app_handle);

        update_menubar_appearance(&app_handle);

        setup_menubar_panel_listeners(&app_handle);
    });
}

#[tauri::command]
pub fn show_menubar_panel(app_handle: tauri::AppHandle) {
    let panel = app_handle.get_webview_panel("main").unwrap();

    panel.show();
}

#[tauri::command]
pub fn get_agent_summary() -> AgentSummary {
    let manager = AgentManager::new();
    manager.get_summary()
}

#[tauri::command]
pub fn get_processing_count() -> usize {
    let manager = AgentManager::new();
    manager.get_processing_count()
}

#[tauri::command]  
pub fn has_active_agents() -> bool {
    let manager = AgentManager::new();
    manager.has_active_agents()
}

#[tauri::command]
pub fn quit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
