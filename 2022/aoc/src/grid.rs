use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(cells: Vec<Vec<T>>) -> Self {
        Self { cells }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct GridCursor<'a, T> {
    grid: &'a Grid<T>,
    pub row: i32,
    pub col: i32,
}

impl<'a, T> Clone for GridCursor<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Deref for GridCursor<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.grid.cells[self.row as usize][self.col as usize]
    }
}


impl<T> Grid<T> {
    pub fn cursors(&self) -> GridIterator<T> {
        GridIterator { cursor: self.cursor_at(0,0) }
    }
    pub fn cursor_at(&self, row: i32, col: i32) -> Option<GridCursor<T>> {
        GridCursor::new(self, row, col)
    }
}

impl<'a, T> Debug for GridCursor<'a,T>
where T: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridCursor")
            .field("row", &self.row)
            .field("col", &self.col)
            .field("val", self.deref())
            .finish()
    }
}

impl<'a, T> GridCursor<'a, T>
{
    fn new(grid: &'a Grid<T>, row: i32, col: i32) -> Option<Self> {
        if row < 0 || col < 0
            || row as usize >= grid.cells.len()
            || col as usize >= grid.cells[0].len()
        {
            None
        } else {
            Some(Self{ grid, col, row })
        }
    }
    pub fn move_by(&self, down: i32, right: i32) -> Option<Self> {
        Self::new(self.grid, self.row + down, self.col + right)
    }
    pub fn iter_by(&self, down: i32, right: i32) -> GridDirIterator<'a, T> {
        GridDirIterator { cursor: self.clone(), down, right }
    }
    pub fn right(&self) -> Option<Self> { self.move_by(0, 1) }
    pub fn left(&self) -> Option<Self> { self.move_by(0, -1) }
    pub fn up(&self) -> Option<Self> { self.move_by(-1, 0) }
    pub fn down(&self) -> Option<Self> { self.move_by(1, 0) }
}

pub struct GridIterator<'a, T> {
    cursor: Option<GridCursor<'a, T>>,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = GridCursor<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cursor.clone();
        self.cursor = cur.clone().and_then(|c| c.right().or(
            c.iter_by(0, -1).last().and_then(|s| s.down()))
        );
        cur
    }
}

pub struct GridDirIterator<'a, T> {
    cursor: GridCursor<'a, T>,
    down: i32,
    right: i32,
}

impl<'a, T> Iterator for GridDirIterator<'a, T> {
    type Item = GridCursor<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.move_by(self.down, self.right).map(|cursor| {
            self.cursor = cursor.clone();
            cursor
        })
    }
}
