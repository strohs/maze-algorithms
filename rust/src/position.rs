/// holds a (row, column) position in a rectangular Grid
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub r: usize,
    pub c: usize,
}

impl Pos {
    /// create a new `Pos`ition with the specified `row` and `column` values
    pub fn new(row: usize, col: usize) -> Self {
        Self { r: row, c: col }
    }

    pub fn iter(max_row: usize, max_col: usize) -> PositionIter {
        PositionIter::new(max_row, max_col)
    }
}

/// convert from a `(usize, usize)` tuple into a position
impl From<(usize, usize)> for Pos {
    fn from(tup: (usize, usize)) -> Self {
        Pos::new(tup.0, tup.1)
    }
}

pub struct PositionIter {
    max_row: usize,
    max_col: usize,
    cur_idx: usize,
    max_idx: usize,
}

impl PositionIter {
    /// create a new Position iterator
    pub fn new(max_row: usize, max_col: usize) -> Self {
        Self {
            cur_idx: 0,
            max_row,
            max_col,
            max_idx: max_row * max_col,
        }
    }
}

/// an iterator which generates positions from `Pos(0,0)` to `Pos(max_row-1, max_col-1)`
impl Iterator for PositionIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx == 0 {
            self.cur_idx += 1;
            Some(Pos::new(0, 0))
        } else if self.cur_idx < self.max_idx {
            let r = self.cur_idx / self.max_col;
            let c = self.cur_idx % self.max_col;
            self.cur_idx += 1;
            Some(Pos::new(r, c))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Pos;

    #[test]
    fn should_iterate_to_last() {
        assert_eq!(Pos::iter(3, 3).last().unwrap(), Pos::new(2, 2));
    }
}
