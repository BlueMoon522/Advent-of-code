use std::{fs::File, io::Read};

fn get_string() -> String {
    let mut file = File::open("biginput.txt").expect("Cant open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cant read the file");
    return contents;
}
fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}
fn main() {
    let string_all = get_string();
    println!("String:{}", string_all);
    let pattern = "mul(";

    //finding all the muls inside the string
    let mut matches = Vec::new();

    //iterating through the string
    let chars = string_all.chars().collect::<Vec<_>>();
    let len = chars.len();

    let mut i = 0;

    while i < len {
        if string_all[i..].starts_with(pattern) {
            let start = i + pattern.len();
            let mut j = start;

            //first integer
            while j < len && chars[j].is_numeric() {
                j = j + 1;
            }
            if j == start || j >= len || chars[j] != ',' {
                i = i + 1;
                continue;
            }
            let first_num: i32 = string_all[start..j]
                .parse()
                .expect("Failed to parse the first integer");
            let mut k = j + 1;
            while k < len && chars[k].is_numeric() {
                k = k + 1;
            }
            if k == j + 1 || k >= len || chars[k] != ')' {
                i = i + 1;
                continue;
            }
            let second_num: i32 = string_all[j + 1..k]
                .parse()
                .expect("Failed to parse the second integer");
            matches.push((first_num, second_num));
            i = k + 1;
            //second integer
        } else {
            i = i + 1;
        }
    }
    println!("{:?}", matches);
    let mut sum = 0;
    for (a, b) in matches {
        let result = mul(a, b);
        println!("mul({}, {}) = {}", a, b, result);
        sum = sum + result;
    }
    println!("Final sum:{}", sum);
}
