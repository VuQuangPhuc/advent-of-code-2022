use std::collections::BTreeSet;

use crate::timer;

pub fn solve() -> () {
    println!("Solving day 3:");
    timer::timeit(|| run())
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");

    println!("Part 1: {}", p1(input.clone()).iter().sum::<i32>());
    println!("Part 2: {}", p2(input).iter().sum::<i32>());
}

fn p1(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| calc_rucksack_item_priority(line))
        .collect()
}

fn calc_rucksack_item_priority(rucksack: &str) -> i32 {
    let item_len = rucksack.len();
    let first_half: BTreeSet<char> = BTreeSet::<_>::from_iter((&rucksack[0..item_len / 2]).chars());
    let second_half: BTreeSet<char> =
        BTreeSet::<_>::from_iter((&rucksack[item_len / 2..item_len]).chars());

    let intersection: BTreeSet<char> = &first_half & &second_half;
    map_item_to_priority(intersection.first().unwrap())
}

fn p2(input: &str) -> Vec<i32> {
    let mut priorities: Vec<i32> = Vec::new();

    let contents: Vec<BTreeSet<char>> = input
        .lines()
        .map(|rucksack| BTreeSet::<_>::from_iter((*rucksack).chars()))
        .collect();

    for [a, b, c] in contents.into_iter().array_chunks() {
        let intersection: BTreeSet<char> = &(&a & &b) & &c;
        let item: &char = intersection.first().unwrap();
        priorities.push(map_item_to_priority(item));
    }

    priorities
}

fn map_item_to_priority(item: &char) -> i32 {
    let alphabet: Vec<char> = "♥abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    alphabet.iter().position(|i| i == item).unwrap() as i32
}
