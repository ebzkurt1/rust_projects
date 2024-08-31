mod txt_reader;

use std::io::{self, Write};
use std::collections::HashMap;


fn count_characters(input: &str) -> HashMap<char, usize> {
    let mut character_counts = HashMap::new();

    for character in input.chars() {
        *character_counts.entry(character).or_insert(0) += 1;
    }

    character_counts
}

fn main() {
    print!("Please enter the filename: ");
    io::stdout().flush().unwrap();
    
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read line");

    let file_name = file_name.trim();

    match txt_reader::read_file_contents(file_name) {
        Ok(contents) => {
            let character_counts = count_characters(&contents);
            print!("Character counts:");
            for (char, count) in &character_counts {
                println!("'{}' occurs {} time(s)", char, count);
            }
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
