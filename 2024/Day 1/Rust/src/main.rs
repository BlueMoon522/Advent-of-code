use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the file
    let path = Path::new("input.txt");
    let file = File::open(&path)?;

    // Initialize vectors for columns
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    // Read file line by line
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 2 {
            first_column.push(words[0].to_string());
            second_column.push(words[1].to_string());
        }
    }

    // Print results
    println!("First Column: {:?}", first_column);
    println!("Second Column: {:?}", second_column);

    Ok(())
}
