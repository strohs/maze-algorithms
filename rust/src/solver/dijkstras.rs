use crate::grid::Grid;
use crate::position::Pos;
use crate::solver::distances::Distances;

/// find the distances from a `root` (cell Pos) to all other cells in the `grid`, using each cell's
/// weight to compute the cost.
/// returns a `Distances` struct containing the computed costs for each GridCell.
fn distances(grid: &Grid, root: Pos) -> Distances {

    // weights holds the Positions and current costs (weights) of the shortest path
    let mut weights = Distances::new(root);

    // pending holds positions that need to be visited
    let mut pending = vec![root];

    while !pending.is_empty() {

        // sort pending so that cells with lowest weight are at the end of pending
        pending.sort_unstable_by(|ap, bp| grid[*bp].weight().cmp(&grid[*ap].weight()) );

        // pop the last position from pending, it has the lowest weight
        let cur_pos = pending.pop().unwrap();

        // iterate thru the linked neighbors and compute the cost of moving into
        // each of them
        for neighbor in grid.links(&cur_pos) {

            // the total weight of moving into a neighboring cell is the total weight
            // of the current path so far, plus the weight of the neighbor
            let total_weight = weights.get(&cur_pos).unwrap() + grid[neighbor].weight();

            // if the cost of moving into neighbor has not been recorded in the weights vec
            // OR the total cost of moving to neighbor is less than the current weight
            if weights.get(&neighbor).is_none() || total_weight < *weights.get(&neighbor).unwrap() {
                pending.push(neighbor);
                weights.insert(neighbor, total_weight);
            }
        }
    }
    weights
}

/// finds the shortest path in the `maze`, beginning at `start` and finishing at `goal`
/// returns a `Distances` struct that only contains the positions of cells on the shortest
/// path
pub fn find_shortest_path(maze: &Grid, start: Pos, goal: Pos) -> Distances {
    // compute distances for all cells in the maze beginning at start Pos
    let maze_dist = distances(maze, start);

    // start from the goal and work backwards towards start
    let mut current = goal;

    // curr_path will only contain positions that are on the shortest path from goal to start
    let mut curr_path = Distances::new(start);
    // insert the current cell into curr_path
    curr_path.insert(current, maze_dist[current]);

    loop {
        if current == start {
            break;
        }

        // get the positions of all neighbors to the current cell's position
        for neighbor_pos in maze.links(&current) {
            // if the neighbor's distance is less than the current cell's distance, insert
            // the neighbor cell into curr_path, and make that neighbor the current cell
            if maze_dist[neighbor_pos] < maze_dist[current] {
                curr_path.insert(neighbor_pos, maze_dist[neighbor_pos]);
                current = neighbor_pos;
                break;
            }
        }
    }

    curr_path
}
