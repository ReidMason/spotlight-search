use crate::services::file_handler::{
    convert_to_spotlight_file, get_dir_items, get_file_name, SpotlightFile,
};

#[tauri::command]
pub fn get_apps(search: String) -> Vec<SpotlightFile> {
    let search = search.trim();
    if search.is_empty() {
        return vec![];
    }

    let mut files = get_dir_items(r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#);

    #[cfg(target_os = "macos")]
    {
        let mac_files = get_dir_items(r#"/Applications"#);
        let mut mac_apps = convert_to_spotlight_file(mac_files);
        files.append(&mut mac_apps);
    }

    files.retain(|x| {
        if let Some(file_name) = get_file_name(x) {
            file_name.to_lowercase().contains(&search.to_lowercase())
        } else {
            false
        }
    });
    convert_to_spotlight_file(files)
}
