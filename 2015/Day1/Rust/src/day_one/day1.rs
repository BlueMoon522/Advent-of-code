use std::fs::File;
use std::io::prelude::*;
pub fn floor() {
    //read the contents in the file using the command: cargo run -- the input.txt
    //this only shows all the contents of the file in the command line
    // let args: Vec<String> = env::args().collect();
    //
    // let _query = &args[1];
    // let file_path = &args[2];
    // let contents = fs::read_to_string(file_path).expect("Why can't i read this file??");
    // println!("{contents}");
    // another way to do it
    let mut file = File::open("input.txt").expect("Cant open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cant read file");
    // println!("{}", contents);

    let characters: Vec<char> = contents.chars().collect();
    println!("length:{}", characters.len());

    let mut current_floor = 0;
    // To get the final Floor
    // for (_, value) in characters.iter().enumerate() {
    //     if *value == '(' {
    //         current_floor = current_floor + 1;
    //     } else if *value == ')' {
    //         current_floor = current_floor - 1;
    //     }
    // }
    // println!("Current floor:{}", current_floor);
    // ******************************************************
    // To get the first time taken to basement i.e -1 case
    for (index, value) in characters.iter().enumerate() {
        if current_floor != -1 {
            if *value == '(' {
                current_floor = current_floor + 1;
            } else if *value == ')' {
                current_floor = current_floor - 1;
            }
        } else {
            println!("At basement in index {}", index);
            break;
        }
        // could learn more about using this same code twice
    }
}
