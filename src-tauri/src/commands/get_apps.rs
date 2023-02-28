use crate::services::file_handler::{get_dir_items, get_apps1};
use tauri::command;
//r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#

#[command]
pub fn get_apps() -> Vec<String> {
    let files = get_apps1();
    files
}
