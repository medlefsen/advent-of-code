#[macro_use]
extern crate pest_derive;

mod day1;
mod util;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::env;

fn run_part(day: i32, part: i32) {
    match (day,part) {
        (1,1) => day1::part1(),
        (1,2) => day1::part2(),
        (2,1) => day2::part1(),
        (2,2) => day2::part2(),
        (3,1) => day3::part1(),
        (3,2) => day3::part2(),
        (4,1) => day4::part1(),
        (4,2) => day4::part2(),
        (5,1) => day5::part1(),
        (5,2) => day5::part2(),
        (6,1) => day6::part1(),
        (6,2) => day6::part2(),
        (7,1) => day7::part1(),
        (7,2) => day7::part2(),
        _ => println!("Invalid args"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            match (args[1].parse(), args[2].parse()) {
                (Ok(day),Ok(1)) => run_part(day, 1),
                (Ok(day),Ok(2)) => run_part(day, 2),
                _ => println!("Invalid args"),
            }

        }
        _ => {
          println!("usage: aoc day 1/2")
        }
    }
}
