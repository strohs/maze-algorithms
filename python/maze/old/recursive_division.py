# The **recursive division** algorithm is a "wall adder". This algorithm is particularly
# fascinating because of its fractal nature: you could theoretically continue the process
# indefinitely at progressively finer and finer levels of detail.
#
# It works like this:
#
# 1. Begin with an empty field.
# 2. Bisect the field with a wall, either horizontally or vertically. Add a single passage through the wall.
# 3. Repeat step #2 with the areas on either side of the wall.
# 4. Continue, recursively, until the maze reaches the desired resolution.

from enum import Enum
from maze import Maze
import random
import sys


class Direction(Enum):
    """A South, East direction that indicates which walls of a cell have been carved out"""
    SOUTH = 1
    EAST = 2


class Orientation(Enum):
    """represents a wall orientation of a cell"""
    HORIZONTAL = 1
    VERTICAL = 2


def rand(max):
    return 0 if max == 0 else random.randrange(0, max)


def choose_orientation(width: int, height: int) -> Orientation:
    """returns a wall orientation based on the passed in width/height"""
    if width < height:
        return Orientation.HORIZONTAL
    elif height < width:
        return Orientation.VERTICAL
    else:
        return random.choice([Orientation.VERTICAL, Orientation.HORIZONTAL])


def display_maze(maze: Maze):
    """pretty prints the cells of the passed in maze to standard out"""
    print(" _" * maze.width)
    for y in range(maze.height):
        row = "|"
        for x in range(maze.width):
            # true if printing the bottom row
            bottom = y + 1 >= len(maze.field)
            # true if a southern facing wall should be printed
            south = ((maze.field[y][x] & Direction.SOUTH.value) != 0) or bottom
            south2 = (x + 1) < len(maze.field[y]) and ((maze.field[y][x+1] & Direction.SOUTH.value != 0) or bottom)
            east = ((maze.field[y][x] & Direction.EAST.value) != 0) or (x + 1) >= len(maze.field[y])

            row += "_" if south else " "
            if east:
                row += "|"
            elif south and south2:
                row += "_"
            else:
                row += " "
        print(row)


def recursive_divide(maze: Maze, x: int, y: int, width: int, height: int, orientation: Orientation):
    """
    generates a random maze using the recursive division algorithm
    :param maze: an empty maze that will be mutated by the algorithm
    :param x: starting x (column) position in the maze
    :param y: starting y (row) position in the maze
    :param width: the current width of the room being built
    :param height: the current height of the room being built
    :param orientation: orientation of the room
    :return:
    """
    if width < 2 or height < 2:
        return

    horizontal = orientation == Orientation.HORIZONTAL

    # determine where a wall will be drawn first
    wx = x if horizontal else x + rand(width - 2)
    wy = y + rand(height - 2) if horizontal else y

    # determine where to place a passage thru the wall
    px = wx + rand(width) if horizontal else wx
    py = wy if horizontal else wy + rand(height)

    # determine direction to draw the wall
    dx = 1 if horizontal else 0
    dy = 0 if horizontal else 1

    # compute wall length
    length = width if horizontal else height

    # what direction is perpendicular to the wall
    perp_dir = Direction.SOUTH if horizontal else Direction.EAST

    for i in range(length):
        if wx != px or wy != py:
            maze.field[wy][wx] |= perp_dir.value
        wx += dx
        wy += dy

    nx, ny = x, y
    w = width if horizontal else wx - x + 1
    h = wy - y + 1 if horizontal else height
    recursive_divide(maze, nx, ny, w, h, choose_orientation(w, h))

    nx = x if horizontal else wx + 1
    ny = wy + 1 if horizontal else y
    w = width if horizontal else x + width - wx - 1
    h = y + height - wy - 1 if horizontal else height
    recursive_divide(maze, nx, ny, w, h, choose_orientation(w, h))


width = int(sys.argv[1]) if len(sys.argv) > 1 else 20
height = int(sys.argv[2]) if len(sys.argv) > 2 else 20
maze = Maze(width, height)
recursive_divide(maze, 0, 0, maze.width, maze.height, choose_orientation(maze.width, maze.height))
display_maze(maze)
