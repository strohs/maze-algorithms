use crate::grid::Grid;
use rand::{thread_rng, Rng};
use crate::position::Pos;

///
/// generates a random maze using Prims algorithm.
///
/// Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
/// entire graph, it starts at one point, and grows outward from that point.
/// The standard version of the algorithm works something like this:
///   1. Choose an arbitrary cell from G (the grid), and add it to some (initially empty) set V (toVisit).
///   2. select a cell (currCell) from V with the lowest weight
///   3. get all unlinked neighbors of currCell and select the neighbor with the lowest weight (neighbor)
///   4. if a neighbor was found:
///   5.   link currentCell to it
///   6.   add neighbor to V
///   7. else (backed into a corner of the maze)
///   8.   remove current from V
///   9. repeat steps 2 thru 9 until there are no longer cells in V
///
/// # Params
/// `height` - the number of rows to generate
/// `width` - the number of columns to generate
/// # Returns
/// a `grid` containing the randomly generated maze
///
pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);

    // assign random weights to all cells in the grid
    for cell in grid.iter_mut_cells() {
        cell.set_weight(thread_rng().gen_range(1, 101));
    }

    let mut to_visit = vec![grid.random_pos()];

    while !to_visit.is_empty() {
        // sort to_visit by cell weight
        to_visit.sort_by_key(|p| grid[*p].weight());
        let cell_pos = to_visit[0];

        let mut neighbors = unlinked_neighbors(&grid, cell_pos);

        if !neighbors.is_empty() {
            neighbors.sort_by_key(|p| grid[*p].weight());
            // link cell_pos to the lowest weighted neighbor
            grid.link(&cell_pos, &neighbors[0], true);
            to_visit.push(neighbors[0]);
        } else {
            let ridx = to_visit.iter().position(|p| *p == cell_pos).unwrap();
            to_visit.remove(ridx);
        }
    }

    grid
}

/// returns the positions of any unlinked neighbors of `pos`
fn unlinked_neighbors(grid: &Grid, pos: Pos) -> Vec<Pos> {
    grid[pos].neighbors()
        .iter()
        .filter(|&p| grid.links(p).is_empty())
        .map(|p| *p)
        .collect::<Vec<Pos>>()
}