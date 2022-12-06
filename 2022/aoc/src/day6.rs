use std::fs::read_to_string;

fn has_duplicates(vec: &Vec<char>) -> bool {
    let mut copy = vec.clone();
    copy.sort();
    copy.dedup();
    copy.len() != vec.len()
}

fn find_sequence_start(filename: &str, min_length: usize) -> Option<usize> {
    let input = read_to_string(filename).unwrap();
    let mut buf : Vec<char> = Vec::new();
    for (i, char) in input.chars().enumerate() {
        buf.push(char);
        if buf.len() == min_length {
            if !has_duplicates(&buf) {
                return Some(i + 1);
            }
            buf.remove(0);
        }
    }
    None
}

pub fn part1() {
    println!("{}", find_sequence_start("inputs/day6/input.txt", 4).unwrap());
}

pub fn part2() {
    println!("{}", find_sequence_start("inputs/day6/input.txt", 14).unwrap());
}