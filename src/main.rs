mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use std::fs;

fn read_lines_of_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.lines();
    return lines.map(|line| line.to_string()).collect();
}

fn main() {
    let res = day11::second();
    println!("{res}");

    // let a: f64 = 1234.;
    // let b= a.log10().floor() as usize;
    // let c = a / (10. as f64).powi((b - 1) as i32);
    // println!("{c}");
}
