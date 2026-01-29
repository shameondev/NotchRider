mod ant;

use ant::usb::AntUsb;
use ant::TrainerData;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    ant: Mutex<AntUsb>,
    trainer_data: Mutex<TrainerData>,
}

#[tauri::command]
fn find_ant_device(state: State<AppState>) -> Result<bool, String> {
    let mut ant = state.ant.lock().map_err(|e| e.to_string())?;
    ant.find_device()
}

#[tauri::command]
fn list_usb_devices(state: State<AppState>) -> Result<Vec<String>, String> {
    let ant = state.ant.lock().map_err(|e| e.to_string())?;
    ant.list_usb_devices()
}

#[tauri::command]
fn get_trainer_data(state: State<AppState>) -> Result<TrainerData, String> {
    let data = state.trainer_data.lock().map_err(|e| e.to_string())?;
    Ok(data.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(AppState {
            ant: Mutex::new(AntUsb::new()),
            trainer_data: Mutex::new(TrainerData::default()),
        })
        .invoke_handler(tauri::generate_handler![
            find_ant_device,
            list_usb_devices,
            get_trainer_data,
        ])
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .ok_or("Main window not found")?;

            // 148px = 74px × 2 (menu bar height doubled for notch coverage)
            if let Some(monitor) = window.current_monitor()? {
                let size = monitor.size();
                window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: size.width,
                    height: 148,
                }))?;
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: 0,
                    y: 0,
                }))?;
            } else {
                eprintln!("Warning: Could not detect monitor, using default window configuration");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
