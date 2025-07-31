// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;
mod agents;
mod agent_manager;

use tauri::Manager;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use agent_manager::{AgentManager, AgentSummary};

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    let agent_manager = Arc::new(Mutex::new(AgentManager::new()));
    let agent_summary = Arc::new(Mutex::new(None::<AgentSummary>));

    let agent_manager_clone = Arc::clone(&agent_manager);
    let agent_summary_clone = Arc::clone(&agent_summary);

    std::thread::spawn(move || {
        loop {
            let summary = agent_manager_clone.lock().unwrap().get_summary();
            *agent_summary_clone.lock().unwrap() = Some(summary);
            std::thread::sleep(Duration::from_secs(2));
        }
    });

    tauri::Builder::default()
        .manage(agent_summary)
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel,
            command::get_agent_summary,
            command::get_processing_count,
            command::has_active_agents,
            command::quit_app
        ])
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            // Apply vibrancy to the main window
            #[cfg(target_os = "macos")]
            if let Some(window) = app.get_webview_window("main") {
                let _ = apply_vibrancy(&window, NSVisualEffectMaterial::Popover, None, Some(8.0));
            }

            tray::create(app_handle)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}