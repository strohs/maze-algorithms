use crate::grid::Grid;
use crate::grid_cell::GridCell;
use crate::position::Pos;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// Generates a random maze using the Sidewinder algorithm. It's similar to binary tree but
/// does have some differences. In a nutshell, it goes like this:
///  1. Work through the grid row-wise, starting with the cell at 0,0. Initialize the “run” set to be empty.
///  2. Add the current cell to the “run” set.
///  3. For the current cell, randomly decide whether to carve east or not.
///  4. If a passage was carved, make the new cell the current cell and repeat steps 2-4.
///  5. If a passage was not carved, choose any one of the cells in the run set and carve a
///     passage north. Then empty the run set, set the next cell in the row to be the current
///     cell, and repeat steps 2-5.
///  6. Continue until all rows have been processed.
pub fn generate(height: usize, width: usize) -> Grid {
    let at_eastern_boundary = |cell: &GridCell| cell.east().is_none();
    let at_northern_boundary = |cell: &GridCell| cell.north().is_none();
    // should we close out the current run of cells
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
