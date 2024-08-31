use std::fs::File;
use std::io::{self, Read};
use std::path::Path;


pub fn read_file_contents<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}