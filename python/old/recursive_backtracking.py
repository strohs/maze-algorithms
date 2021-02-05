from maze import (Maze, Direction)
import random
import sys

# Recursive Backtracking
# Here’s the mile-high view of **recursive backtracking**:
#
# 1. Choose a starting point in the field.
# 2. Randomly choose a wall at that point and carve a passage through to the adjacent cell, but
#    only if the adjacent cell has not been visited yet. This becomes the new current cell.
# 3. If all adjacent cells have been visited, back up to the last cell that has uncarved walls and repeat.
# 4. The algorithm ends when the process has backed all the way up to the starting point.
#    cx,cy is the current cell being visited


def recurse(cx, cy, maze: Maze):
    """
    generates a random maze on the passed in maze, using the recursive backtracing algorithm. The passed in maze in
    modified in place
    :param cx: cell x position to begin the algorithm at, 0 is the leftmost column of maze
    :param cy: cell y position to begin the algorithm at, 0 is the topmost row of the maze
    :param maze: the empty maze to use for the algorithm
    """
    directions = [Direction.N, Direction.S, Direction.E, Direction.W]
    random.shuffle(directions)

    for dir in directions:
        nx = cx + Direction.dx(dir)
        ny = cy + Direction.dy(dir)

        if 0 <= ny < len(maze.field) and nx >= 0 and nx < len(maze.field[ny]) and maze.field[ny][nx] == 0:
            maze.field[cy][cx] |= dir.value
            maze.field[ny][nx] |= Direction.opposite(dir).value
            recurse(nx, ny, maze)


def recursive_backtracking(start_x: int, start_y: int, maze_width: int, maze_height: int) -> Maze:
    """
    generates a random maze using recursive backtracking
    :param start_x: the x  position in the maze to start at
    :param start_y: the y position in the maze to start at
    :param maze_width: the width of the maze to generate
    :param maze_height: the height of the maze to generate
    :return: a random Maze
    """
    maze = Maze(maze_width,  maze_height)
    recurse(start_x, start_y, maze)
    return maze


width = int(sys.argv[1]) if len(sys.argv) > 1 else 20
height = int(sys.argv[2]) if len(sys.argv) > 2 else 20
maze = recursive_backtracking(0, 0, width, height)
maze.display()