use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run())
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");

    const PACKAGE_MARKER_SIZE: usize = 4;
    const MESSAGE_MARKER_SIZE: usize = 14;

    println!("Part 1: {}", find_marker_idx(input, PACKAGE_MARKER_SIZE));
    println!("Part 2: {}", find_marker_idx(input, MESSAGE_MARKER_SIZE));
}

fn find_marker_idx(input: &str, marker_size: usize) -> usize {
    input
        .as_bytes()
        .windows(marker_size)
        .position(|window| {
            let mut seen = [false; 26];
            for &e in window {
                let c = (e - b'a') as usize;
                if seen[c] {
                    return false;
                }
                seen[c] = true;
            }
            true
        })
        .unwrap()
        + marker_size
}
