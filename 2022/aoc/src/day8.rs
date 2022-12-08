use crate::grid::{Grid, GridCursor};
use crate::util::read_lines;

fn read_input() -> Grid<u32> {
    read_lines("inputs/day8/input.txt").iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
        .into()
}

fn is_visible_in_dir(cursor: GridCursor<u32>, down: i32, right: i32) -> bool {
    cursor.iter_by(down, right).all(|c| {
        *c < *cursor
    })
}

fn is_visible(cursor: GridCursor<u32>) -> bool {
    is_visible_in_dir(cursor.clone(), 1, 0)
        || is_visible_in_dir(cursor.clone(), -1, 0)
        || is_visible_in_dir(cursor.clone(), 0, 1)
        || is_visible_in_dir(cursor.clone(), 0, -1)
}

fn visible_trees_in_dir(cursor: GridCursor<u32>, down: i32, right: i32) -> usize {
    let mut visible_trees = 0;
    for c in cursor.iter_by(down, right) {
        if *c < *cursor {
            visible_trees += 1;
        } else {
            return visible_trees + 1;
        }
    }
    return visible_trees;
}

fn scenic_score(cursor: GridCursor<u32>) -> usize {
    visible_trees_in_dir(cursor.clone(), 1, 0)
        * visible_trees_in_dir(cursor.clone(), -1, 0)
        * visible_trees_in_dir(cursor.clone(), 0, 1)
        * visible_trees_in_dir(cursor.clone(), 0, -1)
}

pub fn part1() {
    let grid = read_input();
    let visible = grid.cursors().filter(|c| is_visible(c.clone())).count();
    println!("{}", visible);
}

pub fn part2() {
    let grid = read_input();
    let max_scenic_score = grid.cursors().map(|c| scenic_score(c.clone())).max().unwrap();
    println!("{}", max_scenic_score);
}