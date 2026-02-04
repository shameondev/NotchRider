mod ant;

use ant::channel::AntChannel;
use ant::fec::FecParser;
use ant::hrm::HrmParser;
use ant::usb::AntUsb;
use ant::TrainerData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tauri::{Manager, State};

// macOS-specific imports are used inline in setup()

struct AppState {
    ant: Mutex<AntUsb>,
    trainer_data: Mutex<TrainerData>,
    fec_channel: Mutex<Option<AntChannel>>,  // Channel 0: FE-C (trainer)
    hrm_channel: Mutex<Option<AntChannel>>,  // Channel 1: HRM (heart rate)
    connected: AtomicBool,
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

#[tauri::command]
fn connect_ant_device(state: State<AppState>) -> Result<bool, String> {
    let mut ant = state.ant.lock().map_err(|e| e.to_string())?;

    // Open USB device
    ant.open()?;

    // Initialize ANT+ channel 0 for FE-C (trainer)
    let fec_channel = AntChannel::new(0);
    let fec_init_sequence = fec_channel.get_init_sequence();

    // Send FE-C initialization sequence
    for msg in fec_init_sequence {
        ant.write(&msg)?;
        thread::sleep(Duration::from_millis(50));
    }

    println!("ANT+ FE-C channel 0 initialized");

    // Initialize ANT+ channel 1 for HRM (heart rate monitor)
    let hrm_channel = AntChannel::new(1);
    let hrm_init_sequence = hrm_channel.get_hrm_init_sequence();

    // Send HRM initialization sequence (no reset needed, network key already set)
    for msg in hrm_init_sequence {
        ant.write(&msg)?;
        thread::sleep(Duration::from_millis(50));
    }

    println!("ANT+ HRM channel 1 initialized");

    // Store channels
    {
        let mut ch = state.fec_channel.lock().map_err(|e| e.to_string())?;
        *ch = Some(fec_channel);
    }
    {
        let mut ch = state.hrm_channel.lock().map_err(|e| e.to_string())?;
        *ch = Some(hrm_channel);
    }

    state.connected.store(true, Ordering::SeqCst);
    println!("ANT+ device connected - FE-C and HRM channels ready");
    Ok(true)
}

#[tauri::command]
fn disconnect_ant_device(state: State<AppState>) -> Result<(), String> {
    state.connected.store(false, Ordering::SeqCst);

    let mut ant = state.ant.lock().map_err(|e| e.to_string())?;

    // Close FE-C channel
    if let Ok(mut ch) = state.fec_channel.lock() {
        if let Some(channel) = ch.take() {
            let close_msg = channel.close_channel();
            let _ = ant.write(&close_msg);
        }
    }

    // Close HRM channel
    if let Ok(mut ch) = state.hrm_channel.lock() {
        if let Some(channel) = ch.take() {
            let close_msg = channel.close_channel();
            let _ = ant.write(&close_msg);
        }
    }

    ant.close();
    println!("ANT+ device disconnected");
    Ok(())
}

#[tauri::command]
fn is_connected(state: State<AppState>) -> bool {
    state.connected.load(Ordering::SeqCst)
}

#[tauri::command]
fn set_window_y(window: tauri::Window, y: i32) -> Result<(), String> {
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: 0, y }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn poll_trainer_data(state: State<AppState>) -> Result<Option<TrainerData>, String> {
    if !state.connected.load(Ordering::SeqCst) {
        return Ok(None);
    }

    let ant = state.ant.lock().map_err(|e| e.to_string())?;

    // Read data from USB
    let mut buffer = [0u8; 64];
    let bytes_read = ant.read(&mut buffer)?;

    if bytes_read == 0 {
        // No data available
        let data = state.trainer_data.lock().map_err(|e| e.to_string())?;
        return Ok(Some(data.clone()));
    }

    // Parse received ANT+ message
    if let Some((msg_id, channel, data)) = AntChannel::parse_message(&buffer[..bytes_read]) {
        // Check if it's broadcast data (0x4E)
        if msg_id == 0x4E && data.len() >= 8 {
            let mut trainer_data = state.trainer_data.lock().map_err(|e| e.to_string())?;

            match channel {
                0 => {
                    // Channel 0: FE-C (trainer) data
                    if let Some(page) = FecParser::parse_data_page(&data[1..9]) {
                        FecParser::update_trainer_data(&mut trainer_data, &page);
                    }
                }
                1 => {
                    // Channel 1: HRM (heart rate) data
                    if let Some(hr) = HrmParser::parse_heart_rate(&data[1..9]) {
                        trainer_data.heart_rate = hr;
                    }
                }
                _ => {}
            }

            return Ok(Some(trainer_data.clone()));
        }
    }

    // Return current data if parsing failed
    let data = state.trainer_data.lock().map_err(|e| e.to_string())?;
    Ok(Some(data.clone()))
}

#[tauri::command]
fn show_panel(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("panel") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_panel(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(panel) = app.get_webview_window("panel") {
        panel.hide().map_err(|e| e.to_string())?;
    }
    // Return focus to main window
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.set_focus();
    }
    Ok(())
}

#[tauri::command]
fn toggle_panel(app: tauri::AppHandle) -> Result<bool, String> {
    if let Some(window) = app.get_webview_window("panel") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
            Ok(false)
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
            Ok(true)
        }
    } else {
        Err("Panel window not found".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(AppState {
            ant: Mutex::new(AntUsb::new()),
            trainer_data: Mutex::new(TrainerData::default()),
            fec_channel: Mutex::new(None),
            hrm_channel: Mutex::new(None),
            connected: AtomicBool::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            find_ant_device,
            list_usb_devices,
            get_trainer_data,
            connect_ant_device,
            disconnect_ant_device,
            is_connected,
            poll_trainer_data,
            set_window_y,
            show_panel,
            hide_panel,
            toggle_panel,
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

            // macOS: Set window level above menu bar
            #[cfg(target_os = "macos")]
            {
                use objc::runtime::Object;
                use objc::{msg_send, sel, sel_impl};

                // Get the raw NSWindow pointer
                if let Ok(ns_window) = window.ns_window() {
                    unsafe {
                        let ns_win = ns_window as *mut Object;
                        // kCGMainMenuWindowLevel = 24, we use 25 to be above menu bar
                        let level: i64 = 25;
                        let _: () = msg_send![ns_win, setLevel: level];

                        // NSWindowCollectionBehaviorCanJoinAllSpaces | Stationary | FullScreenAuxiliary
                        // = 1 | 16 | 256 = 273
                        let behavior: u64 = 1 | 16 | 256;
                        let _: () = msg_send![ns_win, setCollectionBehavior: behavior];
                    }
                    println!("Window level set above menu bar");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
