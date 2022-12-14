use std::fmt::{Display, Formatter, Write};
use crate::grid::{Grid, HashMapGrid};
use crate::parsing::ParseFile;

#[derive(Parser)]
#[grammar = "src/day14.pest"]
struct InputParser;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => { f.write_char('.') }
            Tile::Rock => { f.write_char('#') }
            Tile::Sand => { f.write_char('o')}
        }
    }
}

type Cave = HashMapGrid<Tile>;

type Point = (i32, i32);
type Line = Vec<Point>;
struct LineSegIterator {
    cur: Option<Point>,
    end: Point
}

impl LineSegIterator {
    fn new(start: Point, end: Point) -> Self {
        Self{ cur: Some(start), end }
    }
}
impl Iterator for LineSegIterator {
    type Item = Point ;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|cur| {
            let diff = (
                (self.end.0 - cur.0).clamp(-1, 1),
                (self.end.1 - cur.1).clamp(-1, 1),
            );
            match diff {
                (0,0) => { self.cur = None; }
                (0, dy) => { self.cur = Some((cur.0, cur.1 + dy)); }
                (dx, _) => { self.cur = Some((cur.0 + dx, cur.1)); }
            }
            cur
        })
    }
}

fn parse_input() -> Vec<Line> {
    let (lines,) = InputParser::parse_file(Rule::input, "inputs/day14/input.txt");
    lines
}

fn build_cave(lines: &Vec<Line>) -> Cave {
    let mut cave = Cave::new(Tile::Air);
    for line in lines {
        let mut line_points = line.iter();
        let mut start_point = line_points.next().unwrap();
        for end_point in line_points {
            for (col, row) in LineSegIterator::new(*start_point, *end_point) {
               cave.set(row, col, Tile::Rock);
            }
            start_point = end_point;
        }
    }
    cave
}

fn tile_at(cave: &Cave, row: i32, col: i32) -> Tile {
    cave.cursor_at(row, col).map(|c| *c).unwrap_or(Tile::Air)
}

fn move_sand(cave: &Cave, row: i32, col: i32) -> Option<(i32, i32)> {
    [(row + 1, col), (row + 1, col -1), (row + 1, col + 1)].iter().copied()
        .find(|(row, col)| tile_at(cave, *row, *col) == Tile::Air )
}

struct InfiniteSandDrip {
    total : usize,
    falling_off: bool,
    cur_sand : Option<(i32,i32)>,
}

impl InfiniteSandDrip {
    fn new() -> Self {
        Self{ total: 0, falling_off: false, cur_sand: None }
    }

    fn step(&mut self, cave: &mut Cave) {
        if let Some((row, col)) = self.cur_sand.clone() {
            if row == cave.max_row() {
                self.falling_off = true;
                self.cur_sand = None;
            }
            self.cur_sand = move_sand(cave, row, col);
            if let Some((new_row, new_col)) = self.cur_sand {
                cave.set(row, col, Tile::Air);
                cave.set(new_row, new_col, Tile::Sand);
            }
        } else {
            cave.set(0, 500, Tile::Sand);
            self.total += 1;
            self.cur_sand = Some((0, 500));
        }
    }
}

struct FloorSandDrip {
    total : usize,
    floor : i32,
    blocked: bool,
    cur_sand : Option<(i32,i32)>,
}

impl FloorSandDrip {
    fn new(floor: i32) -> Self {
        Self{ total: 0, floor, blocked: false, cur_sand: None }
    }

    fn step(&mut self, cave: &mut Cave) {
        if let Some((row, col)) = self.cur_sand.clone() {
            self.cur_sand = move_sand(cave, row, col);
            if let Some((new_row, new_col)) = self.cur_sand {
                cave.set(row, col, Tile::Air);
                cave.set(new_row, new_col, Tile::Sand);
                if new_row == self.floor {
                    self.cur_sand = None;
                }
            }
        } else {
            if tile_at(cave, 0, 500) == Tile::Air {
                cave.set(0, 500, Tile::Sand);
                self.total += 1;
                self.cur_sand = Some((0, 500));
            } else {
                self.blocked = true;
            }
        }
    }
}

pub fn part1() {
    let lines = parse_input();
    let mut sand_drip = InfiniteSandDrip::new();
    let mut cave = build_cave(&lines);
    while ! sand_drip.falling_off {
        sand_drip.step(&mut cave);
    }
    println!("{}", sand_drip.total - 1);
}

pub fn part2() {
    let lines = parse_input();
    let mut cave = build_cave(&lines);
    let mut sand_drip = FloorSandDrip::new(cave.max_row() + 1);
    while ! sand_drip.blocked {
        sand_drip.step(&mut cave);
    }
    println!("{}", sand_drip.total);
}