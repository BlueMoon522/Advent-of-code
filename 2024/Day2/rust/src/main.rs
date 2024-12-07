use std::{fs::File, io::Read};

fn ascending_check(mut line: Vec<i32>) -> bool {
    let mut copy_line = vec![0; line.len()];
    copy_line.clone_from(&line);
    line.sort();
    if line == copy_line {
        for i in 0..line.len() {
            if i != line.len() - 1 {
                let mut current_diff = line[i] - line[i + 1];
                if current_diff < 0 {
                    current_diff = current_diff * -1;
                }

                // println!("current_diff:{}", current_diff);
                if line[i] == line[i + 1] {
                    return false;
                }
                if current_diff < 1 || current_diff > 3 {
                    return false;
                }
            }
        }
        // println!("Original line{:?}", copy_line);
        // println!("{:?}", line);
        return true;
    } else {
        return false;
    }
}
fn descending_check(mut line: Vec<i32>) -> bool {
    let mut copy_line = vec![0; line.len()];
    copy_line.clone_from(&line);
    line.sort(); //sorting to Ascending
    line.reverse(); //reversing the ascending order
    if line == copy_line {
        for i in 0..line.len() {
            if i != line.len() - 1 {
                let mut current_diff = line[i] - line[i + 1];
                if current_diff < 0 {
                    current_diff = current_diff * -1;
                }

                // println!("current_diff:{}", current_diff);
                if line[i] == line[i + 1] {
                    return false;
                }
                if current_diff < 1 || current_diff > 3 {
                    return false;
                }
            }
        }
        // println!("Original line{:?}", copy_line);
        // println!("{:?}", line);
        return true;
    } else {
        return false;
    }
}

fn main() {
    //storing all the contents of the input.txt in file
    let mut file = File::open("input.txt").expect("Cant open file");
    //creating a string to store file content
    let mut contents = String::new();
    //reading file as string and inserting it to contents
    //..inserting the values inside a vector as rust only allows the arrays to be of fixed size
    file.read_to_string(&mut contents).expect("Cant read file");
    let data: Vec<Vec<i32>> = contents
        .lines() // Split the string into lines
        .map(|line| {
            line.split_whitespace() // Split each line into words
                .map(|num| num.parse::<i32>().expect("Invalid number")) // Parse each word into an i32
                .collect() // Collect the numbers into a Vec<i32>
        })
        .collect();
    // println!("{:?}", data); //for all the current data inside this vector

    let mut current_check = 0;
    for value in &data {
        if descending_check(value.to_vec()) == true || ascending_check(value.to_vec()) == true {
            current_check = current_check + 1
        }
        // println!("Ascending{}", ascending_check(value.to_vec()));

        // println!("{:?}", value);
    }
    println!("Current Check:{}", current_check);
}
