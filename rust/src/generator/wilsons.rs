use crate::grid::Grid;
use rand::{thread_rng};
use rand::seq::SliceRandom;
use crate::position::Pos;
use std::ops::Index;

/// Generates a random maze using Wilson's algorithm:
/// Like Aldous-Broder, this algorithm depends on the idea of a random walk, but with a twist.
/// It performs what is called a loop-erased random walk, which means that as it goes, if the path
/// it is forming happens to intersect with itself and form a loop, it erases that loop before
/// continuing on.
///
/// 1. choose a point on the grid and mark it visited.
/// 2. choose any unvisited cell in the grid and perform a loop-erased random walk until you
///    reach a visited cell.
/// 3. link all the cells in the current random walk to the visited cell
/// 4. repeat step 2 until all cells in the grid have been visited
pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);

    // choose a random position in the grid, this will be the first visited position
    let first = grid.random_pos();
    // initialize unvisited to contain all positions in the grid except for first
    let mut unvisited: Vec<Pos> = grid
        .iter_cells()
        .map(|c| c.pos()).filter(|p| *p != first)
        .collect();

    // while there are still unvisited cells
    while !unvisited.is_empty() {
        // choose a random, unvisited cell and add it to the `path` that is about to be walked
        let mut cell_pos = *unvisited.choose(&mut thread_rng()).unwrap();
        // path contains positions of randomly walked cells
        let mut path: Vec<Pos> = vec![cell_pos];

        // while the current_pos is in the unvisited vec
        while unvisited.contains(&cell_pos) {
            // choose a random neighbor of the current cell position
            cell_pos = *grid[cell_pos]
                .neighbors()
                .choose(&mut thread_rng())
                .expect("all cells will have at least two neighbors");

            // if the random neighbor is already in path, there is a loop, so remove it
            if let Some(pos_idx) = path.iter().position(|p| *p == cell_pos) {
                path = path[0..=pos_idx].to_vec();
            } else {
                // cell_pos is not going to make a loop, so push it onto the path
                path.push(cell_pos);
            }
        }

        // carve passages (i.e. link) between the cells in path
        let mut window = path.windows(2);
        while let Some([cur_pos, next_pos]) = window.next() {
            grid.link(cur_pos, next_pos, true);

            // remove the cells in the path from the vec of unvisited cells
            if let Some(path_idx) = unvisited.iter().position(|p| *p == *cur_pos) {
                unvisited.remove(path_idx);
            }
        }
    }

    grid
}

