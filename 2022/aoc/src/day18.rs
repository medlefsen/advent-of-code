use std::cmp::{max, min};
use std::collections::HashSet;
use pest::iterators::Pair;
use crate::parsing::{FromPair, ParseFile, ParseNext};

#[derive(Parser)]
#[grammar="src/day18.pest"]
struct InputParser;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}
impl Coord {
    fn sides(&self) -> Vec<Self> {
        vec![
            self.move_by(-1, 0, 0),
            self.move_by(1, 0, 0),
            self.move_by(0, -1, 0),
            self.move_by(0, 1, 0),
            self.move_by(0, 0, -1),
            self.move_by(0, 0, 1),
        ]
    }

    fn move_by(&self, x: i32, y: i32, z: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    fn within(&self, a: &Self, b: &Self) -> bool {
        self.x >= a.x && self.y >= a.y && self.z >= a.z &&
            self.x <= b.x && self.y <= b.y && self.z <= b.z
    }
}

impl FromPair<Rule> for Coord {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Self {
            x: pairs.parse_next(),
            y: pairs.parse_next(),
            z: pairs.parse_next(),
        }

    }
}
fn parse_input() -> HashSet<Coord> {
    InputParser::parse_file(Rule::input, "inputs/day18/input.txt")
}

fn detect_air(lava: &HashSet<Coord>) -> HashSet<Coord> {
    let min = lava.iter().cloned().reduce(|a,b| Coord { x: min(a.x, b.x), y: min(a.y,b.y), z: min(a.z, b.z)}).unwrap().move_by(-1,-1,-1);
    let max = lava.iter().cloned().reduce(|a,b| Coord { x: max(a.x, b.x), y: max(a.y,b.y), z: max(a.z, b.z)}).unwrap().move_by(1,1,1);

    let mut air : HashSet<Coord> = HashSet::new();
    let mut queue : Vec<_> = vec![min.clone()];
    air.insert(min.clone());

    while let Some(coord) = queue.pop() {
        let new_air : Vec<_>= coord.sides().into_iter()
            .filter(|c| c.within(&min, &max))
            .filter(|c| !lava.contains(c))
            .filter(|c| !air.contains(c))
            .collect();
        for air_coord in new_air {
            air.insert(air_coord.clone());
            queue.push(air_coord);
        }
    }
    air
}

pub fn part1() {
    let coords = parse_input();
    let total : usize = coords.iter().map(|c| {
      c.sides().iter().filter(|c| !coords.contains(c) ).count()
    }).sum();

    println!("{}", total);
}

pub fn part2() {
    let input = parse_input();
    let air = detect_air(&input);

    let total : usize = input.iter().map(|c| {
        c.sides().iter().filter(|c| air.contains(c) ).count()
    }).sum();

    println!("{}", total);
}