use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run());
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");
    let register_states = run_program(input);
    println!("Part 1: {}", calc_signal_strength_sum(&register_states));
    println!("Part 2:");
    render_display(&register_states);
}

fn run_program(input: &str) -> Vec<i32> {
    let mut register_states: Vec<i32> = vec![1];
    for line in input.lines() {
        let instruction = line.split_once(" ");

        match instruction {
            // noop
            None => {
                register_states.push(register_states[register_states.len() - 1]);
            }
            // addx V
            Some((_, value)) => {
                let x: i32 = register_states[register_states.len() - 1];
                register_states.push(x);
                register_states.push(x + value.parse::<i32>().unwrap());
            }
        }
    }

    register_states
}

fn calc_signal_strength_sum(register_states: &Vec<i32>) -> i32 {
    let mut signal_sum: i32 = 0;
    for idx in [20, 60, 100, 140, 180, 220] {
        signal_sum += idx as i32 * register_states[idx - 1];
    }
    signal_sum
}

fn render_display(register_states: &Vec<i32>) -> () {
    const DISPLAY_WIDTH: usize = 40;
    const DISPLEY_HEIGHT: usize = 6;

    let mut display: [char; 240] = ['•'; DISPLAY_WIDTH * DISPLEY_HEIGHT];

    for idx in 0..DISPLAY_WIDTH * DISPLEY_HEIGHT {
        let p: i32 = register_states[idx];
        let q: i32 = (idx % DISPLAY_WIDTH) as i32;
        if q >= p - 1 && q <= p + 1 {
            display[idx] = '◘';
        }
    }

    for line in display.chunks(DISPLAY_WIDTH) {
        println!("{}", line.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let input: &str = include_str!("__input_test.txt");
        let register_states = run_program(input);
        let res: i32 = calc_signal_strength_sum(&register_states);
        let expected: i32 = 13140;
        assert_eq!(res, expected, "Got {}, expected {}.", res, expected);
    }

    #[test]
    fn test_display() {
        let input: &str = include_str!("__input_test.txt");
        let register_states: Vec<i32> = run_program(input);
        render_display(&register_states);
    }
}
