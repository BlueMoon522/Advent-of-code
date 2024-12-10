use std::fs::File;
use std::io::{self, prelude::*};

fn solve(l: i32, w: i32, h: i32) -> (i32, i32, i32) {
    let extra;
    let wrapper;
    if (l * w) <= (w * h) && (l * w) <= (h * l) {
        wrapper = (2 * l) + (2 * w);
        extra = l * w
    } else if (w * h) <= (l * w) && (w * h) <= (h * l) {
        wrapper = (2 * w) + (2 * h);
        extra = w * h
    } else {
        wrapper = (2 * h) + (2 * l);
        extra = h * l
    }
    let area = (2 * l * w) + (2 * w * h) + (2 * h * l);
    let ribbon = wrapper + (l * w * h);

    (area, extra, ribbon)
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Cant open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("couldnot write to string");
    let mut l_vec = Vec::new();
    let mut w_vec = Vec::new();
    let mut h_vec = Vec::new();

    //read the strings in contents and split it based on * and collect the result vector
    for line in contents.lines() {
        let words: Vec<&str> = line.split("x").collect();
        if words.len() == 3 {
            l_vec.push(words[0].to_string());
            w_vec.push(words[1].to_string());
            h_vec.push(words[2].to_string());
        }
    }

    let intl_vec: Vec<i32> = l_vec
        .iter()
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    let intw_vec: Vec<i32> = w_vec
        .iter()
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    let inth_vec: Vec<i32> = h_vec
        .iter()
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    println!("{}", intl_vec.len());
    let mut final_total = 0;
    let mut total_ribbon = 0;
    for i in 0..l_vec.len() {
        let (area, extra, ribbon) = solve(intl_vec[i], intw_vec[i], inth_vec[i]);
        final_total = final_total + area + extra;
        total_ribbon = ribbon + total_ribbon;
    }
    println!("Final total:{}", final_total);
    println!("Ribbon total:{}", total_ribbon);
    Ok(())
}
