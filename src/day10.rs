use std::collections::HashSet;

use crate::read_lines_of_file;

fn get_map_and_starts() -> (Vec<Vec<i32>>, Vec<(i32, i32)>) {
    let lines = read_lines_of_file("10.txt");
    let map: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c.to_digit(10) {
                    Some(d) => d as i32,
                    _ => -1 as i32,
                })
                .collect::<Vec<i32>>()
        })
        .collect();

    let starts: Vec<(i32, i32)> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, i)| **i == 0)
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect();

    (map, starts)
}

fn get_next_valid_positions(pos: (i32, i32), map: &Vec<Vec<i32>>) -> Vec<((i32, i32), i32)> {
    let (x, y) = pos;
    let want = *map.get(y as usize).unwrap().get(x as usize).unwrap() + 1;

    let width = map.len() as i32;
    let height = map.first().unwrap().len() as i32;

    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .filter(|(x, y)| *x < width && *y < height)
        .map(|(x, y)| {
            (
                (x, y),
                map.get(*y as usize).unwrap().get(*x as usize).unwrap(),
            )
        })
        .filter(|(_, v)| **v == want)
        .map(|((x, y), v)| ((*x, *y), *v))
        .collect()
}

fn count_unqie_trail_ends(pos: (i32, i32), map: &Vec<Vec<i32>>) -> i64 {
    let mut a: HashSet<(i32, i32)> = HashSet::new();
    do_unique_trail_ends(pos, map, &mut a);
    a.len() as i64
}

fn do_unique_trail_ends(pos: (i32, i32), map: &Vec<Vec<i32>>, nines: &mut HashSet<(i32, i32)>) {
    get_next_valid_positions(pos, map).iter().for_each(|((x, y), v)| {
        if *v == 9 {
            nines.insert((*x, *y));
        } else {
            do_unique_trail_ends((*x, *y), map, nines);
        }
    });
}

fn count_all_trail_paths(pos: (i32, i32), map: &Vec<Vec<i32>>) -> i64 {
    get_next_valid_positions(pos, map)
        .iter()
        .map(|((x, y), v)| if *v == 9 { 1 } else { count_all_trail_paths((*x, *y), map) })
        .sum::<i64>()
}

pub fn first() -> i64 {
    let (map, starts) = get_map_and_starts();
    starts.iter().map(|pos| count_unqie_trail_ends(*pos, &map)).sum::<i64>()
}

pub fn second() -> i64 {
    let (map, starts) = get_map_and_starts();
    starts.iter().map(|pos| count_all_trail_paths(*pos, &map)).sum::<i64>()
}
