use std::{fs::File, io::Read};

//to check both of this function are very similar except for the line where u reverse the
//things,may be that can be changed.
fn ascending_check(mut line: Vec<i32>) -> bool {
    // let mut del_counter = 0;
    let mut error_tol = 0;
    let mut copy_line = vec![0; line.len()];
    copy_line.clone_from(&line);
    for i in 0..line.len() {
        let mut rem_next_2check = vec![0; line.len()];
        rem_next_2check.clone_from(&line);
        // println!("Current remnext :{:?}", rem_next_2check);
        rem_next_2check.remove(i);
        if rem_next_2check.windows(2).all(|w| w[0] < w[1]) {
            return true; // If the remaining vector is strictly ascending, return true
        }
        return false;
        // println!("Current remnext after remove :{:?}", rem_next_2check);
        // let mut sort_check = vec![0; rem_next_2check.len()];
        // sort_check.clone_from(&rem_next_2check);
        // println!("Current sort_check :{:?}", sort_check);

        // println!("Current ascend remnext :{:?}", rem_next_2check);
        // rem_next_2check.sort();
        // println!("Current  sorted rem_next_2check :{:?}", rem_next_2check);
        // if rem_next_2check != sort_check {
        //     del_counter = del_counter + 1;
        //     println!("Added to asc del_counter{}", del_counter);
        // }
        // if del_counter > 1 {
        //     println!("in ascending");
        //     // println!("Current ascendremnext :{:?}", rem_next_2check);
        //     // println!("Current ascendremnext after remove :{:?}", rem_next_2check);
        //     // println!("Current ascendsort_check :{:?}", sort_check);
        //     // println!("returned false");
        //     return false;
        // }
    }
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
                    error_tol = error_tol + 1;

                    // return false;
                }
                if current_diff < 1 || current_diff > 3 {
                    error_tol = error_tol + 1;
                    // return false;
                }
            }
        }
        // println!("Original line{:?}", copy_line);
        // println!("{:?}", line);
        if error_tol > 1 {
            return false;
        } else {
            return true;
        }
    } else {
        return false;
    }
}
fn descending_check(mut line: Vec<i32>) -> bool {
    // let mut del_counter = 0;
    let mut error_tol = 0;
    let mut copy_line = vec![0; line.len()];
    copy_line.clone_from(&line);
    for i in 0..line.len() {
        let mut rem_next_2check = vec![0; line.len()];
        rem_next_2check.clone_from(&line);
        // println!("Current remnext :{:?}", rem_next_2check);
        rem_next_2check.remove(i);
        if rem_next_2check.windows(2).all(|w| w[0] > w[1]) {
            return true; // If the remaining vector is strictly ascending, return true
        }
        return false;
        // println!("Current remnext after remove :{:?}", rem_next_2check);
        // let mut sort_check = vec![0; rem_next_2check.len()];
        // sort_check.clone_from(&rem_next_2check);
        // // println!("Current sort_check :{:?}", sort_check);
        //
        // println!("Current descremnext :{:?}", rem_next_2check);
        // rem_next_2check.sort();
        // rem_next_2check.reverse();
        // // println!("Current  sorted rem_next_2check :{:?}", rem_next_2check);
        // if rem_next_2check != sort_check {
        //     del_counter = del_counter + 1;
        //     println!("Added to  desc delcounter{}", del_counter);
        // }
        // if del_counter > 1 {
        //     println!("in descending");
        //     // println!("Current descremnext :{:?}", rem_next_2check);
        //     // println!("Current descremnext after remove :{:?}", rem_next_2check);
        //     // println!("Current descsort_check :{:?}", sort_check);
        //     // println!("returned false");
        //     return false;
        // }
    }
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
                    error_tol = error_tol + 1;
                    // return false;
                }
                if current_diff < 1 || current_diff > 3 {
                    error_tol = error_tol + 1;
                    // return false;
                }
            }
        }
        if error_tol > 1 {
            return false;
        } else {
            return true;
        }
        // println!("Original line{:?}", copy_line);
        // println!("{:?}", line);
        // return true;
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
    // let mut false_check = 0;
    for value in &data {
        if ascending_check(value.to_vec()) == true || descending_check(value.to_vec()) == true {
            current_check = current_check + 1
        }
        // if ascending_check(value.to_vec()) == false && descending_check(value.to_vec()) == false {
        //     false_check = false_check + 1
        // }
        // println!("Ascending{}", ascending_check(value.to_vec()));

        // println!("{:?}", value);
    }
    println!("contents length{:?}", data.len());
    println!("Current safe:{}", current_check);
    // println!("Current unsafe:{}", false_check);
}
