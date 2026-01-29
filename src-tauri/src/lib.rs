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

      let window = app.get_webview_window("main")
          .ok_or("Main window not found")?;

      // Get screen size and position window at top
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
