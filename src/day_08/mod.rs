use std::{collections::BTreeSet, usize};

use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run());
}

fn run() -> () {
    const RADIX: u32 = 10;
    let input: &str = include_str!("__input.txt");
    let mut rows: Vec<Vec<u32>> = Vec::new();
    let mut cols: Vec<Vec<u32>> = Vec::new();
    let mut visible: BTreeSet<(usize, usize)> = BTreeSet::new();

    for (row_idx, line) in input.lines().enumerate() {
        let row: Vec<u32> = line
            .chars()
            .map(|c: char| c.to_digit(RADIX).unwrap())
            .collect::<Vec<u32>>();

        rows.push(row.clone());
        for col_idx in find_double_pass(&mut row.iter().collect()) {
            visible.insert((row_idx, col_idx));
        }
    }

    for col_idx in 0..rows.len() {
        let mut col: Vec<&u32> = rows
            .iter()
            .map(|row: &Vec<u32>| row.get(col_idx).unwrap())
            .collect();

        cols.push(col.clone().iter().map(|tree| **tree).collect());

        for row_idx in find_double_pass(&mut col) {
            visible.insert((row_idx, col_idx));
        }
    }

    println!("Part 1: {}", visible.len());

    let mut scenic_scores: Vec<u32> = vec![0; rows.len() * cols.len()];

    calc_scenic_score(rows.clone(), true, &mut scenic_scores);
    calc_scenic_score(cols.clone(), false, &mut scenic_scores);

    println!("Part 2: {:?}", scenic_scores.iter().max().unwrap());
}

fn find_double_pass(row: &mut Vec<&u32>) -> BTreeSet<usize> {
    let forward: BTreeSet<usize> = find_single_pass(&mut row.clone(), true);
    let backwards: BTreeSet<usize> = find_single_pass(row, false);

    &forward | &backwards
}

fn find_single_pass(row: &mut Vec<&u32>, is_forward: bool) -> BTreeSet<usize> {
    if !is_forward {
        row.reverse();
    }
    let mut p: &u32 = row[0];
    let mut trees: BTreeSet<usize> = BTreeSet::new();
    trees.insert(if is_forward { 0 } else { row.len() - 1 });

    for (idx, tree) in row.iter().enumerate().skip(1) {
        if tree > &p {
            p = *tree;
            trees.insert(if is_forward { idx } else { row.len() - 1 - idx });
        }
    }

    trees
}

fn calc_scenic_score(rows: Vec<Vec<u32>>, is_row: bool, out: &mut Vec<u32>) -> () {
    for (row_idx, row) in rows.iter().enumerate() {
        if row_idx == 0 || row_idx == rows.len() - 1 {
            continue;
        }
        for (col_idx, tree) in row.iter().enumerate() {
            if col_idx == 0 || col_idx == row.len() - 1 {
                continue;
            }

            let mut l = 0;
            for idx in (0..col_idx).rev() {
                l += 1;
                if &row[idx] >= tree {
                    break;
                }
            }

            let mut r = 0;
            for idx in col_idx + 1..row.len() {
                r += 1;
                if &row[idx] >= tree {
                    break;
                }
            }

            let out_idx: usize = if is_row {
                row_idx * rows.len() + col_idx
            } else {
                row_idx + col_idx * row.len()
            };
            let score: u32 = (l * r) as u32;

            out[out_idx] = if out[out_idx] == 0 {
                score
            } else {
                out[out_idx] * score
            };
        }
    }
}
