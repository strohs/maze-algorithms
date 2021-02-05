import random
from Cell import Cell


class Grid:
    """
    Grid is a wrapper object around a two-dimensional list of Cells.
    Grid's are passed to maze generation algorithms, which will then carve passages between cells, using the cell's
     link() method.
    """

    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        # build 2D grid with default cell values
        self.grid = [[Cell(r, c) for c in range(cols)] for r in range(rows)]
        self._build_cell_neighbors()

    def _build_cell_neighbors(self):
        """sets the neighbor pointers for each cell in the grid"""
        for r in range(self.rows):
            for c in range(self.cols):
                # cell will have a north neighbor
                if r > 0:
                    self.grid[r][c].north = self.grid[r - 1][c]
                # cell will have a south neighbor
                if r < self.rows - 1:
                    self.grid[r][c].south = self.grid[r + 1][c]
                # cell will have east neighbor
                if c < self.cols - 1:
                    self.grid[r][c].east = self.grid[r][c + 1]
                # cell will have west neighbor
                if c > 0:
                    self.grid[r][c].east = self.grid[r][c - 1]

    def row_iterator(self):
        """iterates over the rows of this grid"""
        for r in range(self.rows):
            yield self.grid[r]

    def cell_iterator(self):
        """iterates over the cells of this grid in row major order"""
        for r in range(self.rows):
            for c in range(self.cols):
                yield self.grid[r][c]

    def __str__(self) -> str:
        """pretty prints the cells of the grid into a String"""

        # build the top wall
        out = "+" + ("---+" * self.cols) + "\n"

        for row in self.row_iterator():
            top = "|"
            bottom = "+"

            # for each cell only need to check if an east or a south wall should be drawn
            for cell in row:
                # determine if eastern wall should be drawn
                if cell.east and cell.is_linked(cell.east):
                    # don't draw a east wall
                    top += "    "
                else:
                    # draw the east wall
                    top += "   |"

                # determine if south wall should be drawn
                if cell.south and cell.is_linked(cell.south):
                    # don't draw a south wall
                    bottom += "   +"
                else:
                    # draw the south wall
                    bottom += "---+"

            out += top + "\n"
            out += bottom + "\n"

        return out

