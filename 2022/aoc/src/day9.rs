use std::collections::HashSet;
use crate::util::read_lines;

#[derive(Clone, Copy, Debug)]
struct Instruction {
    down: i32,
    right: i32,
    steps: u32,
}

struct Rope {
    knots: Vec<(i32,i32)>,
    seen: HashSet<(i32,i32)>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut knots = Vec::new();
        knots.resize(size, (0,0));
        let mut seen = HashSet::new();
        seen.insert((0,0));
        Self { knots, seen }
    }

    fn move_head(&mut self, inst: Instruction) {
        for _ in 0..inst.steps {
            self.knots[0].0 += inst.down;
            self.knots[0].1 += inst.right;
            for i in 1..self.knots.len() {
                let vec = (
                    self.knots[i-1].0 - self.knots[i].0,
                    self.knots[i-1].1 - self.knots[i].1
                );
                if vec.0.abs() > 1 || vec.1.abs() > 1 {
                    self.knots[i].0 += vec.0.clamp(-1, 1);
                    self.knots[i].1 += vec.1.clamp(-1, 1);
                }
            }
            self.seen.insert(*self.knots.last().unwrap());
        }
    }
}

fn read_input() -> Vec<Instruction> {
    read_lines("inputs/day9/input.txt").iter()
        .map(|line| {
            let mut parts = line.split(" ");
            let dir = parts.next().unwrap();
            let steps = parts.next().unwrap().parse().unwrap();
            match dir {
                "R" => Instruction { down: 0, right: 1, steps },
                "L" => Instruction { down: 0, right: -1, steps },
                "D" => Instruction { down: 1, right: 0, steps },
                "U" => Instruction { down: -1, right: 0, steps },
                _ => { unreachable!(); }
            }
        }).collect()
}

pub fn part1() {
    let mut rope = Rope::new(2);
    let instructions = read_input();
    for instruction in instructions.iter() {
        rope.move_head(*instruction);
    }
    println!("{}", rope.seen.len());
}

pub fn part2() {
    let mut rope = Rope::new(10);
    let instructions = read_input();
    for instruction in instructions.iter() {
        rope.move_head(*instruction);
    }
    println!("{}", rope.seen.len());
}