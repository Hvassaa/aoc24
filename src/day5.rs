use crate::read_lines_of_file;

fn illegal_orderings(test: Vec<i64>, orders: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    orders
        .iter()
        .filter(|(first, last)| {
            let first_idx = test.iter().position(|v| v == first);
            let last_idx = test.iter().position(|v| v == last);

            if first_idx.is_none() || last_idx.is_none() {
                return false;
            }

            let first_idx = first_idx.unwrap();
            let last_idx = last_idx.unwrap();

            last_idx < first_idx
        })
        .map(|x| x.to_owned())
        .collect()
}

fn orders(lines: &Vec<String>) -> Vec<(i64, i64)> {
    lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut split = l.split("|");
            let first: i64 = split.nth(0).unwrap().parse().unwrap();
            let last: i64 = split.last().unwrap().parse().unwrap();
            (first, last)
        })
        .collect()
}

fn tests(lines: &Vec<String>) -> Vec<Vec<i64>> {
    lines
        .iter()
        .rev()
        .take_while(|l| !l.is_empty())
        .map(|l| l.split(",").map(|s| s.parse::<i64>().unwrap()).collect())
        .collect()
}

fn middle_elem<T>(test: &Vec<T>) -> &T {
    let idx = test.len() / 2;
    test.iter().nth(idx).unwrap()
}

pub fn first() -> i64 {
    let lines = read_lines_of_file("5.txt");
    let orders = orders(&lines);
    let tests = tests(&lines);

    tests
        .iter()
        .filter(|test| illegal_orderings(test.to_vec(), orders.to_vec()).len() == 0)
        .map(|test| middle_elem(test))
        .sum()
}

pub fn second() -> i64 {
    let lines = read_lines_of_file("5.txt");
    let orders = orders(&lines);
    let mut tests = tests(&lines);

    tests
        .iter_mut()
        .filter(|test| illegal_orderings(test.to_vec(), orders.to_vec()).len() != 0)
        .map(|test| {
            let mut corrections = illegal_orderings(test.to_vec(), orders.to_vec());
            while corrections.len() != 0 {
                corrections.iter().for_each(|(first, _last)| {
                    let first_idx = test.iter().position(|v| v == first).unwrap();
                    let removed = test.remove(first_idx);
                    test.insert(0, removed);
                });
                corrections = illegal_orderings(test.to_vec(), orders.to_vec());
            }

            test
        })
        .map(|test| middle_elem(test))
        .sum()
}
