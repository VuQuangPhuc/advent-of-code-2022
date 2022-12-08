use std::collections::BinaryHeap;

use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run());
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");

    let mut idx: usize = 0;
    let mut cal_counts: Vec<i32> = Vec::new();

    cal_counts.push(0);

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(cal) => {
                cal_counts[idx] += cal;
            }
            Err(_) => {
                cal_counts.push(0);
                idx += 1;
            }
        }
    }

    println!("Part 1: {0}", cal_counts.iter().max().unwrap());

    let mut heap: BinaryHeap<&i32> = BinaryHeap::new();
    cal_counts.iter().for_each(|cal: &i32| heap.push(cal));
    let mut res: i32 = 0;
    for _ in 0..3 {
        res += heap.pop().unwrap();
    }

    println!("Part 2: {0}", res);
}
