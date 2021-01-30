use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::position::Pos;

/// Holds the position of a cell within a Grid as well as the positions of neighboring cells.
/// Two grid cells are considered equal if their respective positions are equal
#[derive(Debug, Copy, Clone)]
pub struct GridCell {
    // position of this cell within a grid
    pos: Pos,

    // positions of any neighboring cells
    north: Option<Pos>,
    south: Option<Pos>,
    east: Option<Pos>,
    west: Option<Pos>,
}

impl GridCell {
    pub fn new(
        pos: Pos,
        north: Option<Pos>,
        south: Option<Pos>,
        east: Option<Pos>,
        west: Option<Pos>,
    ) -> Self {
        Self {
            pos,
            north,
            south,
            east,
            west,
        }
    }

    /// constructs a GridCell with the specified position, but with option fields set to None
    /// and an empty links HashSet
    pub fn empty(pos: Pos) -> Self {
        Self {
            pos,
            north: None,
            south: None,
            east: None,
            west: None,
        }
    }

    /// returns the (row, col) position of this GridCell
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// returns the position of the cell to the north of this GridCell, if it has one, else `None`
    pub fn north(&self) -> Option<Pos> {
        self.north
    }

    /// returns the position of the cell to the south of this GridCell, else `None`
    pub fn south(&self) -> Option<Pos> {
        self.south
    }

    /// returns the position of the cell to the east of this GridCell, else `None`
    pub fn east(&self) -> Option<Pos> {
        self.east
    }

    /// returns the position of the cell to the west of this GridCell, else `None`
    pub fn west(&self) -> Option<Pos> {
        self.west
    }

    /// returns a vector containing the positions of grid cells that are neighbors of this cell
    #[allow(dead_code)]
    pub fn neighbors(&self) -> Vec<Pos> {
        let mut neighbors = Vec::with_capacity(4);
        if self.north.is_some() {
            neighbors.push(self.north.unwrap());
        }
        if self.south.is_some() {
            neighbors.push(self.south.unwrap());
        }
        if self.east.is_some() {
            neighbors.push(self.east.unwrap());
        }
        if self.west.is_some() {
            neighbors.push(self.west.unwrap());
        }
        neighbors
    }
}

impl PartialEq for GridCell {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for GridCell {}

impl Hash for GridCell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}
