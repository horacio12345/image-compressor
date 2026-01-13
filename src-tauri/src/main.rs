// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            image_compressor_lib::commands::process_images_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}