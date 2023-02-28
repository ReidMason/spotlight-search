use crate::services::file_handler::get_dir_items;
use tauri::command;
//r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#

#[command]
pub fn get_apps() -> Vec<String> {
    let files = get_dir_items(r#"C:\Users\Web.RNW\Pictures"#);
    // let mut files_new = vec![];
    // for file in files {
    //     let file_name = file.display().to_string();
    //     files_new.push(file_name)
    // }
    // files_new
    let mut new_files: Vec<String> = vec![];
    for file in files {
        let kind_opt = match infer::get_from_path(&file) {
            Ok(kind_opt) => kind_opt,
            Err(_) => continue,
        };

        match kind_opt {
            Some(kind) => {
                if kind.mime_type() == "image/jpg" {
                    println!("{:?}", file);
                    new_files.push(file.display().to_string());
                }
            }
            None => continue,
        }
    }

    new_files
}
