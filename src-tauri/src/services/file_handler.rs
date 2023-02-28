use infer::Type;
use std::{
    fs::{read_dir, ReadDir},
    path::PathBuf,
};

const LNK_MIME_TYPE: &str = "application/x-ms-shortcut";

// 4c 00 00 00 01 14 02 00 00 00 00 00 c0 00 00 00 L...............
// 00 00 00 46 8b 00 08 00 11 00 00 00 9f f6 c0 fd ...F............
// bd cb cc 01 42 94 c9 85 92 cc cc 01 5d 6c d7 d2 ....B........l..
// c8 cc cc 01 00 00 00 00 00 00 00 00 01 00 00 00 ................

fn custom_matcher(buf: &[u8]) -> bool {
    return buf.len() >= 3 && buf[0] == 0x4c && buf[1] == 0x00 && buf[2] == 0x00;
}

fn get_custom_infer() -> infer::Infer {
    let mut info = infer::Infer::new();
    info.add(LNK_MIME_TYPE, "lnk", custom_matcher);
    info
}

fn get_matcher_type(mime_type: Type) -> infer::MatcherType {
    match mime_type.mime_type() {
        LNK_MIME_TYPE => infer::MatcherType::App,
        _ => mime_type.matcher_type(),
    }
}

pub fn get_dir_items<'a>(path: &str) -> Vec<PathBuf> {
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

    // TODO: make this a map
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
