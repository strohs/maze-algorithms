
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::maze::grid_maze::GridMaze;

/// Generates a random maze using the Aldous-Broder algorithm.
/// Aldous-Broder generates mazes using "random-walks". This avoids creating mazes
/// with biases (like Binary Tree) and instead produces mazes with lots of winding passages.
///
/// The idea behind it is as follows:
///
/// 1. Start anywhere in the maze you want, and choose a random neighbor.
/// 2. Move to that neighbor, and if it has not previously been visited, link it to the prior node.
/// 3. Repeat until every node has been visited.
pub fn generate(height: usize, width: usize) -> GridMaze {
    let mut maze = GridMaze::new(height, width);

    // start at a random node position
    let mut cur_node = maze.random_node();
    let mut unvisited = maze.len() - 1;

    while unvisited > 0 {
        // choose a random neighbor of the current_node
        let rand_neighbor = *maze
            .neighbors(&cur_node)
            .choose(&mut thread_rng())
            .expect("all nodes in a maze will have at least 2 neighbors");

        // if the rand_neighbor is not linked to anything (i.e. it is unvisited), then link it
        // to the current node
        if maze.get_links(&rand_neighbor).is_empty() {
            maze.link(&cur_node, &rand_neighbor, true);
            unvisited -= 1;
        }

        cur_node = rand_neighbor;
    }

    maze
}
