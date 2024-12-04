use crate::read_lines_of_file;
use regex::Regex;

fn count_matches(line: &String) -> i32 {
    let xmas_re = Regex::new(r"SAMX").unwrap();
    let samx_re = Regex::new(r"XMAS").unwrap();

    let xmas = xmas_re.find_iter(line).count();
    let samx = samx_re.find_iter(line).count();
    (xmas + samx) as i32
}

pub fn first() -> i32 {
    let lines = read_lines_of_file("4.txt");

    let horizontal: i32 = lines.iter().map(count_matches).sum();

    let vertical: i32 = (0..lines.first().unwrap().len())
        .map(|idx| {
            lines
                .iter()
                .map(|line| line.chars().nth(idx).unwrap())
                .collect::<String>()
        })
        .map(|line| count_matches(&line))
        .sum();

    let width = lines.first().unwrap().len();

    let mut sum = 0;
    for i in 0..lines.len() {
        for j in 0..width {
            let mut d = String::from("");
            for k in 0..4 {
                if let Some(line) = lines.get(k + i) {
                    if let Some(c) = line.chars().nth(k + j) {
                        d += &c.to_string();
                    }
                }
            }
            if d == "XMAS" || d == "SAMX" {
                sum += 1;
            }
        }
    }

    for i in 0..lines.len() {
        for j in 0..width {
            let mut d = String::from("");
            for k in 0..4 {
                if let Some(line) = lines.get(k + i) {
                    if let Some(c) = line.chars().rev().nth(k + j) {
                        d += &c.to_string();
                    }
                }
            }
            if d == "XMAS" || d == "SAMX" {
                sum += 1;
            }
        }
    }

    horizontal + vertical + sum
}

pub fn second() -> i32 {
    let lines = read_lines_of_file("4.txt");

    let a_indices: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == 'A')
                .map(|(x, _c)| (y, x))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    a_indices
        .iter()
        .filter(|(y, x)| {
            if *y == 0 || *x == 0 {
                return false;
            }

            let over = lines.get(y - 1);
            let under = lines.get(y + 1);
            if over.is_none() || under.is_none() {
                return false;
            }

            let over = over.unwrap();
            let under = under.unwrap();
            let top_left = over.chars().nth(x - 1);
            let top_right = over.chars().nth(x + 1);
            let bottom_left = under.chars().nth(x - 1);
            let bottom_right = under.chars().nth(x + 1);

            if top_left.is_none()
                || top_right.is_none()
                || bottom_left.is_none()
                || bottom_right.is_none()
            {
                return false;
            }

            let top_left = top_left.unwrap();
            let top_right = top_right.unwrap();
            let bottom_left = bottom_left.unwrap();
            let bottom_right = bottom_right.unwrap();

            let left_ms = top_left == 'M' && bottom_right == 'S';
            let left_sm = top_left == 'S' && bottom_right == 'M';
            let right_ms = top_right == 'M' && bottom_left == 'S';
            let right_sm = top_right == 'S' && bottom_left == 'M';

            (left_ms || left_sm) && (right_ms || right_sm)
        })
        .count() as i32
}
