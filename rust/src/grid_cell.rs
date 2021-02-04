use std::hash::{Hash, Hasher};

use crate::position::Pos;

/// Holds the position of a cell within a Grid as well as the positions of neighboring cells.
/// Two grid cells are considered equal if their respective positions (row,col values) are equal
#[derive(Debug, Copy, Clone)]
pub struct GridCell {
    // row,col position of this cell within a grid
    pos: Pos,

    // positions of any neighboring cells
    north: Option<Pos>,
    south: Option<Pos>,
    east: Option<Pos>,
    west: Option<Pos>,

    // the weight (or cost) of this cell
    weight: i32,
}

impl GridCell {
    pub fn new(
        pos: Pos,
        north: Option<Pos>,
        south: Option<Pos>,
        east: Option<Pos>,
        west: Option<Pos>,
        weight: i32,
    ) -> Self {
        Self {
            pos,
            north,
            south,
            east,
            west,
            weight,
        }
    }

    /// constructs a GridCell with the specified position, but with option fields set to None
    /// and an empty links HashSet. The initial weight of the GridCell is set to 1
    pub fn empty(pos: Pos) -> Self {
        Self {
            pos,
            north: None,
            south: None,
            east: None,
            west: None,
            weight: 1,
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

    /// returns the weight of this GridCell
    pub fn weight(&self) -> i32 {
        self.weight
    }

    /// sets the weight of this grid cell
    pub fn set_weight(&mut self, weight: i32) {
        self.weight = weight;
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
