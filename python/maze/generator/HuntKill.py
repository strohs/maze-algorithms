import random
from maze.Grid import Grid


def generate(height: int, width: int) -> Grid:
    """
    Generates a random, perfect, maze using the Hunt and Kill maze generation algorithm.

    Hunt-and-kill is similar to Aldous-Broder but slightly different. Aldous-Broder allows you to step
    into any cell (even previously visited ones), while hunt-and-kill requires you to walk on only unvisited
    cells. If you walk into a corner, you begin the "hunt" mode, which is where you start from the top of the
    maze, look for the first cell that is a neighbor of the cells in the current walk you are performing, and
    then link into the walk. Then repeat the random walk until all cells are visited.

    Hunt-and-Kill is known to produce mazes with longer winding and meandering corridors than
    other algorithms. That is to say, hunt-and-kill produces mazes with fewer dead ends.
    :param height: the number of rows to generate
    :param width: the number of columns to generate
    :return: a Grid object that contains the maze
    """

    grid = Grid(height, width)
    current = grid.random_cell()

    while current:
        # get all linked neighbors of the current cell that are not linked to other cells
        unvisited_neighbors = list(filter(lambda cell: not cell.linked_cells(), current.neighbors()))

        # start randomly walking cells. If the current has unvisited neighbors, we will link current
        # to a random, unvisited neighbor, and then make that random neighbor the current
        if unvisited_neighbors:
            rand_neighbor = random.choice(unvisited_neighbors)
            current.link(rand_neighbor)
            current = rand_neighbor
        else:
            current = None

            # start the hunt phase. Starting from the top row of the grid, begin looking
            # for the first cell that is unvisited AND that has visited neighbors
            for cell in grid.cell_iterator():
                visited_neighbors = list(filter(lambda cell: cell.linked_cells(), cell.neighbors()))

                # if a cell is unvisited but one of its neighbors is, then link cell to a random
                # neighbor and set current cell to the cell being visited
                if not cell.linked_cells() and visited_neighbors:
                    current = cell
                    current.link(random.choice(visited_neighbors))
                    break
    return grid
