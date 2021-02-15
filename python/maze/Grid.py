import random
from maze.Cell import Cell


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

    def size(self):
        """return the dimension of this grid, which is  grid.rows * grid.cols"""
        return self.rows * self.cols

    def get(self, row: int, col: int) -> Cell:
        return self.grid[row][col]

    def random_cell(self):
        """returns a random cell in this grid"""
        return random.choice(random.choice(self.grid))

    def dead_ends(self) -> list:
        """
        returns a list of cells in this grid that are dead end cells. Dead-end cells only have one link
        into/out of them
        """
        return [cell for cell in self.cell_iterator() if len(cell.linked_cells()) == 1]

    def braid(self, p: float):
        """
        Adds braids to this maze by removing dead-end cells and turning them into loops
        :param p: a float value where 0.0 <= p <= 1.0, that is the percentage amount of dead-ends to remove.
        1.0 = remove all dead-ends, while a value of 0.5 would remove 50 percent of dead-ends
        :return: None, the grid is mutated in-place
        """
        # get all dead-ends in the grid and shuffle them
        dead_ends = self.dead_ends()
        random.shuffle(dead_ends)

        for cell in dead_ends:
            if len(cell.linked_cells()) == 1 and random.random() <= p:
                # get neighbors that are not linked to cell
                neighbors = list(filter(lambda c: not cell.is_linked(c), cell.neighbors()))

                # try to find neighbors that are also dead-ends, if possible
                best = list(filter(lambda c: len(c.linked_cells()) == 1, neighbors))

                # if no best cells could be found, then just use neighbors
                if not best:
                    best = neighbors

                # choose a random neighbor and link to it
                cell.link(random.choice(best))

    def _build_cell_neighbors(self):
        """sets the neighbor cells for each cell in the grid"""
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
                    self.grid[r][c].west = self.grid[r][c - 1]

    def row_iterator(self):
        """iterates over the rows of this grid"""
        for r in range(self.rows):
            yield self.grid[r]

    def cell_iterator(self):
        """iterates over the cells of this grid in row major order"""
        for r in range(self.rows):
            for c in range(self.cols):
                yield self.grid[r][c]

    def print_grid(self, distances=None) -> str:
        """
        pretty prints the cells and walls of the grid into a String. if distances is passed,then the cells
        distance information will be printed inside the cell body as a hexadecimal number
        """

        def cell_body(cell, distances):
            if distances and cell in distances.cells:
                return "{:02x}".format(distances.cells[cell])
            else:
                return "  "

        # build the top wall
        out = "+" + ("---+" * self.cols) + "\n"

        for row in self.row_iterator():
            top = "|"
            bottom = "+"

            # for each cell only need to check if an east or a south wall should be drawn
            for cell in row:
                # body contains what should be printed within a cell of the grid
                body = cell_body(cell, distances)

                # determine if eastern wall should be drawn
                if cell.east and cell.is_linked(cell.east):
                    # don't draw a east wall
                    top += f"{body}  "
                else:
                    # draw the east wall
                    top += f"{body} |"

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

    def __str__(self) -> str:
        return self.print_grid()


