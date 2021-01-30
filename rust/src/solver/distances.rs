use crate::position::Pos;
use crate::grid_cell::GridCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::collections::hash_map::Keys;


/// holds distances information from the root cell, to every other cell in the grid.
#[derive(Debug)]
pub struct Distances {
    // Distances is essentially a wrapper over a HashMap

    // root is the starting position in the grid
    root: Pos,
    // stores the 'distance' from the cell at the given Pos, to the root cell.
    cells: HashMap<Pos, u32>,
}

impl Distances {
    pub fn new(root: Pos) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        Self {
            root,
            cells,
        }
    }

    pub fn get(&self, pos: &Pos) -> Option<&u32> {
        self.cells.get(pos)
    }

    /// insert the cell position and distance
    pub fn insert(&mut self, cell_pos: Pos, distance: u32) {
        self.cells.insert(cell_pos, distance);
    }

    /// returns an iterator over the `Pos`itions in stored within the Distances HashMap
    pub fn keys(&self) -> Keys<'_, Pos, u32> {
        self.cells.keys()
    }
}

/// index Distances using a `Pos`. This passes through to cells' HashMap `Index` implementation
impl Index<Pos> for Distances {
    type Output = u32;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.cells[&pos]
    }
}
