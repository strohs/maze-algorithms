
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::maze::grid_maze::GridMaze;

/// Generates a random maze using the Binary Tree algorithm.
///
/// Binary Tree algorithm is one of the simplest maze generation algorithms:
/// 1. start at a corner of the maze (in this case it will be the North West)
/// 2. iterate through the nodes row by row
/// 3. for each node pick a random East or South wall to remove
/// 4. repeat until all nodes have been visited
pub fn generate(height: usize, width: usize) -> GridMaze {
    let mut maze = GridMaze::new(height, width);

    // iterate over node indices
    for node_index in 0..maze.len() {

        let curr_node = maze[node_index];

        let mut neighbors = vec![];

        // if current cell has a south neighbor, add that neighbors position to neighbors
        if let Some(south_neighbor) = maze.south(&curr_node) {
            neighbors.push(south_neighbor);
        }

        // if current cell has a east neighbor, add that neighbors position to neighbors
        if let Some(east_neighbor) = maze.east(&curr_node) {
            neighbors.push(east_neighbor);
        }

        // choose a random neighbor from neighbors and create a link to it
        if let Some(rand_neighbor) = neighbors.choose(&mut thread_rng()) {
            maze.link(&curr_node, rand_neighbor, true);
        }
    }

    maze
}
