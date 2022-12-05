use std::collections::HashSet;
use crate::util::read_lines;

struct Rucksack {
    chars: Vec<char>,
}

impl Rucksack {
    fn from_line(line: &str) -> Rucksack {
        Rucksack {
            chars: line.chars().collect(),
        }
    }

    fn items(&self) -> HashSet<char> {
        HashSet::from_iter(self.chars.iter().cloned())
    }

    fn compartment_items(&self) -> [HashSet<char>; 2] {
        [
            HashSet::from_iter(self.chars[0..self.compartment_size()].iter().cloned()),
            HashSet::from_iter(self.chars[self.compartment_size()..self.chars.len()].iter().cloned())
        ]
    }

    fn compartment_size(&self) -> usize {
        self.chars.len() / 2
    }
}

struct Day3 {
    score_order: Vec<char>,
    rucksacks: Vec<Rucksack>,
}

impl Day3 {
   fn new(filename: &str) -> Self {
       let mut score_order : Vec<_> = ('a'..='z').collect();
       score_order.extend('A'..='Z');
       Self {
           score_order: score_order,
           rucksacks: read_rucksacks(filename),
       }
   }

    fn part1(&self) -> usize {
        let mut score = 0;
        for rucksack in &self.rucksacks {
            score += self.score_item(common_item(rucksack.compartment_items().iter().cloned()));
        }
        score
    }

    fn part2(&self) -> usize {
        let mut score = 0;
        for group in self.rucksacks.chunks(3) {
            let common_item = *group.iter()
                .map(|r| r.items())
                .reduce(|a,b| a.intersection(&b).cloned().collect()).unwrap()
                .iter().next().unwrap();
            score += self.score_item(common_item);
        }
        score
    }

    fn score_item(&self, item: char) -> usize {
        self.score_order.iter().position(|c| *c == item ).unwrap() + 1
    }
}

fn common_item<I>(items: I) -> char
    where
    I: Iterator<Item = HashSet<char>>
{
    *items
        .reduce(|a,b| a.intersection(&b).cloned().collect()).unwrap()
        .iter().next().unwrap()
}


fn read_rucksacks(filename: &str) -> Vec<Rucksack> {
    read_lines(filename).iter().map(|line| Rucksack::from_line(line)).collect()
}


pub fn part1() {
    let day3 = Day3::new("inputs/day3/part1.txt");
    println!("{}", day3.part1());
}

pub fn part2() {
    let day3 = Day3::new("inputs/day3/part2.txt");
    println!("{}", day3.part2());
}