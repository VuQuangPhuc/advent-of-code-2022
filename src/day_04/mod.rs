use crate::timer;

pub fn solve() -> () {
    println!("Solving day 4:");
    timer::timeit(|| run());
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");
    let assignments: Vec<Vec<Vec<i32>>> = parse_input(input);

    let mut count_fully_contain: i32 = 0;
    let mut count_overlap: i32 = 0;

    for assignment in assignments {
        let l1: i32 = assignment[0][0];
        let l2: i32 = assignment[1][0];
        let r1: i32 = assignment[0][1];
        let r2: i32 = assignment[1][1];

        if (l1 >= l2 && r1 <= r2) || (l2 >= l1 && r2 <= r1) {
            count_fully_contain += 1;
        }

        if l1 <= r2 && l2 <= r1 {
            count_overlap += 1;
        }
    }

    println!("Part 1: {}", count_fully_contain);
    println!("Part 2: {}", count_overlap);
}

fn parse_input(input: &str) -> Vec<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|section| {
                    section
                        .split("-")
                        .map(|limit| limit.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
}
