use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Deref;

pub trait Grid {
    type Item;

    fn get(&self, row: i32, col: i32) -> &Self::Item;
    fn set(&mut self, row: i32, col: i32, val: Self::Item);
    fn min_row(&self) -> i32;
    fn max_row(&self) -> i32;
    fn min_col(&self) -> i32;
    fn max_col(&self) -> i32;

    fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= self.min_row() && col >= self.min_col()
            && row <= self.max_row() && col <= self.max_col()
    }

    fn cursors(&self) -> GridIterator<Self> {
        GridIterator { cursor: self.cursor_at(self.min_row(),self.min_col()) }
    }
    fn cursor_at(&self, row: i32, col: i32) -> Option<GridCursor<Self>> {
        GridCursor::new(self, row, col)
    }
}


#[derive(PartialEq, Eq, Hash)]
pub struct GridCursor<'a, G: Grid + ?Sized> {
    grid: &'a G,
    pub row: i32,
    pub col: i32,
}

impl<'a, G: Grid + ?Sized> Clone for GridCursor<'a, G> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, G: Grid > Deref for GridCursor<'a, G> {
    type Target = G::Item;

    fn deref(&self) -> &Self::Target {
        self.grid.get(self.row, self.col)
    }
}

impl<'a, G: Grid + ?Sized> GridCursor<'a, G>
{
    fn new(grid: &'a G, row: i32, col: i32) -> Option<Self> {
        if grid.in_bounds(row, col) {
            Some(Self{ grid, col, row })
        } else {
            None
        }
    }
    pub fn move_by(&self, down: i32, right: i32) -> Option<Self> {
        Self::new(self.grid, self.row + down, self.col + right)
    }
    pub fn iter_by(&self, down: i32, right: i32) -> GridDirIterator<'a, G> {
        GridDirIterator { cursor: (*self).clone(), down, right }
    }
    pub fn right(&self) -> Option<Self> { self.move_by(0, 1) }
    pub fn left(&self) -> Option<Self> { self.move_by(0, -1) }
    pub fn up(&self) -> Option<Self> { self.move_by(-1, 0) }
    pub fn down(&self) -> Option<Self> { self.move_by(1, 0) }
}

impl<'a, G> Debug for GridCursor<'a,G>
    where
        G: Grid,
        G::Item: Debug

{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridCursor")
            .field("row", &self.row)
            .field("col", &self.col)
            .field("val", self.deref())
            .finish()
    }
}

pub struct GridIterator<'a, G: Grid + ?Sized> {
    cursor: Option<GridCursor<'a, G>>,
}

impl<'a, G: Grid> Iterator for GridIterator<'a, G> {
    type Item = GridCursor<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cursor.clone();
        self.cursor = cur.clone().and_then(|c| c.right().or(
            c.iter_by(0, -1).last().and_then(|s| s.down()))
        );
        cur
    }
}

pub struct GridDirIterator<'a, G: Grid + ?Sized> {
    cursor: GridCursor<'a, G>,
    down: i32,
    right: i32,
}

impl<'a, G: Grid> Iterator for GridDirIterator<'a, G> {
    type Item = GridCursor<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.move_by(self.down, self.right).map(|cursor| {
            self.cursor = cursor.clone();
            cursor
        })
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct VecGrid<T> {
    cells: Vec<Vec<T>>
}

impl<T> Grid for VecGrid<T> {
    type Item = T;

    fn get(&self, row: i32, col: i32) -> &T {
        &self.cells[row as usize][col as usize]
    }

    fn set(&mut self, row: i32, col: i32, val: T) {
        self.cells[row as usize][col as usize] = val;
    }

    fn min_row(&self) -> i32 { 0 }
    fn max_row(&self) -> i32 { self.cells.len() as i32 }
    fn min_col(&self) -> i32 { 0 }
    fn max_col(&self) -> i32 { self.cells[0].len() as i32 }
}

impl<T> From<Vec<Vec<T>>> for VecGrid<T> {
    fn from(cells: Vec<Vec<T>>) -> Self {
        Self { cells }
    }
}

#[derive(Default, Debug, PartialEq, Eq )]
pub struct HashMapGrid<T> {
    default: T,
    cells: HashMap<(i32, i32), T>,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
}

impl<T> HashMapGrid<T> {
    pub fn new(default: T) -> Self {
        Self {
            default,
            cells: HashMap::new(),
            min_row: i32::MAX,
            max_row: i32::MIN,
            min_col: i32::MAX,
            max_col: i32::MIN,
        }
    }
}

impl<T: Clone> Grid for HashMapGrid<T> {
    type Item = T;

    fn get(&self, row: i32, col: i32) -> &T {
        self.cells.get(&(row, col)).unwrap_or(&self.default)
    }

    fn set(&mut self, row: i32, col: i32, val: T) {
        self.min_col = min(col, self.min_col);
        self.min_row = min(row, self.min_row);
        self.max_col = max(col, self.max_col);
        self.max_row = max(row, self.max_row);
        self.cells.insert((row, col), val);
    }

    fn min_row(&self) -> i32 { self.min_row }
    fn max_row(&self) -> i32 { self.max_row }
    fn min_col(&self) -> i32 { self.min_col }
    fn max_col(&self) -> i32 { self.max_col }
}

pub fn format_grid<G>(grid: &G, f: &mut Formatter<'_>) -> std::fmt::Result
where G: Grid,
      G::Item: Display
{
    for c in grid.cursors() {
        f.write_fmt(format_args!("{}", *c))?;
        if c.col == grid.max_col() {
            f.write_char('\n')?;
        }
    }
    Ok(())
}

impl<T: Display> Display for VecGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_grid(self, f)
    }
}

impl<T: Display + Clone> Display for HashMapGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_grid(self, f)
    }
}
