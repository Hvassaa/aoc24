mod day1;
mod day2;
mod day3;
mod day4;

use std::fs;

fn read_lines_of_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.lines();
    return lines.map(|line| line.to_string()).collect();
}

fn main() {
    let res = day4::second();
    println!("{res}");
}
