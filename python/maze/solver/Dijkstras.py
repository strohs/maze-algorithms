from maze.Grid import Grid
from maze.Cell import Cell
from maze.Distances import Distances


def path_to_goal(start: Cell, goal: Cell) -> Distances:
    """
    An implementation of Dijkstra's algorithm to find the shortest path between two cells in a maze

    :param start: a cell in maze to start at
    :param goal: a cell in maze to finish at
    :return: a Distances object that will contain cells on the shortest path
    """
    # compute distances from start cell
    maze_distances = start.distances()

    # start at the goal cell and work backwards towards the start cell
    current = goal
    curr_path = Distances(current)

    # add the goal cell to the current path
    curr_path.cells[current] = maze_distances.cells[current]

    while current is not start:
        for neighbor_cell in current.linked_cells():
            # if the neighbor's distance is less than the current cell's distance, insert
            # the neighbor cell into curr_path
            if maze_distances.cells[neighbor_cell] < maze_distances.cells[current]:
                curr_path.cells[neighbor_cell] = maze_distances.cells[neighbor_cell]
                current = neighbor_cell
                break

    return curr_path
