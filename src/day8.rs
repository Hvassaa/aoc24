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

fn antinodes_of_freq_set(set: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut antinode_set: HashSet<(i64, i64)> = HashSet::new();

    let cartesian: HashSet<((i64, i64), (i64, i64))> = set
        .iter()
        .map(|p| {
            let product: HashSet<((i64, i64), (i64, i64))> = set
                .iter()
                .map(|q| (*p, *q))
                .filter(|(p, q)| p != q)
                .collect();

            product
        })
        .flatten()
        .collect();

    cartesian.iter().for_each(|((x1, y1), (x2, y2))| {
        let x_diff = x1.abs_diff(*x2) as i64;
        let y_diff = y1.abs_diff(*y2) as i64;

        if x1 == x2 {
            let top_x = x1 - x_diff;
            let bottom_x = x1 + x_diff;

            if y1 < y2 {
                let top = (top_x, y1 - y_diff);
                let bottom = (bottom_x, y2 + y_diff);
                antinode_set.insert(top);
                antinode_set.insert(bottom);
            } else {
                let top = (top_x, y2 - y_diff);
                let bottom = (bottom_x, y1 + y_diff);
                antinode_set.insert(top);
                antinode_set.insert(bottom);
            }
        }

        if y1 == y2 {
            let top_y = y1 - y_diff;
            let bottom_y = y1 + y_diff;

            if x1 < x2 {
                let top = (x1 - x_diff, top_y);
                let bottom = (x2 + x_diff, bottom_y);
                antinode_set.insert(top);
                antinode_set.insert(bottom);
            } else {
                let top = (x2 - x_diff, top_y);
                let bottom = (x1 - x_diff, bottom_y);
                antinode_set.insert(top);
                antinode_set.insert(bottom);
            }
        }

        if x1 < x2 {
            if y1 < y2 {
                antinode_set.insert((x1 - x_diff, y1 - y_diff));
                antinode_set.insert((x2 + x_diff, y2 + y_diff));
            } else {
                antinode_set.insert((x1 - x_diff, y1 + y_diff));
                antinode_set.insert((x2 + x_diff, y2 - y_diff));
            }
        } else {
            if y1 < y2 {
                antinode_set.insert((x1 + x_diff, y1 - y_diff));
                antinode_set.insert((x2 - x_diff, y2 + y_diff));
            } else {
                antinode_set.insert((x1 + x_diff, y1 + y_diff));
                antinode_set.insert((x2 - x_diff, y2 - y_diff));
            }
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
        .map(|set| antinodes_of_freq_set(set))
        .flatten()
        .collect::<HashSet<(i64, i64)>>()
        .iter()
        .filter(|(x, _y)| *x >= 0 && *x < width)
        .filter(|(_x, y)| *y >= 0 && *y < height)
        .count() as i64

    // (0..height).for_each(|y| {
    //     (0..width).for_each(|x| {
    //         let c_opt = antennas.iter().filter(|map| {
    //             map.1.contains(&(x, y))
    //         }).map(|map| map.0).last();

    //         if antinodes.contains(&(x,y)) {
    //             print!("#");

    //         } else if let Some(c) = c_opt {
    //             print!("{}", c);
    //         } else {
    //             print!(".");
    //         }
    //     });
    //     println!();
    // });
}
