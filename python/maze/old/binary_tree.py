from maze import (Maze, Direction)
import random
import sys

# Binary Tree algorithm is one of the simplest maze generation algorithms:
#
# 1. start at a corner of the maze (in this case it will be the North West)
# 2. iterate through the cells row by row
# 3.     for each cell pick a random East or South wall to remove
# 4.     repeat until all cells have been visited


def generate(width, height) -> Maze:

    def east_most_cell(x):
        return x == width - 1

    def south_most_cell(y):
        return y == height - 1

    maze = Maze(width, height)

    es_directions = [Direction.E, Direction.S]

    for y in range(height):
        for x in range(width):
            if east_most_cell(x) and south_most_cell(y):
                continue
            elif east_most_cell(x):
                maze.field[y][x] |= Direction.S.value
            elif south_most_cell(y):
                maze.field[y][x] |= Direction.E.value
            else:
                random_dir = random.choice(es_directions)
                maze.field[y][x] |= random_dir.value

    return maze


width = int(sys.argv[1]) if len(sys.argv) > 1 else 20
height = int(sys.argv[2]) if len(sys.argv) > 2 else 20
maze = generate(width, height)
maze.display()
