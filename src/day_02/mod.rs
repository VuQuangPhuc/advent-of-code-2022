use std::io::{Error, ErrorKind};

use crate::timer;

pub fn solve() -> () {
    println!("Solving day 2:");
    timer::timeit(|| run())
}

#[derive(Debug, Clone, Copy)]
enum MatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug)]
enum Symbol<'a> {
    Shape(&'a Shape),
    MatchResult(&'a MatchResult),
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");
    let moves: Vec<Vec<&str>> = input
        .lines()
        .map(|line: &str| line.split(&" ").collect())
        .collect();

    println!(
        "Part 1: {}",
        calc_total_score::<_, i32>(moves.clone(), calc_score_p1)
    );

    println!(
        "Part 1: {}",
        calc_total_score::<_, i32>(moves, calc_score_p2)
    );
}

fn calc_total_score<F: Fn(&Vec<&str>) -> i32, T>(
    moves: Vec<Vec<&str>>,
    score_calc_function: F,
) -> i32 {
    let shapes: Vec<i32> = moves.iter().map(score_calc_function).collect();
    shapes.iter().sum::<i32>()
}

fn calc_score_p1(round: &Vec<&str>) -> i32 {
    let shapes: Vec<Shape> = round
        .iter()
        .map(|shape: &&str| match_shape(shape).unwrap())
        .collect();

    let opponent: &Shape = &shapes[0];
    let player: &Shape = &shapes[1];

    let outcome: i32 = calc_outcome(opponent, player);

    outcome + (*player) as i32
}

fn match_shape(shape: &str) -> Result<Shape, Error> {
    match shape {
        "A" => Ok(Shape::Rock),
        "B" => Ok(Shape::Paper),
        "C" => Ok(Shape::Scissor),
        "X" => Ok(Shape::Rock),
        "Y" => Ok(Shape::Paper),
        "Z" => Ok(Shape::Scissor),
        _ => {
            let msg: String = format!("Could not match shape {}", shape);
            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
}

fn calc_outcome(opponent: &Shape, player: &Shape) -> i32 {
    match opponent {
        Shape::Rock => match player {
            Shape::Rock => MatchResult::Draw as i32,
            Shape::Paper => MatchResult::Win as i32,
            Shape::Scissor => MatchResult::Loss as i32,
        },
        Shape::Paper => match player {
            Shape::Rock => MatchResult::Loss as i32,
            Shape::Paper => MatchResult::Draw as i32,
            Shape::Scissor => MatchResult::Win as i32,
        },
        Shape::Scissor => match player {
            Shape::Rock => MatchResult::Win as i32,
            Shape::Paper => MatchResult::Loss as i32,
            Shape::Scissor => MatchResult::Draw as i32,
        },
    }
}

fn calc_score_p2(round: &Vec<&str>) -> i32 {
    let symbols: Vec<Symbol> = round
        .iter()
        .map(|shape: &&str| match_symbol(shape).unwrap())
        .collect();

    let opponent: &Shape = match &symbols[0] {
        Symbol::Shape(s) => Ok(s),
        Symbol::MatchResult(_) => {
            let msg: String = format!("Could not match opponent shape {:?}", &symbols[0]);
            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
    .unwrap();
    let outcome: &MatchResult = *match &symbols[1] {
        Symbol::MatchResult(mr) => Ok(mr),
        Symbol::Shape(_) => {
            let msg: String = format!("Could not match shape {:?}", &symbols[1]);
            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
    .unwrap();

    let player: &Shape = match outcome {
        MatchResult::Draw => opponent,
        MatchResult::Win => match opponent {
            Shape::Rock => &Shape::Paper,
            Shape::Paper => &Shape::Scissor,
            Shape::Scissor => &Shape::Rock,
        },
        MatchResult::Loss => match opponent {
            Shape::Rock => &Shape::Scissor,
            Shape::Paper => &Shape::Rock,
            Shape::Scissor => &Shape::Paper,
        },
    };

    (*outcome) as i32 + (*player) as i32
}

fn match_symbol(shape: &str) -> Result<Symbol, Error> {
    match shape {
        "A" => Ok(Symbol::Shape(&Shape::Rock)),
        "B" => Ok(Symbol::Shape(&Shape::Paper)),
        "C" => Ok(Symbol::Shape(&Shape::Scissor)),
        "X" => Ok(Symbol::MatchResult(&MatchResult::Loss)),
        "Y" => Ok(Symbol::MatchResult(&MatchResult::Draw)),
        "Z" => Ok(Symbol::MatchResult(&MatchResult::Win)),
        _ => {
            let msg: String = format!("Could not match shape {}", shape);
            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
}
