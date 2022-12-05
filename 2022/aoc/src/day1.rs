use crate::util::read_lines;

fn read_calories(filename: &str) -> Vec<i32> {
    let lines = read_lines(filename);
    lines
        .split(|line| line == "")
        .map(|c| c.iter().map(|line| line.parse::<i32>().unwrap() ).sum())
        .collect()
}

pub fn part1() {
    println!("{}",  read_calories("inputs/day1/part1.txt").iter().max().unwrap());
}

pub fn part2() {
    let mut calories = read_calories("inputs/day1/part2.txt");
    calories.sort();
    println!("{}", calories.iter().rev().take(3).sum::<i32>());
}
