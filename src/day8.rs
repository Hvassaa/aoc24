use crate::read_lines_of_file;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_antennas(lines: &Vec<String>) -> HashMap<char, HashSet<(i64, i64)>> {
    let mut antennas: HashMap<char, HashSet<(i64, i64)>> = HashMap::new();

    lines.iter().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;

            match c {
                '.' => (),
                _ => match antennas.get_mut(&c) {
                    Some(set) => {
                        set.insert((x, y));
                    }
                    _ => {
                        let mut set: HashSet<(i64, i64)> = HashSet::new();
                        set.insert((x, y));
                        antennas.insert(c, set);
                    }
                },
            };
        })
    });

    antennas
}

fn antinodes_of_freq_set(
    set: &HashSet<(i64, i64)>,
    width: i64,
    height: i64,
    recurse: bool,
) -> HashSet<(i64, i64)> {
    let mut antinode_set: HashSet<(i64, i64)> = HashSet::new();

    let cartesian: HashSet<((i64, i64), (i64, i64))> = set
        .iter()
        .map(|p| {
            let product: HashSet<((i64, i64), (i64, i64))> = set
                .iter()
                .map(|q| (*p, *q))
                .filter(|(p, q)| p != q)
                .map(|((x1, y1), (x2, y2))| {
                    // remove duplicate so there is no (p, q) and (q, p)
                    if x1 > x2 {
                        ((x1, y1), (x2, y2))
                    } else {
                        ((x2, y2), (x1, y1))
                    }
                })
                .collect();

            product
        })
        .flatten()
        .collect();

    cartesian.iter().for_each(|((x1, y1), (x2, y2))| {
        let x_diff = x1.abs_diff(*x2) as i64;
        let y_diff = y1.abs_diff(*y2) as i64;

        let mut x_current_diff = x_diff;
        let mut y_current_diff = y_diff;

        let mut do_while_recurse_stupid_boolean = true;

        while do_while_recurse_stupid_boolean {
            do_while_recurse_stupid_boolean = recurse;

            let (top, bottom) = if x1 == x2 {
                let top_x = x1 - x_current_diff;
                let bottom_x = x1 + x_current_diff;

                if y1 < y2 {
                    (
                        (top_x, y1 - y_current_diff),
                        (bottom_x, y2 + y_current_diff),
                    )
                } else {
                    (
                        (top_x, y2 - y_current_diff),
                        (bottom_x, y1 + y_current_diff),
                    )
                }
            } else if y1 == y2 {
                let top_y = y1 - y_current_diff;
                let bottom_y = y1 + y_current_diff;

                (
                    (x2 - x_current_diff, top_y),
                    (x1 - x_current_diff, bottom_y),
                )
            } else if y1 < y2 {
                (
                    (x1 + x_current_diff, y1 - y_current_diff),
                    (x2 - x_current_diff, y2 + y_current_diff),
                )
            } else {
                (
                    (x1 + x_current_diff, y1 + y_current_diff),
                    (x2 - x_current_diff, y2 - y_current_diff),
                )
            };

            let var_name = [top, bottom];
            let to_insert: Vec<&(i64, i64)> = var_name
                .iter()
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .filter(|(x, y)| *x < width && *y < height)
                .collect();

            if to_insert.is_empty() {
                break;
            } else {
                to_insert.iter().for_each(|c| {
                    antinode_set.insert(**c);
                });
            };

            x_current_diff += x_diff;
            y_current_diff += y_diff;
        }
    });

    antinode_set
}

pub fn first() -> i64 {
    let lines = read_lines_of_file("8.txt");

    let mut antennas = get_antennas(&lines);

    let height = lines.len() as i64;
    let width = lines.first().unwrap().len() as i64;

    antennas
        .iter()
        .map(|(_, set)| set)
        .map(|set| antinodes_of_freq_set(set, width, height, false))
        .flatten()
        .collect::<HashSet<(i64, i64)>>()
        .iter()
        .count() as i64
}

pub fn second() -> i64 {
    let lines = read_lines_of_file("8.txt");

    let mut antennas = get_antennas(&lines);

    let height = lines.len() as i64;
    let width = lines.first().unwrap().len() as i64;

    let antinodes = antennas
        .iter()
        .map(|(_, set)| set)
        .map(|set| antinodes_of_freq_set(set, width, height, true))
        .flatten()
        .collect::<HashSet<(i64, i64)>>();

    let antennas_with_antinodes: HashSet<(i64, i64)> = antennas
        .iter()
        .map(|(_, set)| set)
        .filter(|set| set.len() > 1)
        .map(|set| set.to_owned())
        .flatten()
        .collect();

    antinodes.union(&antennas_with_antinodes).count() as i64
}
