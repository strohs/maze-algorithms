use crate::grid_cell::GridCell;
use crate::position::Pos;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter::Skip;
use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut, Iter, IterMut};

/// A one-way link (from --> to) between two positions in the Grid
/// Link.0 is the "from" cell, Link.1 is the "to" cell
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Link(Pos, Pos);

/// Grid represents a two-dimensional grid of `GridCell`s.
/// It also contains a hashmap of "links", or passages, between cells. If two cells are linked,
/// then there is a passage between them (the wall is removed)
#[derive(Debug)]
pub struct Grid {
    // number of rows in this grid
    rows: usize,
    // number of columns in this grid
    cols: usize,
    // stores the cells of this grid in a 1-dimensional array
    grid: Vec<GridCell>,
    // stores the links (passages) between cells
    links: HashSet<Link>,
}

impl Grid {
    /// returns a grid with capacity for rows * cols  GridCells
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = Grid::build_grid_cells(rows, cols);
        let mut grid = Self {
            rows,
            cols,
            grid: cells,
            links: HashSet::new(),
        };

        grid
    }

    /// returns the total number of GridCells in the grid. (size = rows * cols)
    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    /// returns the grid cell at the specified row, col. If row or col are out of bounds
    /// then None is returned.
    /// TODO may not need this function
    // pub fn get(&self, row: isize, col: isize) -> Option<&GridCell> {
    //     if row < 0 || col < 0 {
    //         None
    //     } else {
    //         self.grid.get(self.idx1d(&Pos::from((row as usize, col as usize))))
    //     }
    // }

    // /// creates a uni-directional link between `from` cell and `to` cell. If the
    // /// `bidi` (bi-directional) parameter is true, then another link is created from the
    // /// `to` cell to the `from` cell.
    // pub fn link(&mut self, from: &GridCell, to: &GridCell, bidi: bool) {
    //     self.links.insert(Link(from.pos(), to.pos()));
    //     if bidi {
    //         self.links.insert(Link(to.pos(), from.pos()));
    //     }
    // }

    // pub fn link_by_pos(&mut self, from: Pos, to: Pos, bidi: bool) {
    //     self.links.insert(Link(from, to));
    //     if bidi {
    //         self.links.insert(Link(to, from));
    //     }
    // }

    /// creates a uni-directional link between `from` cell and `to` cell. If the
    /// `bidi` (bi-directional) parameter is true, then another link is created from the
    /// `to` cell to the `from` cell.
    pub fn link(&mut self, from: &Pos, to: &Pos, bidi: bool) {
        self.links.insert(Link(*from, *to));
        if bidi {
            self.links.insert(Link(*to, *from));
        }
    }

    /// remove the link between `from` cell and `to` cell. If `bidi` (bi-directional) is true,
    /// then the link between `to` and `from` is removed.
    pub fn unlink(&mut self, from: &Pos, to: &Pos, bidi: bool) {
        self.links.remove(&Link(*from, *to));
        if bidi {
            self.links.remove(&Link(*to, *from));
        }
    }

    /// returns `true` if there is a link between cells at the given positions
    pub fn has_link(&self, from: &Pos, to: &Pos) -> bool {
        self.links.contains(&Link(*from, *to))
    }

    /// returns a reference to a random GridCell in this grid
    pub fn random_cell(&self) -> &GridCell {
        let ridx = thread_rng().gen_range(0, self.grid.len());
        &self.grid[ridx]
    }

    /// returns an immutable iterator over this Grid's GridCells in row order
    pub fn iter_cells(&self) -> Iter<'_, GridCell> {
        self.grid.iter()
    }

    /// returns a mutable iterator over all the GridCells in this Grid, in row order
    pub fn iter_cells_mut(&mut self) -> IterMut<'_, GridCell> {
        self.grid.iter_mut()
    }

    /// returns an immutable iterator over the *rows* of this grid
    pub fn row_iter(&self) -> ChunksExact<'_, GridCell> {
        self.grid.chunks_exact(self.cols)
    }

    /// returns an mutable iterator over the *rows* of this grid
    pub fn row_iter_mut(&mut self) -> ChunksExactMut<'_, GridCell> {
        self.grid.chunks_exact_mut(self.cols)
    }

    /// returns a one-dimensional index based on the given row, col in `Pos`
    /// panics if any row,col in pos is negative
    fn idx1d(&self, pos: &Pos) -> usize {
        pos.r * self.cols + pos.c
    }

    /// converts a one-dimensional index into a two dimensional row,column index
    // fn idx_2d(&self, idx: usize) -> Pos {
    //     Pos::new(idx / self.cols, idx % self.cols)
    // }

    /// returns Some(Pos) if the given position has a neighbor to the north, else None
    /// the returned position is the position of the North neighbor
    fn has_north(pos: &Pos) -> Option<Pos> {
        match *pos {
            Pos { r: row, c: col } if row > 0 => Some(Pos::new(row - 1, col)),
            _ => None,
        }
    }

    /// returns Some(Pos) if the given row,col position has a neighbor to the south, else None
    /// the returned position is the position of the South neighbor
    fn has_south(pos: &Pos, grid_rows: usize) -> Option<Pos> {
        match *pos {
            Pos { r: row, c: col } if row < grid_rows - 1 => Some(Pos::new(row + 1, col)),
            _ => None,
        }
    }

    /// returns Some(Pos) if the given row,col position has a neighbor to the east, else None
    /// the returned position is the position of the east neighbor
    fn has_east(pos: &Pos, grid_cols: usize) -> Option<Pos> {
        match *pos {
            Pos { r: row, c: col } if col < grid_cols - 1 => Some(Pos::new(row, col + 1)),
            _ => None,
        }
    }

    /// returns Some(Pos) if the given row,col position has a neighbor to the west, else None
    /// the returned position is the position of the west neighbor
    fn has_west(pos: &Pos) -> Option<Pos> {
        match *pos {
            Pos { r: row, c: col } if col > 0 => Some(Pos::new(row, col - 1)),
            _ => None,
        }
    }

    /// Helper that builds a vector of grid cells for the dimensions of this grid. It ensures
    /// that each GridCell struct has links to neighboring cells, set.
    fn build_grid_cells(rows: usize, cols: usize) -> Vec<GridCell> {
        let mut grid = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                let pos = Pos::new(r, c);
                let gc = GridCell::new(
                    Pos::new(r, c),
                    Grid::has_north(&pos),
                    Grid::has_south(&pos, rows),
                    Grid::has_east(&pos, cols),
                    Grid::has_west(&pos),
                );
                grid.push(gc);
            }
        }
        grid
    }
}

/// allows indexing into this grid using a `Pos` struct
impl Index<Pos> for Grid {
    type Output = GridCell;

    fn index(&self, pos: Pos) -> &Self::Output {
        let idx = self.idx1d(&pos);
        &self.grid[idx]
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let idx = self.idx1d(&pos);
        &mut self.grid[idx]
    }
}

// /// allows indexing into this grid using [] syntax. I.e.  grid[[row, col]]
// /// If the specified indices are out of bounds, `None` is returned
// impl Index<[usize; 2]> for Grid {
//     type Output = GridCell;
//
//     fn index(&self, rc_index: [usize; 2]) -> &Self::Output {
//         let idx = self.idx1d(&Pos::new(rc_index[0], rc_index[1]));
//         &self.grid[idx]
//     }
// }

/// Prints the grid to STDOUT as ascii characters
/// the grid is iterated cell by cell in row order. At each cell, we only check if an
/// eastern wall and/or southern wall should be drawn.
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write the top wall of the grid
        writeln!(f, "+{}", "---+".repeat(self.cols))?;

        for row in self.row_iter() {
            // top holds the cell 'bodies' (blank spaces) and eastern walls
            let mut top = String::from("|");
            // bottom holds the cell's southern wall and corners ('+') sign
            let mut bottom = String::from("+");

            for cell in row.iter() {
                // determine if an eastern wall should be drawn
                match cell.east() {
                    Some(east_pos) if self.has_link(&cell.pos(), &east_pos) => top.push_str("    "),
                    _ => top.push_str("   |"),
                }

                // determine if a southern wall should be drawn
                match cell.south() {
                    Some(south_pos) if self.has_link(&cell.pos(), &south_pos) => {
                        bottom.push_str("   +")
                    }
                    _ => bottom.push_str("---+"),
                }
            }

            writeln!(f, "{}", top)?;
            writeln!(f, "{}", bottom)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, Pos};

    #[test]
    fn should_create_new_grid() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cols, 3);
        assert_eq!(grid.grid.len(), 9);
    }

    #[test]
    fn pos_0_0_should_not_have_north_neighbor() {
        let grid = Grid::new(3, 3);
        let pos = Pos::new(0, 0);
        assert_eq!(grid.grid[0].north(), None);
    }

    #[test]
    fn pos_2_1_should_not_have_south_neighbor() {
        let grid = Grid::new(3, 3);
        let pos = Pos::new(2, 1);
        assert_eq!(grid[pos].south(), None);
    }

    #[test]
    fn pos_0_0_should_not_have_west_neighbor() {
        let grid = Grid::new(3, 3);
        let pos = Pos::new(0, 0);
        assert_eq!(grid[pos].west(), None);
    }

    #[test]
    fn pos_0_2_should_not_have_east_neighbor() {
        let grid = Grid::new(3, 3);
        let pos = Pos::new(0, 2);
        assert_eq!(grid[pos].east(), None);
    }

    #[test]
    fn pos_1_1_should_have_all_neigbors() {
        let grid = Grid::new(3, 3);
        let pos = Pos::new(1, 1);
        assert_eq!(grid[pos].north(), Some(Pos::new(0, 1)));
        assert_eq!(grid[pos].south(), Some(Pos::new(2, 1)));
        assert_eq!(grid[pos].east(), Some(Pos::new(1, 2)));
        assert_eq!(grid[pos].west(), Some(Pos::new(1, 0)));
    }

    #[test]
    fn can_index_into_grid_with_brackets() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid[[1, 1]].pos(), Pos::new(1, 1));
    }

    #[test]
    fn grid_size_is_15() {
        let grid = Grid::new(3, 5);
        assert_eq!(grid.size(), 15);
    }

    #[test]
    fn should_iterate_over_rows() {
        let grid = Grid::new(3, 3);
        let ri = grid.row_iter();
        // should return three total rows
        assert_eq!(ri.len(), 3);
    }
}
