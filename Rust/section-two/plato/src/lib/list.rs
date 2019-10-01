use std::env;
use std::path::PathBuf;
use std::fs;
use std::fs::ReadDir;
use std::io::Error;

// Gets the current directory of the user
pub fn get_current_directory() -> PathBuf {
    // Fill me out !
}

// Reads the contents of the current directory
pub fn get_directory_contents(directory: &PathBuf) -> Result<ReadDir, Error> {
    // Fill me out !
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;
    use std::path::Path;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_get_current_directory() {

        // Setup
        fs::create_dir("./test_one").expect("could not create directory");
        let path = Path::new("./test_one");
        env::set_current_dir(&path).expect("could not set current working directory");
        let result = get_current_directory().into_os_string().into_string().unwrap();

        // Test
        assert_eq!(true, result.contains("/test_one"));

        // Teardown
        let upper_path = Path::new("..");
        env::set_current_dir(&upper_path).expect("could not set current working directory");
        fs::remove_dir_all("./test_one").expect("could not remove directory");
    }

    #[test]
    fn test_get_directory_contents() {

        // Setup
        fs::create_dir("./test_two").expect("could not create directory");
        let path_buf = PathBuf::from("./test_two");
        File::create("./test_two/a.txt").expect("could not create file");
        File::create("./test_two/b.txt").expect("could not create file");
        let a_path = PathBuf::from("./test_two/a.txt");
        let b_path = PathBuf::from("./test_two/b.txt");
        let result = get_directory_contents(&path_buf).unwrap();
        let mut paths: Vec<_> = result.map(|res| res.unwrap().path()).collect();
        paths.sort();

        // Test
        assert_eq!(a_path, paths[0]);
        assert_eq!(b_path, paths[1]);

        // Teardown
        fs::remove_dir_all("./test_two").expect("could not remove directory");
    }
}
