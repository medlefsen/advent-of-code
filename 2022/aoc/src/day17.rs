use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::fs::read_to_string;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Cell {
    #[default]
    Air,
    Rock,
    Wall,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Cell::Air => { '.' }
            Cell::Rock => { '#' }
            Cell::Wall => { '*' }
        };
        f.write_char(ch)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos { x: i32, y: i32 }

#[derive(Debug)]
struct Shape {
    pieces: Vec<Pos>,
}
impl Shape {
    fn new(pieces: Vec<Pos>) -> Self {
        Self { pieces }
    }
    fn from_strings(strs: Vec<&str>) -> Vec<Self> {
        strs.iter().map(|s| s.parse().unwrap()).collect()
    }
    fn height(&self) -> i32 { self.pieces.iter().map(|p| p.y).max().unwrap() + 1 }
}
impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Shape::new(s.lines().rev().enumerate().flat_map(|(y, line)| {
                line.chars().enumerate().filter(|(_,ch)| *ch == '#')
                    .map(|(x, _)| Pos { x: x as i32, y: y as i32}).collect::<Vec<_>>()
            }).collect())
        )
    }
}


struct Rock {
    pos: Pos,
    shape: usize,
    shapes: &'static Vec<Shape>,
}

impl Rock {
    fn top(&self) -> i32 { self.pos.y + self.shapes[self.shape].height() - 1 }
    fn poses(&self) -> Vec<Pos> {
        self.shapes[self.shape].pieces.iter().map(|pos| {
            Pos{ x: self.pos.x + pos.x, y: self.pos.y + pos.y }
        }).collect()
    }
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn from_char(ch: char) -> Self {
        match ch {
            '>' => Move::Right,
            '<' => Move::Left,
            _ => panic!("Invalid char")
        }
    }

    fn apply(&self, pos: Pos) -> Pos {
        let dx = match self { Self::Left => -1, Self::Right => 1};
        Pos { x: pos.x + dx, y: pos.y }
    }
}

lazy_static! {
    static ref SHAPES : Vec<Shape> = Shape::from_strings(vec![
        "####",
        ".#.\n\
         ###\n\
         .#.",
        "..#\n\
         ..#\n\
         ###",
        "#\n\
         #\n\
         #\n\
         #",
        "##\n\
         ##",
    ]);
}

#[derive(Clone)]
struct Simulation {
    left_wall: i32,
    right_wall: i32,
    floor: i32,
    shapes: &'static Vec<Shape>,
    moves: Vec<Move>,
}

impl Simulation {
    fn new(moves: Vec<Move>) -> Self {
        Self {
            left_wall: 0,
            right_wall: 8,
            floor: 0,
            shapes: &SHAPES,
            moves,
        }
    }

    fn new_rock(&self, shape: usize, highest_point: i32) -> Rock {
        Rock {
            shapes: self.shapes,
            pos: Pos { x: self.left_wall + 3, y: highest_point + 4},
            shape,
        }
    }

    fn new_run(&self) -> Run {
        Run::new(self)
    }
}

struct Run<'a> {
    simulation: &'a Simulation,
    highest_point: i32,
    grid: HashMap<Pos, Cell>,
    num_rocks: usize,
    cur_rock: Rock,
    cur_move: usize,
    move_down: bool,
}

impl<'a> Run<'a> {
    fn new(simulation: &'a Simulation) -> Self {
        let mut run = Self {
            simulation,
            highest_point: simulation.floor,
            grid: Default::default(),
            num_rocks: 0,
            cur_rock: simulation.new_rock(0, simulation.floor),
            cur_move: 0,
            move_down: false,
        };
        run.set_rock_cells();
        run
    }

    fn next_rock(&mut self) {
        self.num_rocks += 1;
        let shape = (self.cur_rock.shape + 1) % self.cur_rock.shapes.len();
        self.cur_rock = self.simulation.new_rock(shape, self.highest_point);
        self.set_rock_cells();
    }

    fn update_high_point(&mut self) {
        self.highest_point = max(self.highest_point, self.cur_rock.top());
    }

    fn step(&mut self) {
        if self.move_down {
            self.move_down();
        } else {
            self.move_horiz();
        }
        self.move_down = !self.move_down;
    }

    fn run_until_next_rock(&mut self) {
        let count = self.num_rocks;
        while self.num_rocks == count {
            self.step();
        }
    }

    fn move_horiz(&mut self) {
        let m = self.simulation.moves[self.cur_move];
        self.try_move(|p| m.apply(p));
        self.cur_move = (self.cur_move + 1) % self.simulation.moves.len();
    }

    fn move_down(&mut self) {
        if !self.try_move(|pos| Pos { x: pos.x, y: pos.y - 1 }) {
            self.update_high_point();
            self.next_rock();
        }
    }

    fn try_move<F: FnOnce(Pos) -> Pos>(&mut self, f: F) -> bool {
        self.unset_rock_cells();
        let old_pos = self.cur_rock.pos.clone();
        self.cur_rock.pos = f(old_pos);
        if self.is_possible_move() {
            self.set_rock_cells();
             true
        } else {
            self.cur_rock.pos = old_pos;
            self.set_rock_cells();
            false
        }
    }

    fn unset_rock_cells(&mut self) {
        for pos in &self.cur_rock.poses() {
            self.grid.insert(*pos, Cell::Air);
        }
    }

    fn set_rock_cells(&mut self) {
        for pos in &self.cur_rock.poses() {
            self.grid.insert(*pos, Cell::Rock);
        }
    }

    fn is_possible_move(&self) -> bool {
        self.cur_rock.poses().iter().all(|p| {
            self.cell(*p) == Cell::Air
        })
    }

    fn cell(&self, pos: Pos) -> Cell {
        if pos.x > self.simulation.left_wall &&
            pos.x < self.simulation.right_wall &&
            pos.y > self.simulation.floor {
            self.grid.get(&pos).copied().unwrap_or(Cell::Air)
        } else {
            Cell::Wall
        }
    }
}

impl<'a> Display for Run<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (self.simulation.floor..=self.cur_rock.top()).rev() {
            for x in self.simulation.left_wall..=self.simulation.right_wall {
                self.cell(Pos { x, y}).fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn detect_loop(diffs: &Vec<usize>, start: usize, size: usize) -> bool {
    let mut values : Vec<usize> = Vec::new();
    let mut index = start;
    while (index + size) < diffs.len() {
        values.push(diffs[index..(index+size)].iter().sum());
        index += size;
    }
    values.iter().all(|v| v == values.first().unwrap()) && {
        println!("Found loop: (start: {}, size: {}) {} times", start, size, values.len());
        true
    }
}

fn parse_input() -> Vec<Move> {
    read_to_string("inputs/day17/input.txt")
        .unwrap().lines().next().unwrap().chars().map(|c| Move::from_char(c)).collect()
}

pub fn part1() {
    let moves = parse_input();
    let simulation = Simulation::new(moves);
    let mut run = simulation.new_run();
    while run.num_rocks < 2022 {
        run.step();
    }
    println!("{}", run.highest_point);
}

pub fn part2() {
    let moves = parse_input();
    let simulation = Simulation::new(moves);
    let mut run = simulation.new_run();
    let num_shapes = simulation.shapes.len();
    let high_points : Vec<_> = (0..(10000 * num_shapes)).map(|_| {
        run.run_until_next_rock();
        run.highest_point
    }).collect();
    let mut high_point = 0;
    let diffs : Vec<_>= high_points.iter().map(|hp| {
        let uhp = *hp as usize;
        let diff = uhp - high_point;
        high_point = uhp;
        diff
    }).collect();
    let (start, size) = (0..500).cartesian_product(1..400)
        .map(|(start,size)| (start * num_shapes, size * num_shapes) )
        .find(|(start, size)| detect_loop(&diffs, *start , *size)).unwrap();
    let mut rocks_left = 1000000000000;
    let start_height : usize = diffs[0..start].iter().sum();
    rocks_left -= start;

    let loop_height : usize = diffs[start..(start + size)].iter().sum();
    let loops = rocks_left / size;
    rocks_left -= loops * size;
    let rest_height: usize = diffs[start..(start + rocks_left)].iter().sum();
    let height = start_height + (loop_height * loops) + rest_height;
    println!("{} = {} + ({} * {}) + {}", height, start_height, loop_height, loops, rest_height);
}