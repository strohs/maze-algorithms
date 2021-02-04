use crate::grid::Grid;
use crate::position::Pos;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Returns a maze generated using the recursive-backtracker algorithm
///
/// Here’s the mile-high view of **recursive backtracker**:
///
/// 1. Choose a random starting point in the grid.
/// 2. Randomly choose a random neighbor that has not been visited and link to it. This neighbor
///    becomes the new current cell.
/// 3. If all neighbor cells have been visited, back up to the last cell that has unvisited
///    neighbors and repeat.
/// 4. The algorithm ends when the process has backed all the way up to the starting point.
///
/// in essence, this carves passages using a depth-first search with back-tracking.
/// Also, like hunt-and-kill, recursive-backtracker also produces mazed that are full of long
/// and meandering passages.
pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);

    // pick a random position to start at
    let start = grid.random_pos();

    // the stack of visited grid positions
    let mut stack = vec![start];

    // while we have positions left to visit...
    while let Some(current_pos) = stack.last() {
        // get neighbors of the current position that are not linked to other cells (unvisited)
        let neighbors = grid[*current_pos]
            .neighbors()
            .iter()
            .filter(|&pos| grid.links(pos).is_empty())
            .map(|pos| *pos)
            .collect::<Vec<Pos>>();

        // if there are unvisited neighbors choose a random neighbor, link to it, and push it
        // onto the stack.
        if !neighbors.is_empty() {
            if let Some(rand_neighbor_pos) = neighbors.choose(&mut thread_rng()) {
                grid.link(current_pos, rand_neighbor_pos, true);
                stack.push(*rand_neighbor_pos);
            }
        } else {
            // else every neighbor has been visited, so backtrack by popping the current pos
            // off of the stack
            stack.pop();
        }
    }

    grid
}