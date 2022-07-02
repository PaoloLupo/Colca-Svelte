#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use window_vibrancy::{apply_mica,apply_blur};
use tauri::Manager;

fn main() {
    let context = tauri::generate_context!();
    let window= tauri::Builder::default()
        .setup(|app| {
            let win= app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            apply_mica(&win)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
