#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{window, AppHandle, LogicalSize, Manager, Size, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn resize_window(app: AppHandle) -> Window {
    let window = app.get_window("main").unwrap();
    window.set_size(Size::Logical(LogicalSize {
        width: 675.0,
        height: 100.0,
    }));
    window
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, resize_window])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // This but it needs to be done responsively when the content loads

            use window_vibrancy::{
                apply_blur, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState,
            };

            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &window,
                NSVisualEffectMaterial::HudWindow,
                Some(NSVisualEffectState::Active),
                Some(11.2),
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
