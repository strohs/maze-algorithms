use crate::grid::Grid;
use crate::position::Pos;
use std::collections::HashMap;
use std::ops::Index;

/// Distances is a helper struct that holds how far every cell in a Grid os from a `root` cell.
/// These distances are used by certain shortest-path algorithms (like Dijkstra's)
#[derive(Debug)]
pub struct Distances {
    // Distances is essentially a wrapper over a HashMap

    // root is the starting position in the grid
    root: Pos,
    // stores the 'distance' from the cell at `Pos`, to the `root` cell.
    cells: HashMap<Pos, i32>,
}


impl Distances {

    /// returns a new Distance struct with the specified `root` Position as the root of
    /// the returned distance struct.
    pub fn new(root: Pos) -> Self {
        let mut cells = HashMap::new();
        // the root is inserted into the hashmap with a distance of 0 from itself
        cells.insert(root, 0);

        Self { root, cells }
    }


    /// returns the distance information for the cell at the specified `pos`. Returns `None` if
    /// the cell is not contained within Distances
    pub fn get(&self, pos: &Pos) -> Option<&i32> {
        self.cells.get(pos)
    }


    /// insert the cell position and distance
    pub fn insert(&mut self, cell_pos: Pos, distance: i32) {
        self.cells.insert(cell_pos, distance);
    }

    // /// returns an iterator over the `Pos`itions in stored within the Distances HashMap
    // pub fn keys(&self) -> Keys<'_, Pos, u32> {
    //     self.cells.keys()
    // }
}

/// index Distances using a `Pos`ition struct.
/// This implementation passes to a cell's HashMap `Index` implementation
impl Index<Pos> for Distances {
    type Output = i32;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.cells[&pos]
    }
}

/// Returns a "pretty printed" String containing the distance amounts of each cell of the grid
/// displayed within the cells as a Hexa-Decimal amount.
/// Useful for debugging purposes
#[allow(dead_code)]
pub fn display_distances(grid: &Grid, distances: &Distances) -> String {
    let mut buf = String::new();
    // write the top wall of the grid
    buf.push_str(&format!("+{}", "---+".repeat(grid.cols)));

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
                Some(east_pos) if grid.has_link(&cell.pos(), &east_pos) => {
                    top.push_str(&format!(" {:2x} ", dist))
                }
                _ => top.push_str(&format!(" {:2x}|", dist)),
            }

            // determine if a southern wall should be drawn
            match cell.south() {
                Some(south_pos) if grid.has_link(&cell.pos(), &south_pos) => {
                    bottom.push_str("   +")
                }
                _ => bottom.push_str("---+"),
            }
        }

        buf.push_str(&top.to_string());
        buf.push_str(&bottom.to_string());
    }
    buf
}
