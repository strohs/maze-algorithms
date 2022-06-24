use crate::solver::distances::{Distances};
use crate::maze::grid_maze::GridMaze;
use crate::maze::grid_node::GridNode;


/// finds the shortest path in the `maze`, beginning at `start` and finishing at `goal`
/// returns a `Distances` struct that only contains the positions of cells on the shortest
/// path
pub fn find_shortest_path(maze: &GridMaze, start: GridNode, goal: GridNode) -> Distances {
    // compute distances for all cells in the maze beginning at start Pos
    let maze_dist = maze.distances(&start);

    // start from the goal and work backwards towards start
    let mut cur_node = goal;

    // curr_path will only contain positions that are on the shortest path from goal to start
    let mut cur_path = Distances::new(start);
    // insert the current cell into curr_path
    cur_path.insert(cur_node, maze_dist[cur_node]);

    loop {
        if cur_node == start {
            break;
        }

        // get the positions of all neighbors to the current cell's position
        for neighbor_node in maze.get_links(&cur_node) {

            // if the neighbor's distance is less than the current cell's distance, insert
            // the neighbor cell into curr_path, and make that neighbor the current cell
            if maze_dist[neighbor_node] < maze_dist[cur_node] {
                cur_path.insert(neighbor_node, maze_dist[neighbor_node]);
                cur_node = neighbor_node;
                break;
            }
        }
    }

    cur_path
}
