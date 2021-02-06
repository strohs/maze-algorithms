import random
import sys

from Grid import Grid


def generate(height: int, width: int) -> Grid:
    """
    generates a random maze using binary tree algorithm.
    Binary Tree algorithm is one of the simplest maze generation algorithms:
    1. start at a corner of the maze (in this case it will be the North West)
    2. iterate through the cells row by row
    3. for each cell pick a random East or South wall to remove
    4. repeat until all cells have been visited

    :param height: number of rows in the generated maze
    :param width: number of columns in the generated maze
    :return: a Grid containing the random maze
    """
    grid = Grid(height, width)

    for cell in grid.cell_iterator():
        neighbors = []

        # if a cell has a neighbor to the south, add it to neighbors
        if cell.south:
            neighbors.append(cell.south)

        # if a cell has a neighbor to the east, add it to neighbors
        if cell.east:
            neighbors.append(cell.east)

        # choose a random neighbor from neighbors and create a link to it
        if neighbors:
            cell.link(random.choice(neighbors))

    return grid


# example of generating a maze using BinaryTree algorithm
if __name__ == "__main__":
    def parse_input(argv):
        """parses height and width from STDIN"""
        height, width = 10, 15
        if len(argv) > 1:
            height = int(argv[1])
        if len(argv) > 2:
            width = int(argv[2])
        return height, width

    (height, width) = parse_input(sys.argv)
    maze = generate(height, width)
    print(f"Binary Tree {height}x{width}")
    print(maze)
