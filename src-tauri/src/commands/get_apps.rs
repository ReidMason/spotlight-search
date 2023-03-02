use crate::services::file_handler::{get_apps_from_files, get_dir_items};
use tauri::command;

#[command]
pub fn get_apps(search: String) -> Vec<String> {
    if search.is_empty() {
        return vec![];
    }

    let mut files = get_apps_from_files(get_dir_items(
        r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#,
    ));

    #[cfg(target_os = "macos")]
    {
        let mac_files = get_dir_items(r#"/Applications"#);
        let mut mac_apps = get_apps_from_files(mac_files);
        files.append(&mut mac_apps);
    }

    files.retain(|x| x.to_lowercase().contains(&search));
    files.truncate(5);
    files
}
