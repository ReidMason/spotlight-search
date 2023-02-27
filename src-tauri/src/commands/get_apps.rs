use crate::services::file_handler::get_dir_items;
use tauri::command;

#[command]
pub fn get_apps() -> Vec<String> {
    let files = get_dir_items(r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#);
    let mut files_new = vec![];
    for file in files {
        let file_name = file.display().to_string();
        files_new.push(file_name)
    }
    files_new
}
