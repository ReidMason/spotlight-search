use crate::services::mime_type_handler::{get_custom_infer, get_matcher_type};
use std::{fs::read_dir, path::PathBuf};

pub fn get_dir_items(path: &str) -> Vec<PathBuf> {
    let mut files_array: Vec<PathBuf> = vec![];

    let files = match read_dir(path) {
        Ok(files) => files,
        Err(_) => {
            return files_array;
        }
    };

    for file in files {
        let file = match file {
            Ok(file) => file,
            Err(_) => continue,
        };

        let path = file.path();

        if path.is_file() {
            files_array.push(path.to_owned())
        } else if let Some(path) = path.to_str() {
            files_array.append(&mut get_dir_items(path));
        }
    }
    files_array
}

pub fn get_apps_from_files(files: Vec<PathBuf>) -> Vec<String> {
    let infer = get_custom_infer();

    // TODO: make this a map?
    let mut new_files: Vec<String> = vec![];

    for file in files {
        let kind_opt = match infer.get_from_path(&file) {
            Ok(kind_opt) => kind_opt,
            Err(_) => continue,
        };

        match kind_opt {
            Some(kind) => {
                if get_matcher_type(kind) == infer::MatcherType::App {
                    new_files.push(file.display().to_string());
                }
            }
            None => continue,
        }
    }

    println!("THIS IS LET NEW_FILES: {:#?}", new_files);
    new_files
}

pub fn get_search_items() -> Vec<String> {
    let files = get_apps_from_files(get_dir_items(
        r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#,
    ));
    let mut new_array: Vec<String> = vec![];
    let input = "zzzz".to_lowercase();

    for file in files {
        if file.to_lowercase().contains(&input.to_lowercase()) {
            new_array.push(file)
        }
    }

    println!("THIS IS GET SEARCH ITEMS: {:?}", new_array);
    new_array
}
