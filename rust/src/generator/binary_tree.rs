use crate::grid::Grid;
use crate::position::Pos;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// Generates a random maze using the Binary Tree algorithm.
///
/// Binary Tree algorithm is one of the simplest maze generation algorithms:
/// 1. start at a corner of the maze (in this case it will be the North West)
/// 2. iterate through the cells row by row
/// 3. for each cell pick a random East or South wall to remove
/// 4. repeat until all cells have been visited
pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);

    for pos in Pos::iter(height, width) {
        let cell = grid[pos];

        let mut neigbors = vec![];

        // if current cell has a south neighbor, add that neighbors position to neighbors
        cell.south().map(|p| neigbors.push(p));

        // if current cell has a east neighbor, add that neighbors position to neighbors
        cell.east().map(|p| neigbors.push(p));

        // choose a random east or west neighbor and create a link to it
        neigbors
            .choose(&mut thread_rng())
            .map(|neigh_pos| grid.link(&cell.pos(), neigh_pos, true));
    }

    grid
}
