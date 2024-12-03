use crate::read_lines_of_file;

pub fn first() -> i32 {
    let lines = read_lines_of_file("1.txt");

    let pairs: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let mut a = line.split_whitespace();
            let first = a.next().unwrap().to_string();
            let last = a.last().unwrap().to_string();
            (first.parse::<i32>().unwrap(), last.parse::<i32>().unwrap())
        })
        .collect();

    let mut lefts: Vec<i32> = pairs.iter().map(|p| p.0).collect();
    let mut rights: Vec<i32> = pairs.iter().map(|p| p.1).collect();
    lefts.sort();
    rights.sort();
    let res: i32 = lefts.iter().zip(rights).map(|(a, b)| (a - b).abs()).sum();
    res
}

pub fn second() -> i32 {
    let lines = read_lines_of_file("1.txt");

    let pairs: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let mut a = line.split_whitespace();
            let first = a.next().unwrap().to_string();
            let last = a.last().unwrap().to_string();
            (first.parse::<i32>().unwrap(), last.parse::<i32>().unwrap())
        })
        .collect();

    let lefts: Vec<i32> = pairs.iter().map(|p| p.0).collect();
    let rights: Vec<i32> = pairs.iter().map(|p| p.1).collect();

    let res: i32 = lefts
        .iter()
        .map(|n| {
            let a = rights.iter().filter(|p| n == *p).count() as i32;
            a * n
        })
        .sum();
    res
}
