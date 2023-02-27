use std::{fs::read_dir, path::PathBuf};

pub fn get_dir_items<'a>(path: &str) -> Vec<PathBuf> {
    let mut files_array: Vec<PathBuf> = vec![];
    let files = match read_dir(path) {
        Ok(files) => files,
        Err(_) => {
            return vec![];
        }
    };

    for file in files {
        let file = match file {
            Ok(file) => file,
            Err(_) => continue,
        };

        let path = file.path();
        match path.extension() {
            Some(_) => files_array.push(path.to_owned()),
            None => {
                if let Some(path) = path.to_str() {
                    files_array.append(&mut get_dir_items(path));
                }
            }
        }
    }
    println!("Here's The Items The Directory: \n {:#?}", files_array);
    files_array
}