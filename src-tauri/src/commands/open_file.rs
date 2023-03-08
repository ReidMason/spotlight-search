#[tauri::command]
pub fn open_file(file: String) {
    opener::open(r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Word.lnk"#).unwrap()
}
