use crate::read_lines_of_file;

fn get_diffs(l: Vec<i32>) -> Vec<i32> {
    let mut a = l.clone();
    let mut b = l.clone();
    a.push(0);
    b.insert(0, 0);
    let mut diffs: Vec<i32> = a.iter().zip(b).map(|(a, b)| (a - b)).collect();

    diffs.remove(0);
    diffs.remove(diffs.len() - 1);
    diffs
}

fn asd(line: Vec<i32>) -> bool {
    let diffs: Vec<i32> = get_diffs(line);

    let first = *diffs.first().unwrap();
    if first == 0 {
        return false;
    }

    let sign = first.signum();

    diffs.iter().all(|d| {
        let good_sign = d.signum() == sign;
        let abs = d.abs();
        abs > 0 && abs < 4 && good_sign
    })
}

pub fn first() -> i32 {
    let lines = read_lines_of_file("2.txt");

    let lines: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            let l: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>())
                .filter(|n| n.is_ok())
                .map(|n| n.unwrap())
                .collect();
            l
        })
        .collect();

    let res = lines.iter().filter(|l| asd(l.to_vec())).count();

    res as i32
}

pub fn second() -> i32 {
    let lines = read_lines_of_file("2.txt");

    let lines: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            let l: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>())
                .filter(|n| n.is_ok())
                .map(|n| n.unwrap())
                .collect();
            l
        })
        .collect();

    let res = lines
        .iter()
        .filter(|l| {
            let unchanged = asd(l.to_vec());

            let changed = (0..l.len()).any(|i| {
                let mut l_clone = l.to_vec();
                l_clone.remove(i);
                asd(l_clone)
            });

            unchanged || changed
        })
        .count();

    res as i32
}
