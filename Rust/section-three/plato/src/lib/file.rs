use std::fs;
use fs::Metadata;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};
use users;
use std::ffi::OsString;
use pretty_bytes::converter::convert;
use pathdiff::diff_paths;
use std::path::PathBuf;

// Determines if a file is a hidden file or not.
pub fn is_hidden_file(file: &PathBuf, current_dir: &PathBuf) -> bool{
    let relative_path = diff_paths(&file, &current_dir).unwrap();
    let file = relative_path.to_str().unwrap();
    return file.starts_with(".")
}

// Get the name of the owning user of this file
pub fn get_owning_user(meta: &Metadata) -> OsString {
    let user = users::get_user_by_uid(meta.uid()).unwrap();
    return user.name().to_os_string();
}

// Get the time this file was last modified
pub fn get_time_last_modified(meta: &Metadata) -> DateTime<Local> {
    let modified: DateTime<Local> = DateTime::from(meta.modified().unwrap());
    modified
}

// Get the file size
pub fn get_file_size(meta: &Metadata) -> u64 {
    if meta.is_dir() {
        0
    }
    else {
        meta.size()
    }
}

// Get the file size but formatted to easily be read
// Uses the rust pretty bytes crate 
// https://github.com/banyan/rust-pretty-bytes
pub fn get_human_file_size(meta: &Metadata) -> String {
    let num = get_file_size(meta);
    return convert(num as f64);
}

