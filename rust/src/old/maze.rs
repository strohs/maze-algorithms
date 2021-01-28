use crate::direction::Direction;
//use std::fmt::{Display, Formatter};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

/// Cells in the maze are essentially u8 values
pub type Cell = u8;

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    field: Vec<Cell>,
}

impl Maze {
    /// creates an empty maze with the specified width and height
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            field: vec![0; width * height],
        }
    }

    /// convert a two-dimensional index to 1d
    fn to_1d(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// returns true if the given x and y coordinates are within the bounds of the maze
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        y < self.height && x < self.width
    }

    /// return the indices of cells that are the North,South,East,West neighbors of the
    /// cell located at cell x,y in this maze
    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let xi = x as isize;
        let yi = y as isize;
        let in_range = |x_idx: isize, y_idx: isize| {
            (0..self.width as isize).contains(&x_idx) && (0..self.height as isize).contains(&y_idx)
        };

        let deltas = [(0_isize, -1_isize), (0, 1), (-1, 0), (1, 0)];
        deltas
            .iter()
            .map(|&(dx, dy)| (dx + xi, dy + yi))
            .filter_map(|(nx, ny)| {
                if in_range(nx, ny) {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    /// returns the Direction of the wall that would result from carving a path from fx,fy to tx,ty
    pub fn carve_passage(fx: usize, fy: usize, tx: usize, ty: usize) -> Direction {
        if fx < tx {
            Direction::E
        } else if fx > tx {
            Direction::W
        } else if fy < ty {
            Direction::S
        } else {
            // fy > ty
            Direction::N
        }
    }

    /// checks if the cell at the given `x`,`y` coordinate has a wall removed in the direction
    /// specified by `dir`.
    /// returns `Some(true)` if the wall is removed. `Some(false)` if the wall is NOT removed.
    /// `None` if the given `x,y` coordinate is out of bounds of the maze
    pub fn is_carved(&self, x: usize, y: usize, dir: Direction) -> Option<bool> {
        let idx = self.to_1d(x, y);
        self.field.get(idx).map(|cell| *cell & dir as u8 != 0)
    }
}

impl Display for Maze {
    /// pretty prints the maze to using ascii characters: "|", "-" and "_" to represent walls.
    /// The maze is printed row by row, starting from the North-West corner and finishing in
    /// the south-east corner
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let top = " _".repeat(self.width);
        writeln!(f, "{}", top)?;

        for y in 0..self.height {
            // at the start of a row, print the WEST-most wall
            write!(f, "|")?;

            for x in 0..self.width {
                // if the south wall is carved print " " else print "_"
                match self.is_carved(x, y, Direction::S) {
                    Some(true) => write!(f, " ")?,
                    Some(false) => write!(f, "_")?,
                    _ => (),
                }

                match self.is_carved(x, y, Direction::E) {
                    // the east wall is carved
                    Some(true) => {
                        // this match is used to "join" the southern walls of adjacent cells
                        // to make them look "prettier", versus having a space gap between
                        // adjacent cells
                        match self.is_carved(x + 1, y, Direction::S) {
                            Some(true) => write!(f, " ")?,
                            Some(false) => write!(f, "_")?,
                            _ => (),
                        }
                    }
                    // east wall is present, print a wall "|"
                    Some(false) => write!(f, "|")?,
                    _ => (),
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// allow for indexing into the maze using a 2D-matrix like syntax, [[row, col]]
/// i.e.:  `maze[[y, x]]`
impl Index<[usize; 2]> for Maze {
    type Output = Cell;

    fn index(&self, yx_index: [usize; 2]) -> &Self::Output {
        // convert the x,y index into a 1d index
        let idx = self.to_1d(yx_index[1], yx_index[0]);
        &self.field[idx]
    }
}

impl IndexMut<[usize; 2]> for Maze {
    fn index_mut(&mut self, yx_index: [usize; 2]) -> &mut Self::Output {
        // convert the x,y index into a 1d index
        let idx = self.to_1d(yx_index[1], yx_index[0]);
        &mut self.field[idx]
    }
}
