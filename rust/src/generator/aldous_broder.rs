use crate::grid::Grid;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Generates a random maze using the Aldous-Broder algorithm.
/// The idea behind it is as follows:
///
/// 1. Start anywhere in the grid you want, and choose a random neighbor.
/// 2. Move to that neighbor, and if it hasn’t previously been visited, link it to the prior cell.
/// 3. Repeat until every cell has been visited.
pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);

    // start at a random cell position
    let mut curr_pos = grid.random_pos();
    let mut unvisited = grid.size() - 1;

    while unvisited > 0 {
        // choose a random neighbor of the current_cell
        let neighbor_pos = *grid[curr_pos]
            .neighbors()
            .choose(&mut thread_rng())
            .expect("cells in a grid will have at least 2 neighbors");

        // if the neighbor pos is not linked to anything, then link it to the current cell
        if grid.links(&neighbor_pos).is_none() {
            grid.link(&curr_pos, &neighbor_pos, true);
            unvisited -= 1;
        }

        curr_pos = grid[neighbor_pos].pos();
    }

    grid
}
