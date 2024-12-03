mod day1;
mod day2;
mod day3;

use std::fs;

fn read_lines_of_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.lines();
    return lines.map(|line| line.to_string()).collect();
}

fn main() {
    //let res = day3::first();
    let res = day3::second();
    println!("{res}");
}
