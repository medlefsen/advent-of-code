use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;
use crate::a_star::{a_star, AStarNode};
use crate::grid::*;

struct Input {
    grid: Grid<i32>,
    start: (i32, i32),
    end: (i32, i32),
}

impl<'a> FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
          static ref CHARS : Vec<char> = ('a'..='z').collect();
        }
        let mut start : Option<(i32, i32)> = None;
        let mut end : Option<(i32, i32)> = None;
        let grid : Grid<i32> = s.lines().enumerate().map(|(y,line)| {
            line.chars().enumerate().map(|(x, char)| {
               match char {
                   'S' => { start = Some((y as i32, x as i32)); Ok(0) }
                   'E' => { end = Some((y as i32, x as i32)); Ok(25) }
                   'a'..='z' => { Ok(CHARS.binary_search(&char).unwrap() as i32) }
                   _ => Err(format!("invalid char: {}", char))
               }
            }).collect::<Result<Vec<i32>, String>>()
        }).collect::<Result<Vec<Vec<i32>>, String>>()?.into();
        if let (Some(start), Some(end)) = (start, end) {
            Ok(Input { start, end, grid })
        } else {
            Err(format!("Invalid start: {:?} or end: {:?}", start, end))
        }
    }
}

fn read_input() -> Input {
    read_to_string("inputs/day12/input.txt").unwrap().parse().unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos<'a>(GridCursor<'a, i32>);

fn push_neigh_if_valid<'a, 'b: 'a>(vec: &'a mut Vec<Pos<'b>>, cur: &GridCursor<'b, i32>, maybe_neigh: Option<GridCursor<'b, i32>>) {
    if let Some(neigh) = maybe_neigh {
        if *neigh <= **cur + 1 {
            vec.push(Pos(neigh));
        }
    }
}

impl<'a> AStarNode for Pos<'a> {
    fn neighbors(&self) -> Vec<Self> {
        let mut vec = Vec::new();
        push_neigh_if_valid(&mut vec, &self.0, self.0.left());
        push_neigh_if_valid(&mut vec, &self.0, self.0.right());
        push_neigh_if_valid(&mut vec, &self.0, self.0.up());
        push_neigh_if_valid(&mut vec, &self.0, self.0.down());
        vec
    }

    fn estimate_cost_to(&self, other: &Self) -> usize {
        ((self.0.row - other.0.row).abs() + (self.0.col - other.0.col).abs()) as usize
    }
}


pub fn part1() {
    let Input { grid, start, end } = read_input();
    let start_pos = Pos(grid.cursor_at(start.0, start.1).unwrap());
    let end_pos = Pos(grid.cursor_at(end.0, end.1).unwrap());
    let path = a_star(start_pos, end_pos).unwrap();
    println!("{}", path.len() - 1);
}

pub fn part2() {
    let Input { grid, start: _,  end } = read_input();
    let end_pos = Pos(grid.cursor_at(end.0, end.1).unwrap());
    let min = grid.cursors()
        .filter(|c| **c == 0)
        .map(|start| {
            a_star(Pos(start), end_pos.clone())
                .map(|path| path.len() - 1)
                .unwrap_or(usize::MAX)
        })
        .min().unwrap();
    println!("{}", min);
}