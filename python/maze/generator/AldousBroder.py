import random
from maze.Grid import Grid


def generate(height: int, width: int) -> Grid:
    """
    Aldous-Broder maze generation. Aldous-Broder generates perfect mazes, (mazes without loops), using "random-walks".
    This avoids creating mazes with biases (like Binary Tree) and instead produces mazes with lots of winding passages.

    The idea behind it is as follows:
    1. Start anywhere in the grid you want, and choose a random neighbor.
    2. Move to that neighbor, and if it has not previously been visited, link it to the prior cell.
    3. Repeat until every cell has been visited.
    :param height:
    :param width:
    :return:
    """

    grid = Grid(height, width)
    # the current cell being visited
    current = grid.random_cell()
    unvisited_count = grid.size() - 1

    while unvisited_count > 0:
        # choose a random neighbor of the current cell
        rand_neighbor = random.choice(current.neighbors())

        # if the random neighbor is not linked to anything, then link current to it
        if not rand_neighbor.linked_cells():
            current.link(rand_neighbor)
            unvisited_count -= 1

        # make rand_neighbor the new current cell
        current = rand_neighbor

    return grid
