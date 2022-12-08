use std::num::ParseIntError;

use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run());
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");

    let stack_count_idx: usize = input
        .lines()
        .position(|line: &str| !line.contains("["))
        .unwrap();
    let stack_count: usize = input.lines().collect::<Vec<&str>>()[stack_count_idx]
        .split_whitespace()
        .count();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let stack_lines: &[&str] = &input.lines().collect::<Vec<&str>>()[0..stack_count_idx];

    for idx in 0..stack_count {
        stacks.push(Vec::new());
        for depth in (0..stack_count_idx).rev() {
            let content: char = stack_lines[depth].chars().collect::<Vec<char>>()[1 + idx * 4];
            if content.is_whitespace() {
                continue;
            }
            stacks[idx].push(content);
        }
    }

    let instructions: &[&str] = &input.lines().collect::<Vec<&str>>()[stack_count_idx + 1..];

    crate_mover_9000(stacks.clone(), instructions.clone());
    crate_mover_9001(stacks, instructions);
}

fn crate_mover_9000(mut stacks: Vec<Vec<char>>, instructions: &[&str]) -> () {
    for instruction in instructions {
        if instruction.is_empty() {
            continue;
        }
        let parsed_instruction: Vec<usize> = instruction
            .split_whitespace()
            .map(|part: &str| part.parse::<usize>())
            .filter_map(|result: Result<usize, ParseIntError>| result.ok())
            .collect::<Vec<usize>>();

        assert_eq!(parsed_instruction.len(), 3);

        for _ in 0..parsed_instruction[0] {
            let item: char = stacks[parsed_instruction[1] - 1].pop().unwrap();
            stacks[parsed_instruction[2] - 1].push(item);
        }
    }

    println!("Part 1: {}", assemble_message(stacks));
}

fn crate_mover_9001(mut stacks: Vec<Vec<char>>, instructions: &[&str]) -> () {
    for instruction in instructions {
        if instruction.is_empty() {
            continue;
        }
        let parsed_instruction: Vec<usize> = instruction
            .split_whitespace()
            .map(|part| part.parse::<usize>())
            .filter_map(|result| result.ok())
            .collect::<Vec<usize>>();

        assert_eq!(parsed_instruction.len(), 3);

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..parsed_instruction[0] {
            let item = stacks[parsed_instruction[1] - 1].pop().unwrap();
            tmp.push(item);
        }

        tmp.reverse();

        for item in tmp {
            stacks[parsed_instruction[2] - 1].push(item);
        }
    }

    println!("Part 2: {}", assemble_message(stacks));
}

fn assemble_message(stacks: Vec<Vec<char>>) -> String {
    let mut message: Vec<char> = Vec::new();
    for mut stack in stacks {
        match stack.pop() {
            Some(content) => message.push(content),
            None => (),
        }
    }

    message.iter().collect::<String>()
}
