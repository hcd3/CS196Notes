use std::fs;
use fs::Metadata;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use users;
use std::ffi::OsString;
use pretty_bytes::converter::convert;

// Takes in the raw permission decimal value and returns the 
// the permissions string using the letter format. Grabs the 
// permissions for user, group, and other. The S_IRUSR constants
// are used for doing bitwise and operations to check if those bits
// are set in the file's permissions. 
fn parse_permissions(is_dir: bool, mode: u32) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
    let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
    let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
    if is_dir {
        ["d".to_string(), user, group, other].join("")
    }
    else {
        ["-".to_string(), user, group, other].join("")
    }
}

// Uses bitwise and operator to fetch what permissions are set for this object.
fn triplet(mode: u32, read: u16, write: u16, execute: u16) -> String {
    match (mode & u32::from(read), mode & u32::from(write), mode & u32::from(execute)) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }.to_string()
}

// Get the permissions using the associate file metadata
pub fn get_permissions(meta: &Metadata) ->  String {
    // Fill me out !
}

// Get the name of the owning user of this file
pub fn get_owning_user(meta: &Metadata) -> OsString {
    // Fill me out !
}

// Get the time this file was last modified
pub fn get_time_last_modified(meta: &Metadata) -> DateTime<Local> {
    // Fill me out !
    DateTime::from(meta.modified().expect("wrong Input"))
}

// Get the file size
pub fn get_file_size(meta: &Metadata) -> u64 {
    // Fill me out !
    meta.size()
}

// Get the file size but formatted to easily be read
// Uses the rust pretty bytes crate 
// https://github.com/banyan/rust-pretty-bytes
pub fn get_human_file_size(meta: &Metadata) -> String {
    // Fill me out !
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;
    use std::io::Write;
    use std::time::SystemTime;
    use chrono::{DateTime, Local};
    use users::{get_user_by_uid, get_current_uid};


    #[test]
    fn test_get_human_file_size() {

        // Setup
        let mut f = File::create("first.txt").unwrap();
        f.write_all(b"Hello, world!").unwrap();
        let file_path = PathBuf::from("first.txt");
        let meta = file_path.metadata().unwrap();
        let human_size = get_human_file_size(&meta);
        
        // Test
        assert_eq!("13 B", human_size);

        // Teardown
        fs::remove_file("first.txt").expect("could not delete file");
    }

    #[test]
    fn test_get_file_size() {

        // Setup
        let mut f = File::create("second.txt").unwrap();
        f.write_all(b"Hello, world!").unwrap();
        let file_path = PathBuf::from("second.txt");
        let meta = file_path.metadata().unwrap();
        let size = get_file_size(&meta);

        // Test
        assert_eq!(13, size);

        // Teardown
        fs::remove_file("second.txt").expect("could not delete file");
    }

    #[test]
    fn test_get_time_last_modified() {

        // Setup
        let mut f = File::create("third.txt").unwrap();
        f.write_all(b"Hello, world!").unwrap();
        let time = SystemTime::now();
        let date_time: DateTime<Local> = DateTime::from(time);
        let file_path = PathBuf::from("third.txt");
        let meta = file_path.metadata().unwrap();
        let time_last_modified = get_time_last_modified(&meta);

        // Test
        assert_eq!(time_last_modified.timestamp(), date_time.timestamp());

        // Teardown
        fs::remove_file("third.txt").expect("could not delete file");
    }

    #[test]
    fn test_get_owning_user() {

        // Setup
        let mut f = File::create("fourth.txt").unwrap();
        f.write_all(b"Hello, world!").unwrap();
        let file_path = PathBuf::from("fourth.txt");
        let meta = file_path.metadata().unwrap();
        let owning_user = get_owning_user(&meta);
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let name = user.name();

        // Test
        assert_eq!(owning_user.into_string(), name.to_os_string().into_string());

        // Teardown
        fs::remove_file("fourth.txt").expect("could not delete file");
    }

    #[test]
    fn test_get_permissions() {

        // Setup
        let mut f = File::create("fifth.txt").unwrap();
        f.write_all(b"Hello, world!").unwrap();
        let file_path = PathBuf::from("fifth.txt");
        let meta = file_path.metadata().unwrap();
        let permissions = get_permissions(&meta);
        let answer = String::from("-rw-r--r--");

        // Test
        assert_eq!(answer, permissions);

        // Teardown
        fs::remove_file("fifth.txt").expect("could not delete file");
    }
}
