use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Get the main window and position it at top of screen with full width
      let window = app.get_webview_window("main").unwrap();

      if let Some(monitor) = window.current_monitor().unwrap() {
        let size = monitor.size();
        window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
          width: size.width,
          height: 148,
        })).unwrap();
        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
          x: 0,
          y: 0,
        })).unwrap();
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
