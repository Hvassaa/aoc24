use crate::read_lines_of_file;
use std::collections::HashSet;

fn get_map(lines: &Vec<String>) -> ((i64, i64), HashSet<(i64, i64)>) {
    let mut position: (i64, i64) = (0, 0);
    let blocks: HashSet<(i64, i64)> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(x, char)| {
                    if *char == '^' {
                        position = (*x as i64, y as i64)
                    }
                    *char == '#'
                })
                .map(move |(x, _char)| (x as i64, y as i64))
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect();

    (position, blocks)
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Dir {
    fn next_pos(&self, position: (i64, i64)) -> (i64, i64) {
        let (x, y) = position;

        match *self {
            Dir::UP => (x, y - 1),
            Dir::RIGHT => (x + 1, y),
            Dir::DOWN => (x, y + 1),
            Dir::LEFT => (x - 1, y),
        }
    }

    fn turn(&self) -> &Dir {
        match *self {
            Dir::UP => &Dir::RIGHT,
            Dir::RIGHT => &Dir::DOWN,
            Dir::DOWN => &Dir::LEFT,
            Dir::LEFT => &Dir::UP,
        }
    }
}

fn get_visited_with_directions(
    blocks: &HashSet<(i64, i64)>,
    width: i64,
    height: i64,
    start: (i64, i64),
) -> Option<HashSet<((i64, i64), Dir)>> {
    let mut position = start;
    let mut direction = &Dir::UP;

    let mut visitied: HashSet<((i64, i64), Dir)> = HashSet::new();
    visitied.insert((position, direction.clone()));

    loop {
        let new_pos = direction.next_pos(position);
        let (new_x, new_y) = new_pos;

        if new_x < 0 || new_y < 0 || new_x == width || new_y == height {
            break;
        }

        if blocks.contains(&new_pos) {
            direction = direction.turn();
        } else {
            if !visitied.insert((new_pos, direction.clone())) {
                return None;
            }
            position = new_pos;
        }
    }

    return Some(visitied);
}

pub fn first() -> i64 {
    let lines = read_lines_of_file("6.txt");

    let (position, blocks) = get_map(&lines);
    let height = lines.len() as i64;
    let width = lines.first().unwrap().len() as i64;

    let visited = get_visited_with_directions(&blocks, width, height, position);
    if let Some(a) = visited {
        let positions: HashSet<(i64, i64)> = a.iter().map(|(pos, _dir)| *pos).collect();
        return positions.len() as i64;
    }

    return 0;
}

pub fn second() -> i64 {
    let lines = read_lines_of_file("6.txt");

    let (position, mut blocks) = get_map(&lines);
    let height = lines.len() as i64;
    let width = lines.first().unwrap().len() as i64;

    let original_visited = get_visited_with_directions(&blocks, width, height, position).unwrap();
    let original_positions: HashSet<(i64, i64)> = original_visited
        .iter()
        .map(|(pos, _dir)| *pos)
        .filter(|pos| *pos != position)
        .collect();

    let mut res = 0;

    original_positions.iter().for_each(|new_block| {
        if blocks.insert(*new_block) {
            let visited = get_visited_with_directions(&blocks, width, height, position);

            if visited.is_none() {
                res += 1;
            }

            blocks.remove(&new_block);
        }
    });

    return res;
}
