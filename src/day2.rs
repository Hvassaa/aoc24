use crate::read_lines_of_file;

fn get_diffs(l: Vec<i64>) -> Vec<i64> {
    let mut a = l.clone();
    let mut b = l.clone();
    a.push(0);
    b.insert(0, 0);
    let mut diffs: Vec<i64> = a.iter().zip(b).map(|(a, b)| (a - b)).collect();

    diffs.remove(0);
    diffs.remove(diffs.len() - 1);
    diffs
}

fn asd(line: Vec<i64>) -> bool {
    let diffs: Vec<i64> = get_diffs(line);

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

pub fn first() -> i64 {
    let lines = read_lines_of_file("2.txt");

    let lines: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            let l: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse::<i64>())
                .filter(|n| n.is_ok())
                .map(|n| n.unwrap())
                .collect();
            l
        })
        .collect();

    let res = lines.iter().filter(|l| asd(l.to_vec())).count();

    res as i64
}

pub fn second() -> i64 {
    let lines = read_lines_of_file("2.txt");

    let lines: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            let l: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse::<i64>())
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

    res as i64
}
