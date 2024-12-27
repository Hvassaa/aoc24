use std::{collections::HashSet, fmt::Debug};

use crate::read_lines_of_file;

fn get_blocks() -> Vec<i32> {
    let lines = read_lines_of_file("9.txt");
    let line = lines.first().unwrap();

    let mut id = 0;
    let mut blocks: Vec<i32> = Vec::new();

    line.chars()
        .enumerate()
        .map(|(i, c)| {
            let bs = c.to_digit(10).unwrap();

            if i % 2 == 0 {
                let a = (id, bs);
                id += 1;
                a
            } else {
                (-1, bs)
            }
        })
        .for_each(|(id, bs)| {
            (0..bs).for_each(|_| blocks.push(id));
        });

    blocks
}

fn hash_blocks(blocks: Vec<i32>) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| **b != -1)
        .map(|(i, b)| i as i64 * (*b as i64))
        .sum()
}

pub fn first() -> i64 {
    let mut blocks = get_blocks();

    while migrate(&mut blocks) {}

    hash_blocks(blocks)
}

fn migrate(blocks: &mut Vec<i32>) -> bool {
    let first_empty_idx_opt = blocks.iter().position(|b| *b == -1);
    let rev_last_non_empty_idx_opt = blocks.iter().rev().position(|b| *b != -1);

    if first_empty_idx_opt.is_none() || rev_last_non_empty_idx_opt.is_none() {
        return false;
    }

    let first_empty_idx = first_empty_idx_opt.unwrap();
    let last_non_empty_idx = blocks.len() - 1 - rev_last_non_empty_idx_opt.unwrap();

    if first_empty_idx > last_non_empty_idx {
        return false;
    }

    blocks.swap(first_empty_idx, last_non_empty_idx);

    true
}

fn migrate_block(blocks: &mut Vec<i32>) {
    // certified spaghetti incoming, to not be dog-shit slow
    let mut migrated_ids: HashSet<i32> = HashSet::new();
    let mut tried_empty_indices_for_id: HashSet<usize> = HashSet::new();

    let mut use_to_move_cache = false;
    let mut to_move_cache: Vec<usize> = vec![];

    loop {
        let empty_initial_idx_opt = tried_empty_indices_for_id.iter().max();
        let empty_indices: Vec<usize> = if let Some(idx) = empty_initial_idx_opt {
            blocks
                .iter()
                .enumerate()
                .skip(*idx)
                .skip_while(|(i, b)| **b != -1 || tried_empty_indices_for_id.contains(i))
                .take_while(|(_, b)| **b == -1)
                .map(|(i, _)| i)
                .collect()
        } else {
            blocks
                .iter()
                .enumerate()
                .skip_while(|(i, b)| **b != -1 || tried_empty_indices_for_id.contains(i))
                .take_while(|(_, b)| **b == -1)
                .map(|(i, _)| i)
                .collect()
        };

        let mut id = -2;

        let to_move: Vec<usize> = if use_to_move_cache {
            use_to_move_cache = false;
            let clone = to_move_cache.clone();

            let idx = clone.first().unwrap();
            let cached_id = blocks.get(*idx).unwrap();
            id = *cached_id;

            clone
        } else {
            blocks
                .iter()
                .enumerate()
                .rev()
                .skip_while(|(_, b)| **b == -1 || migrated_ids.contains(*b))
                .take_while(|(_, b)| {
                    if id == -2 {
                        id = **b;
                    }

                    **b == id
                })
                .map(|(i, _)| i)
                .collect()
        };

        let to_move_min_idx = to_move.iter().min();
        let empty_min_idx = empty_indices.iter().min();

        if empty_min_idx.is_none() {
            migrated_ids.insert(id);
            tried_empty_indices_for_id.clear();
            continue;
        }

        if to_move_min_idx.is_none() {
            break;
        }

        let to_move_min_idx = to_move_min_idx.unwrap();
        let empty_min_idx = empty_min_idx.unwrap();

        if to_move_min_idx < empty_min_idx {
            migrated_ids.insert(id);
            tried_empty_indices_for_id.clear();
            continue;
        }

        if to_move.len() <= empty_indices.len() {
            to_move
                .iter()
                .zip(&empty_indices)
                .for_each(|(a, b)| {
                    blocks.swap(*a, *b);
                });
            migrated_ids.insert(id);
            tried_empty_indices_for_id.clear();
        } else {
            empty_indices.iter().for_each(|i| {
                tried_empty_indices_for_id.insert(*i);
            });
            use_to_move_cache = true;
            to_move_cache = to_move;
        }
    }
}

pub fn second() -> i64 {
    let mut blocks = get_blocks();
    migrate_block(&mut blocks);
    hash_blocks(blocks)
}
