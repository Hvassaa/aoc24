use std::collections::HashMap;

use crate::read_lines_of_file;

fn split(p: u64) -> Option<(u64, u64)> {
    let s = p.to_string();
    let len = s.len();
    if len == 0 || len % 2 == 1 {
        None
    } else {
        let half = s.len() / 2;
        let left: u64 = s.chars().take(half).collect::<String>().parse().unwrap();
        let right: u64 = s.chars().skip(half).collect::<String>().parse().unwrap();

        Some((left, right))
    }
}

fn blink_n_times(n: usize) -> i64 {
    let lines = read_lines_of_file("11.txt");
    let mut line: Vec<u64> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut cache: HashMap<u64, u64> = HashMap::new();
    line.iter().for_each(|v| {
        cache.insert(*v, 1);
    });

    for i in 0..n {
        let mut new: HashMap<u64, u64> = HashMap::new();
        cache.keys().for_each(|k| {
            let count = *cache.get(k).unwrap();
            match *k {
                0 => {
                    let already_inserted_count = new.get(&1).unwrap_or(&0);
                    new.insert(1, count+ already_inserted_count);
                }
                _ => {
                    if let Some((left, right)) = split(*k) {
                        let already_inserted_count_left = new.get(&left).unwrap_or(&0);

                        new.insert(left, count + already_inserted_count_left);

                        let already_inserted_count_right = match new.get(&right) {
                            Some(a) => *a,
                            _ => 0,
                        };
                        new.insert(right, count + already_inserted_count_right);
                    } else {
                        let multed = (*k) * 2024;
                        let already_inserted_count = new.get(&multed).unwrap_or(&0);
                        new.insert(multed, count + already_inserted_count);
                    }
                }
            };
        });
        cache = new;
    }
    let res: u64 = cache.values().sum();
    res as i64
}


pub fn first() -> i64 {
    blink_n_times(25)
}

pub fn second() -> i64 {
    blink_n_times(75)
}
