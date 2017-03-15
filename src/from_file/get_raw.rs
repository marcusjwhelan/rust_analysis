use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


/// Read the file name and Return a raw string from file.
pub fn csv_string (file_name: &str) -> String {
  let path = Path::new(file_name);
  let mut file = match File::open(&path) {
    Err(why) => panic!("Could not open file because: {}", why.description()),
    Ok(file) => file,
  };
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}:", why.description()),
    Ok(contents) => contents,
  };
  
  s  
}
  