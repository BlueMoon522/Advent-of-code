use std::{fs::File, io::Read};
fn err_corrector(line: Vec<i32>) -> bool {
    // let mut change_counter = line.len();
    // let mut second_change_counter = line.len();
    let mut checker = vec![0; line.len()];
    checker.clone_from(&line);
    // println!("Actual  checker{:?}", checker);
    // let loop_length = checker.len() - 1;
    // println!("loop lenght{}", loop_length);

    if condition_checker(line.to_vec())
        && (ascending_check(line.to_vec()) || descending_check(line.to_vec()))
    {
        return true;
    }

    // Try removing each element to see if the line becomes valid.
    for i in 0..line.len() {
        let mut modified_line = line.to_vec();
        modified_line.remove(i);

        if ascending_check(modified_line.to_vec())
            || descending_check(modified_line.to_vec()) && condition_checker(modified_line.to_vec())
        {
            return true;
        }
    }

    false

    // for i in 0..loop_length {
    //     // println!("Current i{}", i);
    //     checker.remove(i);
    //     // println!("checker now:{:?}", checker);
    //     // println!("checker length now:{:?}", checker.len());
    //     let mut changed_checker = vec![0; checker.len()];
    //     changed_checker.clone_from(&checker);
    //     // println!("Changed checker{:?}", changed_checker);
    //     checker.clone_from(&line);
    //     if descending_check(changed_checker.to_vec()) {
    //         change_counter = change_counter - 1;
    //     }
    //     if ascending_check(changed_checker.to_vec()) {
    //         second_change_counter = second_change_counter - 1;
    //     }
    // }
    // if change_counter <= 1 || second_change_counter <= 1 {
    //     println!("Flase ones{:?}", line);
    //     return true;
    // } else {
    //     return false;
    // }
}

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
fn condition_checker(line: Vec<i32>) -> bool {
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
    return true;
}
fn descending_check(mut line: Vec<i32>) -> bool {
    let mut copy_line = vec![0; line.len()];
    copy_line.clone_from(&line);
    line.sort(); //sorting to Ascending
    line.reverse(); //reversing the ascending order
    if line == copy_line {
        // println!("Original line{:?}", copy_line);
        // println!("{:?}", line);
        return true;
    } else {
        return false;
    }
}

fn main() {
    //storing all the contents of the input.txt in file
    let mut file = File::open("biginput.txt").expect("Cant open file");
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

    // let mut current_check = 0;
    // for value in &data {
    //     if err_corrector(value.to_vec()) == true {
    //         current_check = current_check + 1;
    //     }
    //     // println!("Ascending{}", ascending_check(value.to_vec()));
    //
    //     // println!("{:?}", value);
    // }
    let safe_count = data
        .iter()
        .filter(|line| err_corrector(line.to_vec().clone()))
        .count();

    println!("Number of safe reports: {}", safe_count);
    // println!("Current Check:{}", current_check);
}
