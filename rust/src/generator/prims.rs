use rand::{thread_rng, Rng};
use crate::maze::grid_maze::GridMaze;
use crate::maze::grid_node::GridNode;

/// Generates a random maze using Prims algorithm.
///
/// Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
/// entire maze, it starts at one point, and grows outward from that point.
/// The standard version of the algorithm works something like this:
///   1. Choose an arbitrary cell from G (the maze), and add it to some (initially empty) set V (toVisit).
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
/// a `GridMaze` containing the randomly generated maze
///
pub fn generate(height: usize, width: usize) -> GridMaze {
    let mut maze = GridMaze::new(height, width);

    // assign random weights to all cells in the maze
    for node in maze.iter_mut_nodes() {
        node.set_weight(thread_rng().gen_range(1, 101));
    }

    // holds the nodes to be visited
    let mut to_visit = vec![maze.random_node()];

    while !to_visit.is_empty() {
        // sort the to_visit nodes by weight
        to_visit.sort_by_key(|node| node.weight());
        let cur_node = to_visit[0];

        let mut neighbors = unlinked_neighbors(&maze, &cur_node);

        if !neighbors.is_empty() {
            neighbors.sort_by_key(|node| node.weight());
            // link cur_node to the lowest weighted neighbor node
            maze.link(&cur_node, &neighbors[0], true);
            to_visit.push(neighbors[0]);
        } else {
            // remove cur_node from to_visit, it should always be the first element of to_visit
            to_visit.remove(0);
        }
    }

    maze
}

/// returns a vector of nodes that are unlinked neighbors of the given `node`
fn unlinked_neighbors(maze: &GridMaze, node: &GridNode) -> Vec<GridNode> {
    maze
        .neighbors(node)
        .iter()
        .filter(|&neighbor| maze.get_links(neighbor).is_empty())
        .copied()
        .collect()
}