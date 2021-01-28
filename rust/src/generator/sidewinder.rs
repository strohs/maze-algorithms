use crate::grid::Grid;
use crate::grid_cell::GridCell;
use crate::position::Pos;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// Generates a random maze using the Sidewinder algorithm
///
pub fn generate(height: usize, width: usize) -> Grid {
    let at_eastern_boundary = |cell: &GridCell| cell.east().is_none();
    let at_northern_boundary = |cell: &GridCell| cell.north().is_none();
    let should_close_out = |cell: &GridCell| {
        at_eastern_boundary(cell) || (!at_northern_boundary(cell) && thread_rng().gen::<bool>())
    };

    let mut grid = Grid::new(height, width);

    for pos in Pos::iter(height, width) {
        let cur_cell = grid[pos];
        let mut runs = vec![cur_cell];

        if should_close_out(&cur_cell) {
            let rand_member = runs.choose(&mut thread_rng());

            // if the random_member has a north neighbor, carve a passage from the random cell
            // to it's north neighbor
            if let Some(rand_cell) = rand_member {
                rand_cell
                    .north()
                    .map(|north_pos| grid.link(&rand_cell.pos(), &north_pos, true));
            }
            runs.clear();
        } else {
            // carve a passage from current cell to the east neighbor
            if cur_cell.east().is_some() {
                grid.link(&cur_cell.pos(), &cur_cell.east().unwrap(), true);
            }
        }
    }

    grid
}
