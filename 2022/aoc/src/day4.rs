use std::ops::Range;
use crate::util::read_lines;

type Section = Range<i32>;
type Pair = (Section, Section);

fn read_pairs(filename: &str) -> Vec<Pair> {
    read_lines(filename).iter().map(|line| {
        let sections : Vec<Section>= line.split(",").map(|section| {
            let parts = section.split("-").take(2).map(|str| str.parse().unwrap()).collect::<Vec<i32>>();
            Range { start: parts[0], end: parts[1] }
        }).collect();
        (sections[0].clone(), sections[1].clone())
    }).collect()
}

fn fully_contains(a: &Section, b: &Section) -> bool {
    a.start <= b.start && a.end >= b.end || b.start <= a.start && b.end >= a.end
}

fn overlaps(a: &Section, b: &Section) -> bool {
    (b.start <= a.end && !(b.end < a.start)) ||
        (a.start <= b.end && !(a.end < b.start))
}

pub fn part1() {
    let score = read_pairs("inputs/day4/part1.txt").iter().filter(|(a,b)| fully_contains(a,b)).count();
    println!("{}", score);
}

pub fn part2() {
    let score = read_pairs("inputs/day4/part2.txt").iter().filter(|(a,b)| overlaps(a,b)).count();
    println!("{}", score);
}