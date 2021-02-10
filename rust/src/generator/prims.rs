use crate::grid::Grid;
use rand::{thread_rng, Rng};
use crate::position::Pos;
use crate::grid_cell::GridCell;

pub fn generate(height: usize, width: usize) -> Grid {
    let mut grid = Grid::new(height, width);
    // assign random weights to all cells in the grid
    for cell in grid.iter_mut_cells() {
        cell.set_weight(thread_rng().gen_range(1, 101));
    }

    let mut to_visit = vec![grid.random_pos()];

    while !to_visit.is_empty() {
        // sort to_visit by cell weight
        to_visit.sort_by_key(|p| grid[*p].weight());
        let cell_pos = to_visit[0];

        let mut neighbors = unlinked_neighbors(&grid, cell_pos);

        if !neighbors.is_empty() {
            neighbors.sort_by_key(|p| grid[*p].weight());
            // link cell_pos to the lowest weighted neighbor
            grid.link(&cell_pos, &neighbors[0], true);
            to_visit.push(neighbors[0]);
        } else {
            let ridx = to_visit.iter().position(|p| *p == cell_pos).unwrap();
            to_visit.remove(ridx);
        }
    }

    grid
}

/// returns the positions of any unlinked neighbors of `pos`
fn unlinked_neighbors(grid: &Grid, pos: Pos) -> Vec<Pos> {
    grid[pos].neighbors()
        .iter()
        .filter(|&p| grid.links(p).is_empty())
        .map(|p| *p)
        .collect::<Vec<Pos>>()
}