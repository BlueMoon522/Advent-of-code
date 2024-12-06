use std::{fs::File, io::Read};
fn main() {
    //storing all the contents of the input.txt in file
    let mut file = File::open("input.txt").expect("Cant open file");
    //creating a string to store file content
    let mut contents = String::new();
    //reading file as string and inserting it to contents
    file.read_to_string(&mut contents).expect("Cant read file");
    let data: Vec<Vec<i32>> = contents
        .lines() // Split the string into lines
        .map(|line| {
            line.split_whitespace() // Split each line into words
                .map(|num| num.parse::<i32>().expect("Invalid number")) // Parse each word into an i32
                .collect() // Collect the numbers into a Vec<i32>
        })
        .collect();
    println!("{:?}", data);
}
