use crate::grid::Grid;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::position::Pos;


/// Returns a maze generated using the Hunt-and-Kill algorithm.
/// Hunt-and-kill is similar to Aldous-Broder but slightly different. Where Aldous-Broder allows
/// you to step into any cell (even previously visited ones), hunt-and-kill requires you to walk
/// on only unvisited cells. If you walk into a corner, you begin the "hunt" mode, which is where
/// you start from the top of the maze, look for the first cell that is a neighbor of the cells in
/// the current walk you are performing, and then link into the walk. Then repeat the random walk
/// until all cells are visited
///
/// Hunt-and-Kill is known to produce mazes with longer winding and meandering corridors than
/// other algorithms. That is to say, hunt-and-kill produces mazes with fewer dead ends.
pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);

    // choose a random start position in the maze to begin the random walk
    let mut current = Some(grid.random_pos());

    // continue walking until all cells are visited
    while let Some(current_pos) = current {

        let unvisited_neighbors = unvisited_neighbors(&grid, current_pos);

        // this is the random walk, if the current_pos has unvisited neighbors, we will link
        // the current_pos to a random unvisited neighbor and then make that random neighbor
        // the current pos
        if !unvisited_neighbors.is_empty() {
            if let Some(neighbor_pos) = unvisited_neighbors.choose(&mut thread_rng()) {
                grid.link(&current_pos, neighbor_pos, true);
                current = Some(*neighbor_pos);
            }
        } else {
            // else we begin the hunt phase, starting from the top of the maze, looking for the
            // first cell that is unvisited AND has neighbors that are visited

            current = None;

            // begin iterating the Grid by Position starting at the top-left
            for pos in Pos::iter(height, width) {

                // get any visited neighbors of the current Position
                let visited_neighbors = visited_neighbors(&grid, pos);

                // if position is unvisited BUT one of its neighbors is, then link pos to a
                // to a random neighbor and set current to pos
                if grid.links(&pos).is_empty() && !visited_neighbors.is_empty() {
                    current = Some(pos);

                    if let Some(neighbor_pos) = visited_neighbors.choose(&mut thread_rng()) {
                        grid.link(&pos, neighbor_pos, true);
                    }
                    // break so we can repeat the random walk
                    break;
                }
            }
        }
    }

    grid
}

/// Returns a vector of positions that are neighbors of the given `pos` AND that do not have
/// a link to a cell in the `maze`
fn unvisited_neighbors(grid: &Grid, pos: Pos) -> Vec<Pos> {
    grid[pos]
        .neighbors()
        .iter()
        .filter(|&p| grid.links(p).is_empty())
        .map(|p| *p)
        .collect()
}


/// Returns a vector of positions that are neighbors of the given `pos` AND that have a link
/// to a cell in the maze
fn visited_neighbors(grid: &Grid, pos: Pos) -> Vec<Pos> {
    grid[pos]
        .neighbors()
        .iter()
        .filter(|&p| !grid.links(p).is_empty())
        .map(|p| *p)
        .collect()
}