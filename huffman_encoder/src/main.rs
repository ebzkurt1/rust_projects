mod txt_reader;

use std::io::{self, Write};


fn main() {
    print!("Please enter the filename: ");
    io::stdout().flush().unwrap();
    
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read line");

    let file_name = file_name.trim();

    match txt_reader::read_file_contents(file_name) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
