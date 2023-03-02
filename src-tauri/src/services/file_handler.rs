use crate::services::mime_type_handler::{get_custom_infer, get_matcher_type};
use std::{
    fs::{read_dir, DirEntry},
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
};

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
            #[cfg(target_os = "macos")]
            if is_executable(file) {
                files_array.push(path.to_owned())
            }

            #[cfg(target_os = "windows")]
            files_array.push(path.to_owned())
        } else if let Some(path_name) = path.to_str() {
            #[cfg(target_os = "macos")]
            if is_valid_mac_dir(path_name) {
                files_array.append(&mut get_dir_items(path_name));
            }

            #[cfg(target_os = "windows")]
            files_array.append(&mut get_dir_items(path_name));
        }
    }
    files_array
}

fn is_executable(file: DirEntry) -> bool {
    let path = file.path();
    if path.is_dir() {
        return false;
    }

    match file.metadata() {
        Ok(file) => {
            let permissions = file.permissions();
            permissions.mode() & 0o111 != 0
        }
        Err(_) => false,
    }
}

fn is_valid_mac_dir(path_name: &str) -> bool {
    let valid_dirs = ["/Contents", ".app", "MacOS"];
    for valid_dir in valid_dirs {
        if path_name.ends_with(valid_dir) {
            return true;
        }
    }

    false
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
                    new_files.push(file.file_name().unwrap().to_str().unwrap().to_string());
                }
            }
            None => continue,
        }
    }

    // println!("THIS IS LET NEW_FILES: {:#?}", new_files);
    new_files
}

// pub fn get_search_items() -> Vec<String> {
//     let files = get_apps_from_files(get_dir_items(
//         r#"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"#,
//     ));
//     let mut new_array: Vec<String> = vec![];
//     let input = "zzzz".to_lowercase();

//     for file in files {
//         if file.to_lowercase().contains(&input.to_lowercase()) {
//             new_array.push(file)
//         }
//     }

//     println!("THIS IS GET SEARCH ITEMS: {:?}", new_array);
//     new_array
// }
