use std::collections::{HashMap, HashSet};

use crate::read_lines_of_file;

fn get_cluster(c: char, x: usize, y: usize, lines: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut found: HashSet<(usize, usize)> = HashSet::new();
    do_cluster(c, x, y, lines, &mut found);
    found
}

fn do_cluster(
    needle: char,
    x: usize,
    y: usize,
    lines: &Vec<String>,
    found: &mut HashSet<(usize, usize)>,
) {
    if found.contains(&(x, y)) {
        return;
    } else {
        let line = lines.get(y);
        if line.is_none() {
            return;
        }

        let c = line.unwrap().chars().nth(x);
        if c.is_none() {
            return;
        }

        if c.unwrap() == needle {
            found.insert((x, y));

            if x > 0 {
                do_cluster(needle, x - 1, y, &lines, found);
            }

            if y > 0 {
                do_cluster(needle, x, y - 1, &lines, found);
            }

            let height = lines.len();
            let width = lines.first().unwrap().len();

            if y < height {
                do_cluster(needle, x, y + 1, &lines, found);
            }

            if x < width {
                do_cluster(needle, x + 1, y, &lines, found);
            }
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Hit {
    OB,
    HIT,
    MISS,
}

fn find_hits(needle: char, x: i64, y: i64, lines: &Vec<String>) -> Hit {
    if x < 0 || y < 0 {
        Hit::OB
    } else {
        let line = lines.get(y as usize);
        if line.is_none() {
            Hit::OB
        } else {
            let c = line.unwrap().chars().nth(x as usize);
            if c.is_none() {
                Hit::OB
            } else {
                if c.unwrap() != needle {
                    Hit::MISS
                } else {
                    Hit::HIT
                }
            }
        }
    }
}

fn get_price(lines: &Vec<String>, corner_counts: &Vec<Vec<usize>>, as_corners: bool) -> i64{
    let mut handled: HashSet<(usize, usize)> = HashSet::new();
    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let cluster = get_cluster(c, x, y, &lines);
                    if handled.intersection(&cluster).count() > 0 {
                        0
                    } else {
                        cluster.iter().for_each(|v| {
                            handled.insert(*v);
                        });
                        let size = cluster.len();
                        let divider = if as_corners {
                            2
                        } else {
                            1
                        };
                        let fences: usize = cluster
                            .iter()
                            .map(|(x, y)| corner_counts.get(*y).unwrap().get(*x).unwrap())
                            .map(|a| a)
                            .sum::<usize>()
                            / divider;
                        let a = size as i64 * fences as i64;
                        a
                    }
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

pub fn first() -> i64 {
    let lines = read_lines_of_file("12.txt");

    let fence_counts: Vec<Vec<usize>> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let fence_counts: Vec<usize> = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let adjacents: Vec<(i64, i64)> = vec![
                        (x as i64 - 1, y as i64),
                        (x as i64 + 1, y as i64),
                        (x as i64, y as i64 - 1),
                        (x as i64, y as i64 + 1),
                    ];
                    let fence_count: usize = adjacents
                        .iter()
                        .map(|(x, y)| find_hits(c, *x, *y, &lines))
                        .filter(|h| *h != Hit::HIT)
                        .count();
                    fence_count
                })
                .collect();
            fence_counts
        })
        .collect();

    get_price(&lines, &fence_counts, false)
}

pub fn second() -> i64 {
    let lines = read_lines_of_file("12.txt");

    let corner_counts: Vec<Vec<usize>> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let corner_counts: Vec<usize> = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let left = find_hits(c, x as i64 - 1, y as i64, &lines);
                    let right = find_hits(c, x as i64 + 1, y as i64, &lines);
                    let up = find_hits(c, x as i64, y as i64 - 1, &lines);
                    let down = find_hits(c, x as i64, y as i64 + 1, &lines);
                    let right_up = find_hits(c, x as i64 + 1, y as i64 - 1, &lines);
                    let left_up = find_hits(c, x as i64 - 1, y as i64 - 1, &lines);
                    let right_down = find_hits(c, x as i64 + 1, y as i64 + 1, &lines);
                    let left_down = find_hits(c, x as i64 - 1, y as i64 + 1, &lines);

                    let mut corners = 0;

                    if left != Hit::HIT && up != Hit::HIT {
                        corners += 2;
                    }

                    if right != Hit::HIT && up != Hit::HIT {
                        corners += 2;
                    }

                    if left != Hit::HIT && down != Hit::HIT {
                        corners += 2;
                    }

                    if right != Hit::HIT && down != Hit::HIT {
                        corners += 2;
                    }

                    if left == Hit::HIT && left_up == Hit::HIT && up == Hit::MISS {
                        corners += 1;
                    }

                    if right == Hit::HIT && right_up == Hit::HIT && up == Hit::MISS {
                        corners += 1;
                    }

                    if up == Hit::HIT && left_up == Hit::HIT && left == Hit::MISS {
                        corners += 1;
                    }

                    if down == Hit::HIT && left_down == Hit::HIT && left == Hit::MISS {
                        corners += 1;
                    }

                    if left == Hit::HIT && left_down == Hit::HIT && down == Hit::MISS {
                        corners += 1;
                    }

                    if right == Hit::HIT && right_down == Hit::HIT && down == Hit::MISS {
                        corners += 1;
                    }

                    if down == Hit::HIT && right_down == Hit::HIT && right == Hit::MISS {
                        corners += 1;
                    }

                    if up == Hit::HIT && right_up == Hit::HIT && right == Hit::MISS {
                        corners += 1;
                    }

                    corners
                })
                .collect();
            corner_counts
        })
        .collect();

    get_price(&lines, &corner_counts, true)
}
