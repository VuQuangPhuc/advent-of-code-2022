use std::collections::HashSet;

use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run());
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");

    const ROPE_LEN_1: usize = 2;
    const ROPE_LEN_2: usize = 10;

    println!("Part 1: {}", simulate::<ROPE_LEN_1>(input));
    println!("Part 2: {}", simulate::<ROPE_LEN_2>(input));
}

fn simulate<const ROPE_LEN: usize>(input: &str) -> usize {
    let mut rope = [[0, 0]; ROPE_LEN];
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    history.insert((0, 0));

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();

        for _ in 0..steps.parse::<u32>().unwrap() {
            match direction {
                "U" => rope[0][1] += 1,
                "D" => rope[0][1] -= 1,
                "R" => rope[0][0] += 1,
                "L" => rope[0][0] -= 1,
                _ => (),
            }

            for idx in 1..ROPE_LEN {
                let x_diff: i32 = rope[idx - 1][0] - rope[idx][0];
                let y_diff: i32 = rope[idx - 1][1] - rope[idx][1];

                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    rope[idx][0] += x_diff.signum();
                    rope[idx][1] += y_diff.signum();
                }
            }

            history.insert((rope[ROPE_LEN - 1][0], rope[ROPE_LEN - 1][1]));
        }
    }

    history.len()
}
