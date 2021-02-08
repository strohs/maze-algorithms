import random
from maze.Grid import Grid


def generate(height: int, width: int) -> Grid:
    """
    Generates a random, perfect, maze using Wilson's algorithm

    Like Aldous-Broder, this algorithm depends on the idea of a random walk, but with a twist.
    It performs what is called a loop-erased random walk, which means that as it goes, if the path
    it is forming happens to intersect with itself and form a loop, it erases that loop before
    continuing on.
    1. choose a point on the grid and mark it visited.
    2. choose any unvisited cell in the grid and perform a loop-erased random walk until you
        reach a visited cell.
    3. link all the cells in the current random walk to the visited cell
    4. repeat step 2 until all cells in the grid have been visited
    :param height: number of rows to generate
    :param width: number of columns to generate
    :return: a Grid containing the generated maze
    """
    grid = Grid(height, width)

    # initialize the unvisited list to contain all cells in the grid
    unvisited = [cell for cell in grid.cell_iterator()]
    # choose a random cell in unvisited to become "visited"
    unvisited.remove(random.choice(unvisited))

    while unvisited:
        # choose a random, unvisited cell to begin at
        cell = random.choice(unvisited)
        # add the cell to the random walk "path"
        path = [cell]

        # start randomly visiting unvisited cells
        while cell in unvisited:
            cell = random.choice(cell.neighbors())

            if cell in path:
                # if we've already visited a cell, (creating a loop), remove the last cell from the path
                path = path[0: path.index(cell) + 1]
            else:
                # add cell to the path as it has not been added to the path
                path.append(cell)

        # link together all cell in the path, then remove each cell in the path from the unvisited list
        for i in range(len(path) - 1):
            path[i].link(path[i + 1])
            unvisited.remove(path[i])

    return grid
