use crate::grid::Grid;
use crate::solver::distances::Distances;
use crate::position::Pos;

use std::fmt::Write;


/// find the distances from a `root` (cell Pos) to all other cells in the `grid`
/// returns a `Distances` struct containing the computed distances for each cell
pub fn distances(grid: &Grid, root: Pos) -> Distances {
    let mut distances = Distances::new(root);
    let mut frontier = vec![root];

    let mut new_frontier = vec![];
    while !frontier.is_empty() {


        for cur_pos in frontier.pop() {
            // if the current cells has links to other cells...
            if let Some(linked_cells) = grid.links(&cur_pos) {
                // for each linked cell...
                for linked_pos in linked_cells {
                    // only visit cells that have not already been visited
                    if distances.get(linked_pos).is_none() {
                        // the linked cells distance is 1 + the previous cell's distance
                        distances.insert(*linked_pos, distances[cur_pos] + 1);
                        new_frontier.push(*linked_pos);
                    }
                }
            }
        }
        frontier.append(&mut new_frontier);
    }

    distances
}


/// finds the shortest path in the `maze`, beginning at `start` and finishing at `goal`
/// returns a `Distances` struct that only contains the positions of cells on the shortest
/// path
pub fn find_shortest_path(maze: &Grid, start: Pos, goal: Pos) -> Distances {
    // compute distances for all cells in the maze beginning at start Pos
    let maze_dist = distances(maze, start);

    // start from the goal and work backwards
    let mut current = goal;

    // bread_crumbs will only contain positions that are on the shortest path from goal to start
    let mut bread_crumbs = Distances::new(start);
    bread_crumbs.insert(current, maze_dist[current]);

    loop {
        if current == start {
            break;
        }
        if let Some(neighbor_links) = maze.links(&current) {
            for neighbor_pos in neighbor_links {
                if maze_dist[*neighbor_pos] < maze_dist[current] {
                    bread_crumbs.insert(*neighbor_pos, maze_dist[*neighbor_pos]);
                    current = *neighbor_pos;
                    break;
                }
            }
        }
    }

    bread_crumbs
}


/// pretty prints path information (as asterisks) on top of the passed in `grid` and returns
/// it as a String
pub fn display_shortest_path(grid: &Grid, path: &Distances) -> String {
    let mut buf = String::new();
    // write the top wall of the grid
    buf.push_str(&format!("+{}\n", "---+".repeat(grid.cols)));


    for row in grid.row_iter() {
        // top holds the cell 'bodies' (blank spaces) and eastern walls
        let mut top = String::from("|");
        // bottom holds the cell's southern wall and corners ('+') sign
        let mut bottom = String::from("+");

        for cell in row.iter() {

            // if the current cell is part of the path, we want to display a "*" else a " "
            let body = path.get(&cell.pos()).map_or(" ", |_p| "*");

            // the body of the cell will display the distance from the root
            // determine if an eastern wall should be drawn
            match cell.east() {
                Some(east_pos) if grid.has_link(&cell.pos(), &east_pos) => top.push_str(&format!("  {} ", body)),
                _ => top.push_str(&format!(" {} |", body )),
            }

            // determine if a southern wall should be drawn
            match cell.south() {
                Some(south_pos) if grid.has_link(&cell.pos(), &south_pos) => {
                    bottom.push_str("   +")
                }
                _ => bottom.push_str("---+"),
            }
        }

        buf.push_str(&format!("{}\n", top));
        buf.push_str( &format!("{}\n", bottom));
    }
    buf
}