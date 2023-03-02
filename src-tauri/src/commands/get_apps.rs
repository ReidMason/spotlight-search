use crate::services::file_handler::{get_apps_from_files, get_dir_items};
use tauri::command;

#[command]
pub fn get_apps() -> Vec<String> {
    let files = get_apps_from_files(get_dir_items(
        r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#,
    ));
    files
}
