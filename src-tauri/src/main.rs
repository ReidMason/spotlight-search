#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{fs::read_dir, io, path::PathBuf};

use tauri::{command, window, AppHandle, LogicalSize, Manager, Size};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn resize_window(height: f64, app_handle: AppHandle) -> String {
    let window = app_handle.get_window("main").unwrap();
    window.set_size(Size::Logical(LogicalSize {
        width: 675.0,
        height,
    }));

    return "testing".to_string();
}

fn get_files_from_dir() -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for path in read_dir("./")? {
        let path = path?.path();
        if let Some(_) = path.extension() {
            files.push(path.to_owned());
        }
    }
    println!("It should Print HERE: {:?}", files);
    Ok(files)
}

#[command]
fn get_files() -> Vec<String> {
    let files = get_files_from_dir().unwrap();
    let mut files_new = vec![];
    for file in files {
        let file_name = file.display().to_string();
        files_new.push(file_name)
    }
    files_new
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![resize_window, get_files])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // This but it needs to be done responsively when the content loads

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
