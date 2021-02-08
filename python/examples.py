import maze.generator.RecursiveBacktracker
import maze.generator.BinaryTree

height = 10
width = 15

# generate a random maze using BinaryTree algorithm
bt_maze = maze.generator.BinaryTree.generate(height, width)
print(f"binary tree {height}x{width}")
print(f"{bt_maze}\n\n")


# generate a random maze using RecursiveBacktracking
rb_maze = maze.generator.RecursiveBacktracker.generate(height, width)
print(f"recursive backtracker {height}x{width}")
print(f"{rb_maze}\n\n")





# def parse_input(argv):
#     """parses height and width from STDIN"""
#     height, width = 10, 15
#     if len(argv) > 1:
#         height = int(argv[1])
#     if len(argv) > 2:
#         width = int(argv[2])
#     return height, width