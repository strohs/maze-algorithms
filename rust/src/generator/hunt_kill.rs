use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::maze::grid_maze::GridMaze;
use crate::maze::grid_node::GridNode;


/// Returns a maze generated using the Hunt-and-Kill algorithm.
/// Hunt-and-kill is similar to Aldous-Broder but slightly different. Where Aldous-Broder allows
/// you to step into any node (even previously visited ones), hunt-and-kill requires you to walk
/// on only unvisited nodes. If you walk into a corner, you begin the "hunt" mode, which is where
/// you start from the top of the maze, look for the first node that is a neighbor of the nodes in
/// the current walk you are performing, and then link into the walk. Then repeat the random walk
/// until all nodes are visited
///
/// Hunt-and-Kill is known to produce mazes with longer winding and meandering corridors than
/// other algorithms. That is to say, hunt-and-kill produces mazes with fewer dead ends.
pub fn generate(height: usize, width: usize) -> GridMaze {
    let mut maze = GridMaze::new(height, width);

    // choose a random start node in the maze to begin the random walk
    let mut next_node = Some(maze.random_node());

    // continue walking until all nodes are visited
    while let Some(cur_node) = next_node {

        let unvisited_neighbors = unvisited_neighbors(&maze, &cur_node);

        // this is the random walk, if the cur_node has unvisited neighbors, we will link
        // the cur_node to a random unvisited neighbor and then make that random neighbor
        // the cur_node
        if !unvisited_neighbors.is_empty() {
            if let Some(rand_neighbor_node) = unvisited_neighbors.choose(&mut thread_rng()) {
                maze.link(&cur_node, rand_neighbor_node, true);
                next_node = Some(*rand_neighbor_node);
            }
        } else {
            // else we begin the hunt phase, starting from the top of the maze, looking for the
            // first node that is unvisited AND has neighbors that are visited

            next_node = None;

            // begin iterating the maze by index starting at the top-left
            for cur_index in 0..maze.len() {

                let cur_hunt_node = maze[cur_index];

                // get any visited neighbors of the node at cur_index
                let visited_neighbors = visited_neighbors(&maze, &cur_hunt_node);

                // if the current hunt node is unvisited BUT one of its neighbors is visited, then
                // link the current_hunt_node to the random neighbor node and set the next_node to
                // visit (in the random walk) to the current hunt node
                if maze.get_links(&cur_hunt_node).is_empty() && !visited_neighbors.is_empty() {
                    next_node = Some(cur_hunt_node);

                    if let Some(rand_neighbor_node) = visited_neighbors.choose(&mut thread_rng()) {
                        maze.link(&cur_hunt_node, rand_neighbor_node, true);
                    }
                    // break so we can repeat the random walk
                    break;
                }
            }
        }
    }

    maze
}

/// Returns a vector of nodes that are neighbors of the given `node` AND that do not have links to
/// any other nodes in the `maze`
fn unvisited_neighbors(maze: &GridMaze, node: &GridNode) -> Vec<GridNode> {
    maze
        .neighbors(node)
        .iter()
        .filter(|&neighbor_node| maze.get_links(neighbor_node).is_empty())
        .copied()
        .collect()
}


/// Returns a vector of positions that are neighbors of the given `node` AND that have a link
/// to another node in the maze
fn visited_neighbors(maze: &GridMaze, node: &GridNode) -> Vec<GridNode> {
    maze
        .neighbors(node)
        .iter()
        .filter(|&neighbor_node| !maze.get_links(neighbor_node).is_empty())
        .copied()
        .collect()
}