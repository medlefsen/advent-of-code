use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub trait Grid {
    type Item;

    fn get(&self, row: i32, col: i32) -> &Self::Item;
    fn set(&mut self, row: i32, col: i32, val: Self::Item);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn resize(&mut self, rows: usize, cols: usize, val: Self::Item);
    fn cursors(&self) -> GridIterator<Self>;
    fn cursor_at(&self, row: i32, col: i32) -> Option<GridCursor<Self>>;
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
        if row < 0 || col < 0
            || row as usize >= grid.height()
            || col as usize >= grid.width()
        {
            None
        } else {
            Some(Self{ grid, col, row })
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

impl<T> From<Vec<Vec<T>>> for VecGrid<T> {
    fn from(cells: Vec<Vec<T>>) -> Self {
        Self { cells }
    }
}

impl<T: Clone> Grid for VecGrid<T> {
    type Item = T;

    fn get(&self, row: i32, col: i32) -> &T {
        &self.cells[row as usize][col as usize]
    }

    fn set(&mut self, row: i32, col: i32, val: T) {
        self.cells[row as usize][col as usize] = val;
    }

    fn width(&self) -> usize {
        self.cells[0].len()
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn resize(&mut self, rows: usize, cols: usize, val: T) {
        self.cells.resize(rows, Vec::new());
        for cell in &mut self.cells {
            cell.resize(cols, val.clone())
        }
    }
    fn cursors(&self) -> GridIterator<Self> {
        GridIterator { cursor: self.cursor_at(0,0) }
    }
    fn cursor_at(&self, row: i32, col: i32) -> Option<GridCursor<Self>> {
        GridCursor::new(self, row, col)
    }
}
