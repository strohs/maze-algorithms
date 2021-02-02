use crate::grid_cell::GridCell;
use crate::position::Pos;
use rand::{thread_rng, Rng};
use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, Iter};


/// Grid represents a two-dimensional grid of `GridCell`s.
/// It also contains a hashmap of "links", or passages, between cells. If two cells are linked,
/// then there is a passage between them (the wall is removed)
#[derive(Debug)]
pub struct Grid {
    // number of rows in this grid
    pub rows: usize,
    // number of columns in this grid
    pub cols: usize,
    // stores the cells of this grid in a 1-dimensional array
    grid: Vec<GridCell>,
    // stores the links (passages) between cells
    links: HashMap<Pos, Vec<Pos>>,
}

impl Grid {
    
    /// returns a grid with capacity for rows * cols  GridCells
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = Grid::build_grid_cells(rows, cols);
        let grid = Self {
            rows,
            cols,
            grid: cells,
            links: HashMap::new(),
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


    /// links (carves a passage between) `from` to `to`. If bidi (bidirectional) is `true` than
    /// an additional link is created between `to` and `from`
    pub fn link(&mut self, from: &Pos, to: &Pos, bidi: bool) {
        self.link_by_pos(from, to);
        if bidi {
            self.link_by_pos(to, from);
        }
    }

    /// creates a uni-directional link between `from` cell and `to` cell. If the
    /// `bidi` (bi-directional) parameter is true, then another link is created from the
    /// `to` cell to the `from` cell.
    fn link_by_pos(&mut self, from: &Pos, to: &Pos) {
        let links = self.links.entry(*from)
            .or_insert(vec![]);
        links.push(*to);
    }

    /// removes the link between `from` and `to`, if there is one. If `bidi` is `true`,
    /// the link between `to` and `from` is removed as well, (assuming there is one). If the links
    /// did not exist, then nothing happens
    pub fn unlink(&mut self, from: &Pos, to: &Pos, bidi: bool) {
        self.unlink_by_pos(from, to);
        if bidi {
            self.unlink_by_pos(to, from);
        }
    }

    /// remove the link between `from` cell and `to` cell. If `bidi` (bi-directional) is true,
    /// then the link between `to` and `from` is removed.
    fn unlink_by_pos(&mut self, from: &Pos, to: &Pos) {
        if let Some(to_links) = self.links.get_mut(from) {
            // search for the to Pos index within from's vec of links
            if let Some((to_idx, _p)) = to_links.iter().enumerate().find(|&(_i, p)| *p == *to) {
                // remove the to_pos from the vec of links
                to_links.remove(to_idx);
                // if there are no more links that from is pointing to, then remove from from the HashMap
                if to_links.is_empty() {
                    self.links.remove(from);
                }
            }
        }
    }

    /// returns `true` if there is a link from `from`, to `to`
    pub fn has_link(&self, from: &Pos, to: &Pos) -> bool {
        self.links
            .get(from)
            .map_or(false, |tos| tos.contains(to))
    }


    /// returns a borrowed Vector of `Pos`, that the given `pos` links to.
    pub fn links(&self, pos: &Pos) -> Option<&Vec<Pos>> {
        self.links.get(pos)
    }

    /// returns a the position of a random cell in the grid
    pub fn random_pos(&self) -> Pos {
        let ridx = thread_rng().gen_range(0, self.grid.len());
        self.idx2d(ridx)
    }

    /// returns an immutable iterator over this Grid's GridCells in row order
    pub fn iter_cells(&self) -> Iter<'_, GridCell> {
        self.grid.iter()
    }

    // /// returns a mutable iterator over all the GridCells in this Grid, in row order
    // pub fn iter_cells_mut(&mut self) -> IterMut<'_, GridCell> {
    //     self.grid.iter_mut()
    // }

    /// returns an immutable iterator over the *rows* of this grid
    pub fn row_iter(&self) -> ChunksExact<'_, GridCell> {
        self.grid.chunks_exact(self.cols)
    }

    /// returns an mutable iterator over the *rows* of this grid
    // pub fn row_iter_mut(&mut self) -> ChunksExactMut<'_, GridCell> {
    //     self.grid.chunks_exact_mut(self.cols)
    // }

    /// returns a one-dimensional index based on the given row, col in `Pos`
    /// panics if any row,col in pos is negative
    fn idx1d(&self, pos: &Pos) -> usize {
        pos.r * self.cols + pos.c
    }

    /// generates a Position from a 1D index
    fn idx2d(&self, index: usize) -> Pos {
        let row = index / self.cols;
        let col = index % self.cols;
        Pos::new(row, col)
    }

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


/// pretty prints the grid to standard out
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
    use std::path::Path;

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

    #[test]
    fn should_bidi_link_one_from_pos_to_one_to_pos() {
        let mut grid = Grid::new(3, 3);
        let from = Pos::new(0, 0);
        let to = Pos::new(0, 1);
        grid.link(&from, &to, true);
        assert!(grid.links.contains_key(&from));
        assert!(grid.links.get(&from).unwrap().contains(&to));
        assert!(grid.links.contains_key(&to));
        assert!(grid.links.get(&to).unwrap().contains(&from));
    }

    #[test]
    fn should_bidi_link_one_from_position_to_two_to_positions() {
        let mut grid = Grid::new(3, 3);
        let from = Pos::new(0, 0);
        let to1 = Pos::new(0, 1);
        let to2 = Pos::new(1, 0);
        grid.link(&from, &to1, true);
        grid.link(&from, &to2, true);
        assert!(grid.links.contains_key(&from));
        assert!(grid.links.get(&from).unwrap().contains(&to1));
        assert!(grid.links.get(&from).unwrap().contains(&to2));
        assert!(grid.links.contains_key(&to1));
        assert!(grid.links.get(&to1).unwrap().contains(&from));
        assert!(grid.links.get(&to2).unwrap().contains(&from));
    }

    #[test]
    fn should_unlink_one_to_one() {
        let mut grid = Grid::new(3, 3);
        let from = Pos::new(0, 0);
        let to = Pos::new(0, 1);
        grid.link(&from, &to, true);
        assert!(grid.links.contains_key(&from));
        assert!(grid.links.get(&from).unwrap().contains(&to));
        assert!(grid.links.contains_key(&to));
        assert!(grid.links.get(&to).unwrap().contains(&from));
        grid.unlink(&from, &to, true);
        assert_eq!(grid.links.contains_key(&from), false);
        assert_eq!(grid.links.contains_key(&to), false);
    }


    #[test]
    fn should_unlink_a_pos_containing_two_pos() {
        let mut grid = Grid::new(3, 3);
        let from = Pos::new(0, 0);
        let to1 = Pos::new(0, 1);
        let to2 = Pos::new(1, 0);
        grid.link(&from, &to1, true);
        grid.link(&from, &to2, true);
        assert!(grid.links.contains_key(&from));
        assert!(grid.links.get(&from).unwrap().contains(&to1));
        assert!(grid.links.get(&from).unwrap().contains(&to2));
        grid.unlink(&from, &to1, true);
        // the grid should still contain a link from `from` to `to2`
        assert!(grid.links.contains_key(&from));
        assert!(grid.links.get(&from).unwrap().contains(&to2));
    }

}
