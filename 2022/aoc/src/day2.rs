use crate::util::read_lines;

#[derive(Copy, Clone, Debug)]
enum Throw { Rock, Paper, Scissors }
use Throw::{Rock, Paper, Scissors};

#[derive(Copy, Clone, Debug)]
enum Outcome { Win, Tie, Loss }
use Outcome::{Win, Tie, Loss};

fn parse_opp_throw(str: &str) -> Throw {
    match str {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => unreachable!(),
    }
}

fn parse_my_throw(str: &str) -> Throw {
    match str {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => unreachable!(),
    }
}

fn parse_outcome(str: &str) -> Outcome {
    match str {
        "X" => Loss,
        "Y" => Tie,
        "Z" => Win,
        _ => unreachable!(),
    }
}

fn score(opp_throw: Throw, my_throw: Throw) -> i32 {
    let throw_score = match my_throw {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    let outcome_score = match (opp_throw, my_throw) {
        (Rock, Paper) => 6,
        (Paper, Scissors ) => 6,
        (Scissors, Rock ) => 6,
        (Rock, Rock) => 3,
        (Paper, Paper ) => 3,
        (Scissors, Scissors ) => 3,
        _ => 0,
    };
    //println!("{:?} {:?} = {} + {} = {}", opp_throw, my_throw, throw_score, outcome_score, throw_score + outcome_score);
    throw_score + outcome_score
}

fn determine_throw(opp_throw: Throw, outcome: Outcome) -> Throw {
    match (opp_throw, outcome) {
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (throw, Tie) => throw,
    }
}

fn read_part1() -> Vec<(Throw, Throw)> {
    let lines = read_lines("inputs/day2/part1.txt");
    lines.iter().map(|line| {
        let mut throws = line.split(" ");
        (parse_opp_throw(&throws.next().unwrap()), parse_my_throw(&throws.next().unwrap()))
    }).collect()
}

fn read_part2() -> Vec<(Throw, Outcome)> {
    let lines = read_lines("inputs/day2/part2.txt");
    lines.iter().map(|line| {
        let mut throws = line.split(" ");
        (parse_opp_throw(&throws.next().unwrap()), parse_outcome(&throws.next().unwrap()))
    }).collect()
}

pub fn part1() {
    let score : i32 = read_part1().iter()
        .map(|(opp, me)| score(*opp, *me))
        .sum();

    println!("{} == {}", 11666, score);
}

pub fn part2() {
    let score : i32 = read_part2().iter()
        .map(|(opp, outcome)| score(*opp, determine_throw(*opp, *outcome)))
        .sum();

    println!("{} == {}", 12767, score);
}
