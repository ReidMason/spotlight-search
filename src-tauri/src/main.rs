#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod commands;
mod services;

use crate::{
    commands::{get_apps::get_apps, manage_window::resize_window, open_file::open_file},
    services::file_handler::convert_to_spotlight_file,
};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![resize_window, get_apps, open_file])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[allow(unused_imports)]
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
