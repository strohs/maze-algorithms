import random
from maze.Grid import Grid

# Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
# entire graph, it starts at one point, and grows outward from that point.
# The standard version of the algorithm works something like this:
#   1. Choose an arbitrary cell from G (the grid), and add it to some (initially empty) set V.
#   2. select a cell (current) from V with the lowest weight
#   3. get all unlinked neighbors of current and select the neighbor with the lowest weight (neighbor)
#   4. if a neighbor was found:
#   5.   link current to it
#   6.   add neighbor to V
#   7. else (backed into a corner of the maze)
#   8.   remove current from V
#   9. repeat steps 2 thru 9 until there are no longer cells in V


def generate(height: int, width: int) -> Grid:
    """
    Generates a random, perfect maze using Prim's algorithm.
    Cells in the grid will be assigned random weights in order to help generate a more random maze

    :param height: number of rows to generate
    :param width: number of columns to generate
    :return: a Grid object containing the maze
    """

    def unlinked_neighbors(cell):
        """returns a list of cell's neighbors, that are not linked to some other cell"""
        return list(filter(lambda nbr: not nbr.linked_cells(), cell.neighbors()))

    def sort_by_weight(cells):
        """sorts a list of cells, in place, by their weight"""
        cells.sort(key=lambda c: c.weight)

    grid = Grid(height, width)
    # assign random weights to all cells in the grid
    for cell in grid.cell_iterator():
        cell.weight = random.randint(1, 100)

    to_visit = [grid.random_cell()]

    while to_visit:
        # sort cells by their weights, ideally we would use a priority queue
        sort_by_weight(to_visit)
        cell = to_visit[0]
        neighbors = unlinked_neighbors(cell)

        if neighbors:
            # sort neighbors by weight
            sort_by_weight(neighbors)
            neighbor = neighbors[0]
            cell.link(neighbor)
            to_visit.append(neighbor)
        else:
            to_visit.remove(cell)

    return grid
