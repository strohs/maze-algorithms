use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::maze::grid_maze::GridMaze;
use crate::maze::grid_node::GridNode;

/// Returns a maze generated using the recursive-backtracker algorithm
///
/// Here’s the mile-high view of **recursive backtracker**:
///
/// 1. Choose a random starting point in the maze.
/// 2. Randomly choose a random neighbor that has not been visited and link to it. This neighbor
///    becomes the new current cell.
/// 3. If all neighbor cells have been visited, back up to the last cell that has unvisited
///    neighbors and repeat.
/// 4. The algorithm ends when the process has backed all the way up to the starting point.
///
/// in essence, this carves passages using a depth-first search with back-tracking.
/// Also, like hunt-and-kill, recursive-backtracker also produces mazed that are full of long
/// and meandering passages.
pub fn generate(height: usize, width: usize) -> GridMaze {
    let mut maze = GridMaze::new(height, width);

    // pick a random position to start at
    let start = maze.random_node();

    // the stack of visited maze positions
    let mut stack = vec![start];

    // while we have positions left to visit...
    while let Some(current_node) = stack.last() {
        // get neighbors of the current node that are not linked to other nodes (unvisited)
        let unlinked_neighbors = maze
            .neighbors(current_node)
            .iter()
            .filter(|&node| maze.get_links(node).is_empty())
            .map(|node| *node)
            .collect::<Vec<GridNode>>();

        // if there are unvisited neighbors choose a random neighbor, link to it, and push it
        // onto the stack.
        if !unlinked_neighbors.is_empty() {
            if let Some(rand_neighbor) = unlinked_neighbors.choose(&mut thread_rng()) {
                maze.link(current_node, rand_neighbor, true);
                stack.push(*rand_neighbor);
            }
        } else {
            // else every neighbor has been visited, so backtrack by popping the current pos
            // off of the stack
            stack.pop();
        }
    }

    maze
}