mod ant;

use ant::channel::AntChannel;
use ant::fec::FecParser;
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
    channel: Mutex<Option<AntChannel>>,
    connected: AtomicBool,
    running: AtomicBool,
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

    // Initialize ANT+ channel for FE-C
    let channel = AntChannel::new(0);
    let init_sequence = channel.get_init_sequence();

    // Send initialization sequence with delays between messages
    for msg in init_sequence {
        ant.write(&msg)?;
        thread::sleep(Duration::from_millis(50)); // Allow device to process
    }

    // Store channel for later use
    let mut ch = state.channel.lock().map_err(|e| e.to_string())?;
    *ch = Some(channel);
    state.connected.store(true, Ordering::SeqCst);

    println!("ANT+ device connected and channel initialized");
    Ok(true)
}

#[tauri::command]
fn disconnect_ant_device(state: State<AppState>) -> Result<(), String> {
    state.connected.store(false, Ordering::SeqCst);

    let mut ant = state.ant.lock().map_err(|e| e.to_string())?;

    // Close channel if open
    if let Ok(mut ch) = state.channel.lock() {
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
    if let Some((msg_id, _channel, data)) = AntChannel::parse_message(&buffer[..bytes_read]) {
        // Check if it's broadcast data (0x4E)
        if msg_id == 0x4E && data.len() >= 8 {
            // Parse FE-C data page
            if let Some(page) = FecParser::parse_data_page(&data[1..9]) {
                let mut trainer_data = state.trainer_data.lock().map_err(|e| e.to_string())?;
                FecParser::update_trainer_data(&mut trainer_data, &page);
                return Ok(Some(trainer_data.clone()));
            }
        }
    }

    // Return current data if parsing failed
    let data = state.trainer_data.lock().map_err(|e| e.to_string())?;
    Ok(Some(data.clone()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(AppState {
            ant: Mutex::new(AntUsb::new()),
            trainer_data: Mutex::new(TrainerData::default()),
            channel: Mutex::new(None),
            connected: AtomicBool::new(false),
            running: AtomicBool::new(false),
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
