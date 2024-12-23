use crate::read_lines_of_file;
use regex::Regex;

pub fn first() -> i64 {
    let lines = read_lines_of_file("3.txt");

    let line = lines.first().unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let res: i64 = re
        .captures_iter(line)
        .map(|capture| {
            let first = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let second = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
            first * second
        })
        .sum();

    res
}

pub fn second() -> i64 {
    let lines = read_lines_of_file("3.txt");

    let line = lines.first().unwrap();

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut state = true;
    let res: i64 = re
        .captures_iter(line)
        .map(|capture| {
            let mads = capture.get(0).unwrap().as_str();
            if mads.contains("don't") {
                state = false;
            } else if mads.contains("do") {
                state = true;
            } else if state {
                let first = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let second = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
                return first * second;
            }
            0
        })
        .sum();

    res
}
