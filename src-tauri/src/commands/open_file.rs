#[tauri::command]
pub fn open_file(file: String) {
    opener::open(file).unwrap()
}
