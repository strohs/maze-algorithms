import random

from maze.Grid import Grid


def generate(height: int, width: int) -> Grid:
    """
    generates a random, perfect, maze using recursive backtracker algorithm

    Here’s how the recursive backtracker algorithm works:
    1. Choose a random starting point in the grid.
    2. Randomly choose a random neighbor that has not been visited and link to it. This neighbor
    becomes the new current cell.
    3. If all neighbor cells have been visited, back up to the last cell that has unvisited
    neighbors and repeat.
    4. The algorithm ends when the process has backed all the way up to the starting point.
    In essence, this carves passages using a depth-first search, that backtracks once it carves itself
    into a corner. Also, like hunt-and-kill, recursive-backtracker also produces mazes that are full
    of long and meandering passages.
    :param height: the number of rows in the maze
    :param width: the number of columns in the maze
    :return: a Grid containing the generated maze
    """

    grid = Grid(height, width)
    # stack holds the cells to be visited
    stack = [grid.random_cell()]

    while stack:
        # get the last element of the stack
        current = stack[-1]

        # get all linked neighbors of current that are NOT linked to any other cells
        neighbors = list(filter(lambda nbr: not nbr.linked_cells(), current.neighbors()))

        if not neighbors:
            # we are in a corner, so backtrack by popping the current cell off of the stack
            stack.pop()
        else:
            # choose a random neighbor, link to it, and make it the new current by pushing it on the stack
            neighbor = random.choice(neighbors)
            current.link(neighbor)
            stack.append(neighbor)

    return grid
