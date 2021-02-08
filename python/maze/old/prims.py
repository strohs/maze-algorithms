# Generates a maze using Prim's algorithm.
# Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
# entire graph, it starts at one point, and grows outward from that point. The standard version
# of the algorithm works something like this:

# 1. Choose an arbitrary vertex from G (the graph), and add it to some (initially empty) set V.
# 2. Choose a **random** edge from G, that connects a vertex in V with another vertex not in V.
# 3. Add that edge to the minimal spanning tree, and the edge’s other vertex to V.
# 4. Repeat steps 2 and 3 until V includes every vertex in G.

from maze import (Maze, Direction)
import random
import sys

# bitmask used to mark cells that are "in" the maze
IN = 0x10           # 0001 0000
# bitmask used to mark cells that are in the frontier
FRONTIER = 0x20     # 0010 0000


def add_frontier(maze, frontier, x, y) -> bool:
    """adds the specified cell at position x,y to the maze's list of frontier cells."""
    # only add cells to the frontier that are with-in the bounds of the maze AND are empty (i.e. == 0)
    if maze.in_bounds(x, y) and 0 == maze.field[y][x]:
        maze.field[y][x] |= FRONTIER
        frontier.append((x, y))
        return True
    else:
        return False


def mark_in(maze, frontier, x, y):
    """
    marks the cell at the given x,y position as being in the maze. Also adds all neighboring cells to the
    frontier (if those cells are not already in the frontier)
    """
    maze.field[y][x] |= IN
    if x > 0:
        add_frontier(maze, frontier, x - 1, y)
    add_frontier(maze, frontier, x + 1, y)
    if y > 0:
        add_frontier(maze, frontier, x, y - 1)
    add_frontier(maze, frontier, x, y + 1)


def prims(width: int, height: int) -> Maze:
    """
    generates a maze using prim's algorithm
    :param width: width (number of columns) in the generated maze
    :param height: height (number of rows) in the generated maze
    :return: a Maze object
    """
    maze = Maze(width, height)

    # a list of 2-tuples, (x,y) indices, of cells in the maze that are currently in the frontier
    frontier = []

    xs = random.randrange(0, width)
    ys = random.randrange(0, height)
    mark_in(maze, frontier, xs, ys)

    # continue building the maze until there are no more cells in the frontier
    while len(frontier) != 0:
        # choose a random cell in the frontier and remove it
        fx, fy = frontier.pop(random.randrange(0, len(frontier)))

        # chose a random neighbor of the chosen frontier cell, that is "IN" the maze
        ns = maze.neighbors(fx, fy)
        in_ns = [(x, y) for x, y in ns if maze.field[y][x] & IN != 0]
        nx, ny = random.choice(in_ns)

        # remove the cell wall from the chosen frontier cell to the chosen neighbor cell
        dir = maze.remove_wall(fx, fy, nx, ny)
        maze.field[fy][fx] |= dir.value
        maze.field[ny][nx] |= Direction.opposite(dir).value

        mark_in(maze, frontier, fx, fy)
    return maze


width = int(sys.argv[1]) if len(sys.argv) > 1 else 20
height = int(sys.argv[2]) if len(sys.argv) > 2 else 20
maze = prims(width, height)
maze.display()
