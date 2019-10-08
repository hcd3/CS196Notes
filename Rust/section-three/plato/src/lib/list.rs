use std::env;
use std::path::PathBuf;
use std::fs;
use super::opts::Opts;
use std::vec::Vec;
use std::fs::ReadDir;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::lib::list_item;

// Gets the current directory of the user
pub fn get_current_directory() -> PathBuf {
    let path = env::current_dir();
    match path {
        Ok(p) => return p,
        Err(e) => panic!(e)
    }
}

// Reads the contents of the current directory
pub fn get_directory_contents(directory: &PathBuf) -> Result<ReadDir, Error> {
   let paths = fs::read_dir(directory);
   match paths {
       Ok(p) => Ok(p),
       Err(e) => Err(e)
   }
}

