mod day1;
mod util;

use std::env;

fn run_part(day: i32, part: i32) {
    match (day,part) {
        (1,1) => day1::part1(),
        (1,2) => day1::part2(),
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
