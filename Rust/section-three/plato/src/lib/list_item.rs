use std::fs;
use std::ffi::OsString;
use chrono::{DateTime, Local};
use super::file;
use std::collections::HashMap;
use std::path::PathBuf;
use std::vec::Vec;
use pathdiff::diff_paths;

use crate::lib::list;

#[derive(Debug)]
pub struct ListItem {
    pub owning_user: OsString,
    pub file_size: String,
    pub time_last_modified: DateTime<Local>,
    pub name: String
}

pub fn get_file_data(path: PathBuf, directory: &PathBuf, flags: &HashMap<&str, bool>) -> ListItem {
    // Fill me out
}

// Uses the current directory and the associated metadata with a file to
// create list item structs for each file. Depending on the flags, the list item 
// content might be different for file size. 
pub fn create_list_items(current_dir: PathBuf, flags: &HashMap<&str, bool>) -> Vec<ListItem> {
    // Fill me out   
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::list;
    use crate::lib::file;
    use std::path::PathBuf;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn test_get_file_data_human() {

        // Setup
        let mut f = File::create("first.txt").unwrap();
        f.write_all(b"Hello, world!").unwrap();
        let file_path = PathBuf::from("first.txt");
        let meta = file_path.metadata().unwrap();
        let human_size = file::get_human_file_size(&meta);
        let time = file::get_time_last_modified(&meta);
        let owner = file::get_owning_user(&meta);
        let file_name = String::from("first.txt");

        let solution = ListItem {
            owning_user: owner,
            file_size: human_size,
            time_last_modified: time,
            name: file_name
        };
        let mut args = HashMap::new();
        args.insert("long", true);
        args.insert("human", true);

        let path = list::get_current_directory();
        let test = get_file_data(file_path, &path, &args);

        // Test
        assert_eq!(solution.owning_user, test.owning_user);
        assert_eq!(solution.file_size, test.file_size);
        assert_eq!(solution.time_last_modified.timestamp(), test.time_last_modified.timestamp());
        assert_eq!(solution.name, test.name);

        // Teardown
        fs::remove_file("first.txt").expect("could not remove file");
    }

    // This test ASSUMES you have the get_file_data function working
    #[test]
    fn test_create_list_items_directory() {

        // Setup
        fs::create_dir("./test_directory_one").expect("could not create directory");
        let path_buf = PathBuf::from("./test_directory_one");
        File::create("./test_directory_one/a.txt").expect("could not create file");
        File::create("./test_directory_one/b.txt").expect("could not create file");

        let mut flags = HashMap::new();
        flags.insert("hidden", false);
        flags.insert("long", false);
        flags.insert("human", false);
        let base = PathBuf::from("./test_directory_one");
        let file_a = PathBuf::from("./test_directory_one/a.txt");
        let file_b = PathBuf::from("./test_directory_one/b.txt");
        let mut solutions = Vec::new();
        solutions.push(get_file_data(file_b, &path_buf, &flags));
        solutions.push(get_file_data(file_a, &path_buf, &flags));

        let tests = create_list_items(base, &flags);

        // Test
        for i in 1..2 {
            assert_eq!(solutions[i].owning_user, tests[i].owning_user);
            assert_eq!(solutions[i].file_size, tests[i].file_size);
            assert_eq!(solutions[i].time_last_modified.timestamp(),
                tests[i].time_last_modified.timestamp());
            assert_eq!(solutions[i].name, tests[i].name);
        }

        // Teardown
        fs::remove_dir_all("./test_directory_one")
            .expect("could not remove directory");
    }

    #[test]
    fn test_create_list_items_file() {

        // Setup
        fs::create_dir("./test_directory_two").expect("could not create directory");
        File::create("./test_directory_two/a.txt").expect("could not create file");
        File::create("./test_directory_two/b.txt").expect("could not create file");

        let mut flags = HashMap::new();
        flags.insert("hidden", false);
        flags.insert("long", false);
        flags.insert("human", false);
        let file_a_abs = fs::canonicalize(PathBuf::from("./test_directory_two/a.txt"))
            .unwrap();
        let mut solutions = Vec::new();
        let current_dir = list::get_current_directory();
        solutions.push(get_file_data(file_a_abs, &current_dir, &flags));

        let file_a_abs = fs::canonicalize(PathBuf::from("./test_directory_two/a.txt"))
            .unwrap();
        let tests = create_list_items(file_a_abs, &flags);

        // Test
        assert_eq!(solutions[0].owning_user, tests[0].owning_user);
        assert_eq!(solutions[0].file_size, tests[0].file_size);
        assert_eq!(solutions[0].time_last_modified.timestamp(),
            tests[0].time_last_modified.timestamp());
        assert_eq!(solutions[0].name, tests[0].name);

        // Teardown
        fs::remove_dir_all("./test_directory_two")
            .expect("could not remove directory");
    }

    #[test]
    fn test_create_list_items_hidden() {

        // Setup
        fs::create_dir("./test_directory_three").expect("could not create directory");
        let path_buf = PathBuf::from("./test_directory_three");
        File::create("./test_directory_three/a.txt").expect("could not create file");
        File::create("./test_directory_three/b.txt").expect("could not create file");
        File::create("./test_directory_three/.hidden").expect("could not create file");

        let mut flags = HashMap::new();
        flags.insert("hidden", true);
        flags.insert("long", false);
        flags.insert("human", false);

        let base = PathBuf::from("./test_directory_three");
        let file_a = PathBuf::from("./test_directory_three/a.txt");
        let file_b = PathBuf::from("./test_directory_three/b.txt");
        let hidden = PathBuf::from("./test_directory_three/.hidden");

        let mut solutions = Vec::new();
        solutions.push(get_file_data(hidden, &path_buf, &flags));
        solutions.push(get_file_data(file_b, &path_buf, &flags));
        solutions.push(get_file_data(file_a, &path_buf, &flags));

        let tests = create_list_items(base, &flags);

        // Test
        for i in 1..3 {
            assert_eq!(solutions[i].owning_user, tests[i].owning_user);
            assert_eq!(solutions[i].file_size, tests[i].file_size);
            assert_eq!(solutions[i].time_last_modified.timestamp(),
                tests[i].time_last_modified.timestamp());
            assert_eq!(solutions[i].name, tests[i].name);
        }

        // Teardown
        fs::remove_dir_all("./test_directory_three")
            .expect("could not remove directory");
    }

    #[test]
    fn test_create_list_items_unhidden() {

        // Setup
        fs::create_dir("./test_directory_four").expect("could not create directory");
        File::create("./test_directory_four/a.txt").expect("could not create file");
        File::create("./test_directory_four/b.txt").expect("could not create file");
        File::create("./test_directory_four/.hidden").expect("could not create file");

        let mut flags = HashMap::new();
        flags.insert("hidden", false);
        flags.insert("long", false);
        flags.insert("human", false);

        let base = PathBuf::from("./test_directory_four");

        let tests = create_list_items(base, &flags);

        // Test
        assert_eq!(2, tests.len());

        // Teardown
        fs::remove_dir_all("./test_directory_four")
            .expect("could not remove directory");

    }

    #[test]
    fn test_create_list_items_default() {

        // Setup
        File::create("a.txt").expect("could not create file");
        File::create("b.txt").expect("could not create file");
        File::create("c.txt").expect("could not create file");
        let path_buf = list::get_current_directory();

        let mut flags = HashMap::new();
        flags.insert("hidden", false);
        flags.insert("long", false);
        flags.insert("human", false);

        let file_a = PathBuf::from("a.txt");
        let file_b = PathBuf::from("b.txt");
        let file_c = PathBuf::from("c.txt");

        let mut solutions = Vec::new();
        solutions.push(get_file_data(file_c, &path_buf, &flags));
        solutions.push(get_file_data(file_b, &path_buf, &flags));
        solutions.push(get_file_data(file_a, &path_buf, &flags));

        let tests = create_list_items(path_buf, &flags);
        let mut filtered = Vec::new();
        for item in tests {
            if item.name.contains(".txt") {
                filtered.push(item);
            }
        }

        // Test
        for i in 1..3 {
            assert_eq!(solutions[i].owning_user, filtered[i].owning_user);
            assert_eq!(solutions[i].file_size, filtered[i].file_size);
            assert_eq!(solutions[i].time_last_modified.timestamp(),
                filtered[i].time_last_modified.timestamp());
            assert_eq!(solutions[i].name, filtered[i].name);
        }

        // Teardown
        fs::remove_file("a.txt").expect("could not remove file");
        fs::remove_file("b.txt").expect("could not remove file");
        fs::remove_file("c.txt").expect("could not remove file");
    }
}

