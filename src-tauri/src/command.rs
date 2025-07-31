use std::sync::{Arc, Mutex, Once};

use tauri::{Manager, State};
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
pub fn get_agent_summary(state: State<Arc<Mutex<Option<AgentSummary>>>>) -> Option<AgentSummary> {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_processing_count(state: State<Arc<Mutex<Option<AgentSummary>>>>) -> usize {
    if let Some(summary) = state.lock().unwrap().as_ref() {
        summary.processing_count
    } else {
        0
    }
}

#[tauri::command]
pub fn has_active_agents(state: State<Arc<Mutex<Option<AgentSummary>>>>) -> bool {
    if let Some(summary) = state.lock().unwrap().as_ref() {
        summary.active_count > 0
    } else {
        false
    }
}

#[tauri::command]
pub fn quit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}
