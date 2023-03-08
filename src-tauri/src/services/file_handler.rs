use crate::services::mime_type_handler::{get_custom_infer, get_matcher_type};
#[cfg(target_os = "macos")]
use std::os::unix::prelude::PermissionsExt;
use std::{
    fs::{read_dir, DirEntry},
    path::PathBuf,
};

#[derive(Debug)]
pub struct SpotlightFile {
    name: String,
    path: PathBuf,
    icon: String,
}

impl SpotlightFile {
    pub fn new_from_path_buf(path: PathBuf) -> SpotlightFile {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        let file = SpotlightFile {
            name,
            path,
            icon: "img here".to_string(),
        };
        file
    }
}

pub fn convert_to_spotlight_file(path_bufs: Vec<PathBuf>) -> Vec<SpotlightFile> {
    let path_bufs = path_bufs;
    let mut files: Vec<SpotlightFile> = vec![];

    for path_buf in path_bufs {
        files.push(SpotlightFile::new_from_path_buf(path_buf));
    }

    files
}

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

#[cfg(target_os = "macos")]
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

#[cfg(target_os = "macos")]
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
                    // new_files.push(file.to_string_lossy().to_string())
                }
            }
            None => continue,
        }
    }

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
