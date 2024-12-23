use crate::read_lines_of_file;

pub fn run(collapser: &dyn Fn(i64, &[i64], i64) -> bool) -> i64 {
    let lines = read_lines_of_file("7.txt");

    lines
        .iter()
        .map(|line| {
            let mut split = line.split(": ");

            let sum: i64 = split.nth(0).unwrap().parse().unwrap();
            let components: Vec<i64> = split
                .last()
                .unwrap()
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            (sum, components)
        })
        .filter(|(sum, components)| {
            if let Some((head, tail)) = components.split_first() {
                collapser(*head, tail, *sum)
            } else {
                false
            }
        })
        .map(|(sum, components)| sum)
        .sum::<i64>()
}

fn collapse(head: i64, tail: &[i64], sum: i64) -> bool {
    let split = tail.split_first();

    match split {
        Some((new_head, new_tail)) => {
            collapse(head + new_head, new_tail, sum) || collapse(head * new_head, new_tail, sum)
        }
        None => head == sum,
    }
}

fn collapse2(head: i64, tail: &[i64], sum: i64) -> bool {
    let split = tail.split_first();

    match split {
        Some((new_head, new_tail)) => {
	    let s1 = String::from(head.to_string());
	    let s2 = String::from(new_head.to_string());
	    let s = s1 + &s2;
	    
            let joined: i64 = s.parse().unwrap();
	    
            collapse2(head + new_head, new_tail, sum)
                || collapse2(head * new_head, new_tail, sum)
                || collapse2(joined, new_tail, sum)
        }
        None => head == sum,
    }
}

pub fn first() -> i64 {
    run(&collapse)
}

pub fn second() -> i64 {
    run(&collapse2)
}
