use crate::grid::Grid;
use crate::solver::distances::Distances;
use crate::position::Pos;

/// find the distances from the `root` (cell Pos) to all other cells in `grid`
pub fn distances(grid: &Grid, root: Pos) -> Distances {
    let mut distances = Distances::new(root);
    let mut frontier = vec![root];

    // while !frontier.is_empty() {
    //     let mut frontier = vec![];
    //
    //     for pos in frontier {
    //
    //     }
    // }

    distances
}