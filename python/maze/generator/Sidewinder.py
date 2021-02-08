from maze.Grid import Grid
from maze.Cell import Cell
import random


def generate(height: int, width: int) -> Grid:
    """
    generates a random, perfect, maze using the sidewinder algorithm

    Sidewinder is similar to binary tree but does have some differences.
    In a nutshell, it goes like this:
    1. Work through the grid row-wise, starting with the cell at 0,0. Initialize the “run” set to be empty.
    2. Add the current cell to the “run” set.
    3. For the current cell, randomly decide whether to carve east or not.
    4. If a passage was carved, make the new cell the current cell and repeat steps 2-4.
    5. If a passage was not carved, choose any one of the cells in the run set and carve a
        passage north. Then empty the run set, set the next cell in the row to be the current
        cell, and repeat steps 2-5.
    6. Continue until all rows have been processed.
    :param height: the number of rows to generate
    :param width: the number of columns to generate
    :return: a grid containing the maze
    """
    def should_close_out(cell):
        """
        returns true if a current run of Cells should be closed out. A run should be closed out if
        we are at the eastern most cell of a row, OR if we are not on the northernmost row and a random
        true value is generated
        :param cell:
        :return:
        """
        return not cell.east or (cell.north and random.choice([True, False]))

    grid = Grid(height, width)

    for cell in grid.cell_iterator():
        runs = [cell]

        # if a run should be closed out, then choose a random cell from the run, and link to that cells
        # north neighbor
        if should_close_out(cell) and runs:
            rand_cell = random.choice(runs)

            if rand_cell.north:
                rand_cell.link(rand_cell.north)

            runs.clear()
        elif cell.east:
            cell.link(cell.east)

    return grid
