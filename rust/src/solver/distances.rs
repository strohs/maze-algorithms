use crate::position::Pos;
use crate::grid_cell::GridCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::fmt::Write;
use std::collections::hash_map::Keys;
use crate::grid::Grid;


/// holds distances information from the root cell, to every other cell in the grid.
#[derive(Debug)]
pub struct Distances {
    // Distances is essentially a wrapper over a HashMap

    // root is the starting position in the grid
    root: Pos,
    // stores the 'distance' from the cell at `Pos`, to the `root` cell.
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


/// pretty prints the grid along with its cell distances information,  into the supplied `buf`fer.
pub fn display_distances(grid: &Grid, distances: &Distances) -> String {
    let mut buf = String::new();
    // write the top wall of the grid
    writeln!(buf, "+{}", "---+".repeat(grid.cols));

    for row in grid.row_iter() {
        // top holds the cell 'bodies' (blank spaces) and eastern walls
        let mut top = String::from("|");
        // bottom holds the cell's southern wall and corners ('+') sign
        let mut bottom = String::from("+");

        for cell in row.iter() {
            let dist = distances.get(&cell.pos()).unwrap();
            // the body of the cell will display the distance from the root
            // determine if an eastern wall should be drawn
            match cell.east() {
                Some(east_pos) if grid.has_link(&cell.pos(), &east_pos) => top.push_str(&format!(" {:2x} ", dist )),
                _ => top.push_str(&format!(" {:2x}|", dist )),
            }

            // determine if a southern wall should be drawn
            match cell.south() {
                Some(south_pos) if grid.has_link(&cell.pos(), &south_pos) => {
                    bottom.push_str("   +")
                }
                _ => bottom.push_str("---+"),
            }
        }

        writeln!(buf, "{}", top);
        writeln!(buf, "{}", bottom);
    }
    buf
}
