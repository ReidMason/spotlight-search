use crate::services::file_handler::{get_apps_from_files, get_dir_items};
use tauri::command;

#[command]
pub fn get_apps() -> Vec<String> {
    let mut files = get_apps_from_files(get_dir_items(
        r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#,
    ));

    #[cfg(target_os = "macos")]
    let mac_files = get_dir_items(r#"/Applications"#);
    let mut mac_apps = get_apps_from_files(mac_files);
    files.append(&mut mac_apps);

    files
}
